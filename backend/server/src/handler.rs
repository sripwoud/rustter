pub mod bookmark;
pub mod boost;
pub mod post;
pub mod reaction;
pub mod user;

use crate::error::{ApiError, ApiResult};
use crate::extractor::DbConnection::DbConnection;
use crate::extractor::UserSession::UserSession;
use crate::AppState;
use axum::body::{Bytes, Full};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{async_trait, Json};
use hyper::{header, Response};
use rustter_domain::ids::ImageId;
use serde::Deserialize;
use std::path::PathBuf;
use uuid::Uuid;

#[async_trait]
pub trait PublicApiRequest {
    type Response: IntoResponse;

    async fn process_request(
        self,
        conn: DbConnection,
        state: AppState,
    ) -> ApiResult<Self::Response>;
}

#[async_trait]
pub trait AuthorizedApiRequest {
    type Response: IntoResponse;

    async fn process_request(
        self,
        conn: DbConnection,
        session: UserSession,
        state: AppState,
    ) -> ApiResult<Self::Response>;
}

pub async fn with_json_public_handler<'a, Req>(
    conn: DbConnection,
    State(state): State<AppState>,
    Json(payload): Json<Req>,
) -> ApiResult<Req::Response>
where
    Req: PublicApiRequest + Deserialize<'a>,
{
    payload.process_request(conn, state).await
}

pub async fn with_json_handler<'a, Req>(
    conn: DbConnection,
    session: UserSession,
    State(state): State<AppState>,
    Json(payload): Json<Req>,
) -> ApiResult<Req::Response>
where
    Req: AuthorizedApiRequest + Deserialize<'a>,
{
    payload.process_request(conn, session, state).await
}

const USER_CONTENT_DIR: &str = "user_content";

pub async fn save_image<T: AsRef<[u8]>>(id: ImageId, data: T) -> Result<(), ApiError> {
    use tokio::fs;

    let mut path = PathBuf::from(USER_CONTENT_DIR);
    fs::create_dir_all(&path).await?;
    path.push(id.to_string());
    fs::write(&path, data).await?;

    Ok(())
}

pub async fn load_image(Path(img_id): Path<Uuid>) -> Result<Response<Full<Bytes>>, ApiError> {
    use tokio::fs;

    let mut path = PathBuf::from(USER_CONTENT_DIR);
    path.push(img_id.to_string());

    let raw = fs::read_to_string(&path).await?;
    // data:text/plain;base64,SVGasdasdas
    let (header, image_data) = raw.split_once(',').unwrap();

    let mime = header
        .split_once("data:")
        .unwrap()
        .1 // text/plain;base64
        .split_once(';')
        .unwrap()
        .0; // text/plain

    {
        use base64::{engine::general_purpose, Engine as _};
        let image_data = general_purpose::STANDARD.decode(image_data).unwrap();

        Ok(Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, mime)
            .body(image_data.into())
            .unwrap())
    }
}
