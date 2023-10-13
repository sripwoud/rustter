use crate::error::ApiResult;
use crate::extractor::DbConnection::DbConnection;
use crate::extractor::UserSession::UserSession;
use crate::handler::AuthorizedApiRequest;
use crate::AppState;
use axum::http::StatusCode;
use axum::{async_trait, Json};
use rustter_endpoint::post::endpoint::{NewPost, NewPostOk};

#[async_trait]
impl AuthorizedApiRequest for NewPost {
    type Response = (StatusCode, Json<NewPostOk>);

    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        UserSession {
            user_id: mut _conn,
            session_id: _session_id,
        }: UserSession,
        state: AppState,
    ) -> ApiResult<Self::Response> {
        todo!();
        // let post_id = rustter_query::post::new(&mut _conn, user_id, self.r#type, self.options)?;

        // Ok((
        //     StatusCode::CREATED,
        //     Json(NewPostOk {
        //         post_id,
        //     }),
        // ))
    }
}
