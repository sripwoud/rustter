use crate::error::ApiResult;
use crate::extractor::DbConnection::DbConnection;
use crate::extractor::UserSession::UserSession;
use crate::handler::AuthorizedApiRequest;
use crate::AppState;
use axum::http::StatusCode;
use axum::{async_trait, Json};
use rustter_endpoint::post::endpoint::BookmarkOk;
use rustter_endpoint::post::types::BookmarkAction;
use rustter_endpoint::Bookmark;
use rustter_query::bookmark as bookmark_query;
use tracing::info;

#[async_trait]
impl AuthorizedApiRequest for Bookmark {
    type Response = (StatusCode, Json<BookmarkOk>);

    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        session: UserSession,
        _state: AppState,
    ) -> ApiResult<Self::Response> {
        info!(target:"rustter_server", "edit bookmarking post {post_id} for user {user_id}", post_id=self.post_id, user_id=session.user_id);
        match self.action {
            BookmarkAction::Save => {
                bookmark_query::save(&mut conn, session.user_id, self.post_id)?;
            }
            BookmarkAction::Remove => {
                bookmark_query::remove(&mut conn, session.user_id, self.post_id)?;
            }
        }

        Ok((
            StatusCode::OK,
            Json(BookmarkOk {
                status: self.action,
            }),
        ))
    }
}
