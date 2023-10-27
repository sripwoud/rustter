use crate::{
    error::ApiResult,
    extractor::{DbConnection::DbConnection, UserSession::UserSession},
    handler::AuthorizedApiRequest,
    AppState,
};
use axum::{async_trait, http::StatusCode, Json};
use rustter_endpoint::post::types::LikeStatus;
use rustter_endpoint::{Reaction, ReactionOk};
use rustter_query::reaction as reaction_query;
use rustter_query::reaction::AggregatePostInfo;
use tracing::info;

#[async_trait]
impl AuthorizedApiRequest for Reaction {
    type Response = (StatusCode, Json<ReactionOk>);

    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        session: UserSession,
        _state: AppState,
    ) -> ApiResult<Self::Response> {
        info!(target:"rustter_server", "edit bookmarking post {post_id} for user {user_id}", post_id=self.post_id, user_id=session.user_id);
        reaction_query::react(
            &mut conn,
            reaction_query::Reaction {
                user_id: session.user_id,
                post_id: self.post_id,
                like_status: match self.like_status {
                    LikeStatus::Like => 1,
                    LikeStatus::Dislike => -1,
                    LikeStatus::NoReaction => 0,
                },
                reaction: None,
                created_at: chrono::Utc::now(),
            },
        )?;

        let AggregatePostInfo {
            likes, dislikes, ..
        } = reaction_query::aggregate(&mut conn, self.post_id)?;

        Ok((
            StatusCode::OK,
            Json(ReactionOk {
                post_id: Default::default(),
                likes,
                dislikes,
                like_status: self.like_status,
            }),
        ))
    }
}
