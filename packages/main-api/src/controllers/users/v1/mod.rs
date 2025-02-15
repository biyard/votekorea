use by_axum::{
    aide,
    auth::{generate_jwt, Authorization},
    axum::{
        extract::{Query, State},
        http::HeaderMap,
        response::{IntoResponse, Response},
        routing::get,
        Extension, Json,
    },
};
use by_types::Claims;
use dto::*;
use reqwest::header::{self, HeaderValue};
use sqlx::{Pool, Postgres};
use tracing::instrument;
use validator::Validate;

#[derive(
    Debug,
    Clone,
    serde::Deserialize,
    serde::Serialize,
    Default,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct UserWrapper {
    #[serde(flatten)]
    pub user: User,
    pub token: String,
}

impl IntoResponse for UserWrapper {
    fn into_response(self) -> Response {
        let mut headers = HeaderMap::new();
        let cookie = format!("type={}; token={};", "bearer", self.token);
        headers.insert(header::SET_COOKIE, HeaderValue::from_str(&cookie).unwrap());
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.token)).unwrap(),
        );

        (
            by_axum::axum::http::StatusCode::OK,
            headers,
            by_axum::axum::Json(self),
        )
            .into_response()
    }
}

#[derive(Clone, Debug)]
pub struct UserControllerV1 {
    users: UserRepository,
}

impl UserControllerV1 {
    pub fn route(pool: Pool<Postgres>) -> Result<by_axum::axum::Router> {
        let users = User::get_repository(pool);

        let ctrl = UserControllerV1 { users };

        Ok(by_axum::axum::Router::new()
            .route("/", get(Self::read_user).post(Self::act_user))
            .with_state(ctrl.clone()))
    }

    #[instrument]
    pub async fn act_user(
        headers: HeaderMap,
        State(ctrl): State<UserControllerV1>,
        Extension(sig): Extension<Option<Authorization>>,
        Json(body): Json<UserAction>,
    ) -> Result<UserWrapper> {
        tracing::debug!("act_user: {:?}", body);
        body.validate()?;

        match body {
            UserAction::Signup(req) => ctrl.signup(headers, req).await,
            UserAction::Login(req) => ctrl.login(headers, req).await,
        }
    }

    #[instrument]
    pub async fn read_user(
        headers: HeaderMap,
        State(ctrl): State<UserControllerV1>,
        Extension(sig): Extension<Option<Authorization>>,

        Query(req): Query<UserReadAction>,
    ) -> Result<Json<User>> {
        tracing::debug!("read_user: sig={:?}", sig);
        req.validate()?;

        match req.action {
            Some(UserReadActionType::CheckEmail) => ctrl.check_email(req).await,
            Some(UserReadActionType::UserInfo) => ctrl.user_info(req).await,
            _ => Err(ServiceError::NoReadActionType)?,
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct OpenIdResponse {
    pub sub: String,
}

impl UserControllerV1 {
    pub async fn get_openid(&self, headers: &HeaderMap) -> Result<String> {
        let id_token = headers
            .get("x-id-token")
            .ok_or(ServiceError::NoIdToken(
                "you must set kakao OAuth ID token to `X-ID-TOKEN` header".to_string(),
            ))?
            .to_str()
            .map_err(|s| {
                tracing::error!("failed to get id_token: {:?}", s);
                ServiceError::NoIdToken(
                    "you must set kakao OAuth ID token to `X-ID-TOKEN` header".to_string(),
                )
            })?;

        let mut params = std::collections::HashMap::new();
        params.insert("id_token", id_token);

        let client = reqwest::Client::new();
        let res = client
            .post("https://kauth.kakao.com/oauth/tokeninfo")
            .form(&params)
            .send()
            .await?
            .json::<OpenIdResponse>()
            .await?;

        Ok(res.sub)
    }

    #[instrument]
    pub async fn login(&self, headers: HeaderMap, req: UserLoginRequest) -> Result<UserWrapper> {
        let kakao_id = self.get_openid(&headers).await?;

        if &kakao_id != &req.kakao_id {
            Err(ServiceError::Unauthorized)?;
        }

        let req = UserReadAction::new().find_by_kakao_id(kakao_id.clone());

        let user = self
            .users
            .find_one(&req)
            .await
            .map_err(|_| ServiceError::NotFound)?;

        let token = self.generate_token(&user)?;

        Ok(UserWrapper { user, token })
    }

    pub fn generate_token(&self, user: &User) -> Result<String> {
        let mut claims = Claims {
            sub: user.id.to_string(),
            role: (user.role.clone() as i32).try_into().map_err(|e| {
                tracing::error!("Failed to convert UserRole to Role: {}", e);
                ServiceError::RoleConversionException
            })?,
            ..Claims::default()
        };

        generate_jwt(&mut claims).map_err(|e| {
            tracing::error!("Failed to generate JWT: {}", e);
            ServiceError::GenerateJwtException
        })
    }

    #[instrument]
    pub async fn signup(&self, headers: HeaderMap, req: UserSignupRequest) -> Result<UserWrapper> {
        let kakao_id = self.get_openid(&headers).await?;

        let user = self
            .users
            .insert(
                req.nickname,
                req.email,
                req.profile_url,
                UserRole::User,
                kakao_id,
            )
            .await?;

        let token = self.generate_token(&user)?;

        Ok(UserWrapper { user, token })
    }

    #[instrument]
    pub async fn check_email(&self, req: UserReadAction) -> Result<Json<User>> {
        let user = self
            .users
            .find_one(&req)
            .await
            .map_err(|_| ServiceError::NotFound)?;

        Ok(Json(user))
    }

    #[instrument]
    pub async fn user_info(&self, req: UserReadAction) -> Result<Json<User>> {
        let user = self.users.find_one(&req).await?;

        Ok(Json(user))
    }
}
