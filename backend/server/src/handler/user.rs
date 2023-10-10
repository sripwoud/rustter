use crate::error::ApiResult;
use crate::extractor::DbConnection;
use crate::handler::PublicApiRequest;
use crate::AppState;
use axum::http::StatusCode;
use axum::{async_trait, Json};
use rustter_endpoint::{CreateUser, CreateUserOk, Login, LoginOk};
use tracing::info;

#[async_trait]
impl PublicApiRequest for CreateUser {
    type Response = (StatusCode, Json<CreateUserOk>);

    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        state: AppState,
    ) -> ApiResult<Self::Response> {
        let password_hash = rustter_crypto::hash_password(self.password)?;
        let user_id = rustter_query::user::new(&mut conn, password_hash, &self.username)?;

        info!(username = self.username.as_ref(), "new user created");

        Ok((
            StatusCode::CREATED,
            Json(CreateUserOk {
                user_id,
                username: self.username,
            }),
        ))
    }
}

#[async_trait]
impl PublicApiRequest for Login {
    type Response = (StatusCode, Json<LoginOk>);

    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        state: AppState,
    ) -> ApiResult<Self::Response> {
        // info!(username = self.username.as_ref(), "user logged in");

        Ok((
            StatusCode::FOUND,
            Json(LoginOk {
                session_signature: "".to_string(),
                session_id: Default::default(),
                session_expires: Default::default(),
                display_name: None,
                email: None,
                profile_image: None,
                user_id: Default::default(),
            }),
        ))
    }
}
