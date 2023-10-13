use crate::AppState;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::{header, StatusCode};
use axum::{async_trait, Extension, Json, RequestPartsExt};
use rustter_cookie::{get_from_str, CookieKey};
use rustter_domain::ids::{SessionId, UserId};
use rustter_endpoint::RequestFailed;
use rustter_query::schema::web::dsl::web;
use rustter_query::schema::web::table;
use rustter_query::user::User;
use rustter_query::OwnedAsyncConnection;
use std::str::FromStr;
use tracing::info;

// similar to DI for handlers
pub struct DbConnection(pub OwnedAsyncConnection);
//                                \___ what we want to extract

#[async_trait]
impl<S> FromRequestParts<S> for DbConnection
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let Extension(state) = parts.extract::<Extension<AppState>>().await.unwrap(); // can safely unwrap as we added state to the layer
        let connection = state.db_pool.get_owned().await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to connect to database",
            )
        })?;
        Ok(Self(connection))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UserSession {
    pub user_id: UserId,
    pub session_id: SessionId,
}

#[async_trait]
impl<S> FromRequestParts<S> for UserSession {
    type Rejection = (StatusCode, Json<RequestFailed>);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let unauthorized = || {
            (
                StatusCode::UNAUTHORIZED,
                Json(RequestFailed {
                    msg: "unauthorized".into(),
                }),
            )
        };

        let DbConnection(mut conn) = parts.extract::<DbConnection>().await.unwrap();
        let Extension(state) = parts.extract::<Extension<AppState>>().await.unwrap(); // can safely unwrap as we added state to the layer
        let cookies = parts
            .headers
            .get(header::COOKIE)
            .and_then(|cookie| cookie.to_str().ok())
            .ok_or_else(unauthorized)?;

        let session_id = get_from_str(cookies, CookieKey::Id.as_ref())
            .and_then(|id| SessionId::from_str(id).ok())
            .ok_or_else(unauthorized)?;

        let session_signature = get_from_str(cookies, CookieKey::Signature.as_ref())
            .and_then(|signature| rustter_crypto::decode_base64(signature).ok())
            .and_then(|signature| rustter_crypto::sign::signature_from_bytes(&signature).ok())
            .ok_or_else(unauthorized)?;

        state
            .signing_keys
            .verify(session_id.as_uuid().as_bytes(), session_signature)
            .map_err(|_| unauthorized())?;

        let user_id = rustter_query::session::get(&mut conn, session_id)
            .ok()
            .flatten()
            .ok_or_else(unauthorized)?
            .user_id;

        info!(
            user_id = user_id.into_inner().to_string(),
            "user authenticated"
        );

        Ok(Self {
            user_id,
            session_id,
        })
    }
}
