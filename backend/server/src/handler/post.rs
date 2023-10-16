use crate::error::ApiResult;
use crate::extractor::DbConnection::DbConnection;
use crate::extractor::UserSession::UserSession;
use crate::handler::{AuthorizedApiRequest, PublicApiRequest};
use crate::AppState;
use axum::http::StatusCode;
use axum::{async_trait, Json};
use rustter_endpoint::post::endpoint::{NewPost, NewPostOk, TrendingPostsOk};
use rustter_endpoint::TrendingPosts;
use rustter_query::post::{get_trending_posts, Post};
use tracing::info;

#[async_trait]
impl AuthorizedApiRequest for NewPost {
    type Response = (StatusCode, Json<NewPostOk>);

    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        session: UserSession,
        _state: AppState,
    ) -> ApiResult<Self::Response> {
        let post = Post::new(session.user_id, self.content, self.options)?;
        let post_id = rustter_query::post::new(&mut conn, post)?;
        info!(target:"rustter_server", "created post {post_id}");
        Ok((StatusCode::CREATED, Json(NewPostOk { post_id })))
    }
}

#[async_trait]
impl PublicApiRequest for TrendingPosts {
    type Response = (StatusCode, Json<TrendingPostsOk>);

    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        _state: AppState,
    ) -> ApiResult<Self::Response> {
        info!(target:"rustter_server", "fetching trending posts");
        Ok((
            StatusCode::OK,
            Json(TrendingPostsOk(get_trending_posts(&mut conn, None)?)),
        ))
    }
}
