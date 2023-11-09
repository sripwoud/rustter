use crate::error::ApiResult;
use crate::extractor::DbConnection::DbConnection;
use crate::extractor::UserSession::UserSession;
use crate::handler::AuthorizedApiRequest;
use crate::AppState;
use axum::http::StatusCode;
use axum::{async_trait, Json};
use chrono::Utc;
use rustter_endpoint::post::types::BoostAction;
use rustter_endpoint::{Boost, BoostOk};
use rustter_query::boost as boost_query;
use tracing::info;

#[async_trait]
impl AuthorizedApiRequest for Boost {
    type Response = (StatusCode, Json<BoostOk>);

    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        session: UserSession,
        _state: AppState,
    ) -> ApiResult<Self::Response> {
        info!(target:"rustter_server", "edit boosting post {post_id} for user {user_id}", post_id=self.post_id, user_id=session.user_id);
        match self.action {
            BoostAction::Add => {
                boost_query::boost(&mut conn, session.user_id, self.post_id, Utc::now())?;
            }
            BoostAction::Remove => {
                boost_query::remove(&mut conn, session.user_id, self.post_id)?;
            }
        }

        Ok((
            StatusCode::OK,
            Json(BoostOk {
                status: self.action,
            }),
        ))
    }
}
