pub mod user;

use crate::error::ApiResult;
use crate::extractor::DbConnection;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{async_trait, Json};
use serde::Deserialize;

#[async_trait]
pub trait PublicApiRequest {
    type Response: IntoResponse;

    async fn process_request(
        self,
        conn: DbConnection,
        state: AppState,
    ) -> ApiResult<Self::Response>;
}

pub async fn with_public_handler<'a, Req>(
    conn: DbConnection,
    State(state): State<AppState>,
    Json(payload): Json<Req>,
) -> ApiResult<Req::Response>
where
    Req: PublicApiRequest + Deserialize<'a>,
{
    payload.process_request(conn, state).await
}
