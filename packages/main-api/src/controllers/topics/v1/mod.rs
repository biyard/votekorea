pub mod votes;

use std::time::SystemTime;

use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Extension, Json,
    },
};
use by_types::Role;
use dto::*;
use tracing::instrument;
use votes::VoteControllerV1;

#[derive(Clone, Debug)]
pub struct TopicControllerV1 {
    repo: TopicRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl TopicControllerV1 {
    pub async fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = Topic::get_repository(pool.clone());

        repo.create_table().await?;

        let ctrl = TopicControllerV1 {
            repo,
            pool: pool.clone(),
        };

        Ok(by_axum::axum::Router::new()
            .route("/:id", get(Self::get_topic).post(Self::act_topic_by_id))
            .with_state(ctrl.clone())
            .route("/", post(Self::act_topic).get(Self::list_topic))
            .with_state(ctrl.clone())
            .nest("/:id/votes", VoteControllerV1::route(pool.clone()).await?))
    }

    pub async fn act_topic(
        State(ctrl): State<TopicControllerV1>,
        Extension(authz): Extension<Option<Authorization>>,
        Json(body): Json<TopicAction>,
    ) -> Result<Json<Topic>> {
        tracing::debug!("list_topic {:?}", body);
        match authz {
            Some(Authorization::Bearer { claims }) if claims.role == Role::Admin => {}
            _ => return Err(ServiceError::Unauthorized),
        }

        match body {
            TopicAction::Create(TopicCreateRequest {
                title,
                content,
                started_at,
                ended_at,
                requirement,
            }) => {
                let topic = ctrl
                    .repo
                    .insert(title, content, started_at, ended_at, requirement)
                    .await?;

                Ok(Json(topic))
            }
        }
    }

    pub async fn act_topic_by_id(
        State(_ctrl): State<TopicControllerV1>,
        Extension(_sig): Extension<Option<Authorization>>,
        Path(id): Path<String>,
        Json(body): Json<TopicByIdAction>,
    ) -> Result<Json<Topic>> {
        tracing::debug!("list_topic {:?} {:?}", id, body);
        Ok(Json(Topic::default()))
    }

    pub async fn get_topic(
        State(_ctrl): State<TopicControllerV1>,
        Extension(_sig): Extension<Option<Authorization>>,
        Path(id): Path<String>,
    ) -> Result<Json<Topic>> {
        tracing::debug!("get_topic {:?}", id);
        Ok(Json(Topic::default()))
    }

    pub async fn list_topic(
        State(ctrl): State<TopicControllerV1>,
        Extension(authz): Extension<Option<Authorization>>,
        Query(param): Query<TopicParam>,
    ) -> Result<Json<TopicGetResponse>> {
        tracing::debug!("list_topic {:?}", param);

        match param {
            TopicParam::Query(q) => {
                let topics = ctrl.repo.find(&q).await?;
                Ok(Json(TopicGetResponse::Query(topics)))
            }
            TopicParam::Read(action) => ctrl.read_action(action, authz).await,
        }
    }
}

impl TopicControllerV1 {
    #[instrument]
    async fn read_action(
        &self,
        action: TopicReadAction,
        authz: Option<Authorization>,
    ) -> Result<Json<TopicGetResponse>> {
        tracing::debug!("read_action {:?}", action);

        self.get_topic_for_voting(authz).await
    }

    #[instrument]
    async fn get_topic_for_voting(
        &self,
        authz: Option<Authorization>,
    ) -> Result<Json<TopicGetResponse>> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let user_id = match authz {
            Some(Authorization::Bearer { claims }) => claims.sub.parse::<i64>().unwrap_or(0),
            _ => 0,
        };

        let sql = Topic::base_sql(user_id);
        tracing::debug!("get_topic_for_voting base sql {:?}", sql);
        let topic: Topic = sqlx::query(&format!(
            "{sql} WHERE started_at < $1 and ended_at > $1 ORDER BY created_at DESC"
        ))
        .bind(now)
        .map(|row: sqlx::postgres::PgRow| {
            tracing::debug!("get_topic_for_voting row {:?}", row);
            row.into()
        })
        .fetch_one(&self.pool)
        .await?;

        tracing::debug!("get_topic_for_voting {:?}", topic);

        Ok(Json(TopicGetResponse::Read(topic)))
    }
}
#[cfg(test)]
mod tests {
    use by_types::Claims;
    use sqlx::postgres::PgPoolOptions;

    use super::*;

    async fn setup() -> TopicControllerV1 {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_file(true)
            .with_line_number(true)
            .with_thread_ids(true)
            .with_target(false)
            .try_init();

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://postgres:postgres@localhost:5432/referendum")
            .await
            .unwrap();

        let repo = Topic::get_repository(pool.clone());

        repo.create_table().await.unwrap();

        TopicControllerV1 {
            repo,
            pool: pool.clone(),
        }
    }

    #[tokio::test]
    async fn test_act_topic() {
        let ctrl = setup().await;
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let started_at = now as i64 - 10;
        let ended_at = now as i64 + 10000;

        let res = TopicControllerV1::act_topic(
            State(ctrl.clone()),
            Extension(Some(Authorization::Bearer {
                claims: Claims {
                    sub: "1".to_string(),
                    exp: now + 100000,
                    role: Role::Admin,
                    custom: Default::default(),
                },
            })),
            Json(TopicAction::Create(TopicCreateRequest {
                title: "title".to_string(),
                content: "content".to_string(),
                started_at,
                ended_at,
                requirement: 10,
            })),
        )
        .await;

        assert!(res.is_ok());

        let res = res.unwrap();

        let topic = match res {
            Json(topic) => topic,
            _ => panic!("unexpected response"),
        };

        assert_eq!(topic.title, "title".to_string());
        assert_eq!(topic.content, "content".to_string());
        assert_eq!(topic.started_at, started_at);
        assert_eq!(topic.ended_at, ended_at);
        assert_eq!(topic.requirement, 10);
    }

    #[tokio::test]
    async fn test_get_topic_for_voting() {
        let ctrl = setup().await;
        let res = ctrl.get_topic_for_voting().await;
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        assert!(res.is_ok());

        let res = res.unwrap();

        let topic = match res {
            Json(TopicGetResponse::Read(topic)) => topic,
            _ => panic!("unexpected response"),
        };

        assert!(topic.started_at < now, "started_at must be less than now");
        assert!(topic.ended_at > now, "ended_at must be greater than now");
        assert!(topic.id != "0".to_string(), "id must not be empty");
    }
}
