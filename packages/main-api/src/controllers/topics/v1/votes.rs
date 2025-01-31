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

#[derive(Clone, Debug)]
pub struct VoteControllerV1 {
    repo: VoteRepository,
}

impl VoteControllerV1 {
    pub async fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = Vote::get_repository(pool);

        repo.create_table().await?;

        let ctrl = VoteControllerV1 { repo };

        Ok(by_axum::axum::Router::new()
            .route("/:id", get(Self::get_vote).post(Self::act_vote_by_id))
            .with_state(ctrl.clone())
            .route("/", post(Self::act_vote).get(Self::list_vote))
            .with_state(ctrl.clone()))
    }

    pub async fn act_vote(
        State(ctrl): State<VoteControllerV1>,
        Path(topic_id): Path<String>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<VoteAction>,
    ) -> Result<Json<Vote>> {
        tracing::debug!("act_vote {:?}", body);

        let user_id = match auth {
            Some(Authorization::Bearer { claims }) if claims.role != Role::Guest => claims.sub,
            _ => return Err(ServiceError::Unauthorized),
        };

        match body {
            VoteAction::Support(VoteSupportRequest { amount, name }) => {
                let vote = ctrl
                    .repo
                    .insert(topic_id, user_id, amount, name, None)
                    .await?;
                Ok(Json(vote))
            }
        }
    }

    pub async fn act_vote_by_id(
        State(_ctrl): State<VoteControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path((_topic_id, id)): Path<(String, String)>,
        Json(body): Json<VoteByIdAction>,
    ) -> Result<Json<Vote>> {
        tracing::debug!("act_vote_by_id {:?} {:?}", id, body);
        Ok(Json(Vote::default()))
    }

    pub async fn get_vote(
        State(_ctrl): State<VoteControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path((_topic_id, id)): Path<(String, String)>,
    ) -> Result<Json<Vote>> {
        tracing::debug!("get_vote {:?}", id);
        Ok(Json(Vote::default()))
    }

    pub async fn list_vote(
        State(_ctrl): State<VoteControllerV1>,
        Path(_topic_id): Path<String>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(q): Query<VoteQuery>,
    ) -> Result<Json<VoteGetResponse>> {
        tracing::debug!("list_vote {:?}", q);

        Ok(Json(VoteGetResponse::Query(QueryResponse::default())))
    }
}
