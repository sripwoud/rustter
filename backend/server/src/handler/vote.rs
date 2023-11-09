use crate::error::ApiResult;
use crate::extractor::DbConnection::DbConnection;
use crate::extractor::UserSession::UserSession;
use crate::handler::AuthorizedApiRequest;
use crate::AppState;
use axum::http::StatusCode;
use axum::{async_trait, Json};
use rustter_endpoint::{Vote, VoteOk};
use rustter_query::vote as vote_query;
use tracing::info;

#[async_trait]
impl AuthorizedApiRequest for Vote {
    type Response = (StatusCode, Json<VoteOk>);

    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        session: UserSession,
        _state: AppState,
    ) -> ApiResult<Self::Response> {
        info!(target:"rustter_server", "vote {choice_id} for post {post_id} for user {user_id}", choice_id=self.choice_id, post_id=self.post_id, user_id=session.user_id);

        let cast = vote_query::vote(&mut conn, session.user_id, self.post_id, self.choice_id)?;

        Ok((StatusCode::OK, Json(VoteOk { cast })))
    }
}
