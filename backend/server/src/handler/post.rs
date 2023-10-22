use crate::error::{ApiError, ApiResult};
use crate::extractor::DbConnection::DbConnection;
use crate::extractor::UserSession::UserSession;
use crate::handler::AuthorizedApiRequest;
use crate::AppState;
use axum::http::StatusCode;
use axum::{async_trait, Json};
use rustter_domain::Username;
use rustter_endpoint::post::endpoint::{NewPost, NewPostOk, TrendingPostsOk};
use rustter_endpoint::post::types::{LikeStatus, PublicPost};
use rustter_endpoint::{RequestFailed, TrendingPosts};
use rustter_query::post as post_query;
use rustter_query::{user, AsyncConnection};
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
        let post = post_query::Post::new(session.user_id, self.content, self.options)?;
        let post_id = post_query::new(&mut conn, post)?;
        info!(target:"rustter_server", "created post {post_id}");
        Ok((StatusCode::CREATED, Json(NewPostOk { post_id })))
    }
}

fn to_public(
    post: post_query::Post,
    conn: &mut AsyncConnection,
    session: Option<&UserSession>,
) -> ApiResult<PublicPost> {
    let user = user::get(conn, post.user_id).unwrap();
    let author = super::user::to_public(user, session)?;
    let reply_to = match post.reply_to {
        Some(reply_to) => {
            let replied_post = post_query::get(conn, reply_to)?;
            let replied_user = user::get(conn, replied_post.user_id)?;
            Some((
                Username::new(replied_user.handle)?,
                replied_user.id,
                replied_post.id,
            ))
        }
        None => None,
    };

    if let Ok(content) = serde_json::from_value(post.content.0) {
        Ok(PublicPost {
            id: post.id,
            author,
            content,
            time_posted: post.time_posted,
            reply_to,
            like_status: LikeStatus::NoReaction,
            bookmarked: false,
            boosted: false,
            boosts: 0,
            likes: 0,
            dislikes: 0,
        })
    } else {
        Err(ApiError {
            code: Some(StatusCode::INTERNAL_SERVER_ERROR),
            err: color_eyre::Report::new(RequestFailed {
                msg: "failed to deserialize post content".to_string(),
            }),
        })
    }
}

fn _trending_posts(
    DbConnection(mut conn): DbConnection,
    session: Option<&UserSession>,
    limit: Option<i64>,
) -> ApiResult<Vec<PublicPost>> {
    let mut posts = vec![];

    for post in post_query::trending_posts(&mut conn, limit)? {
        let post_id = post.id;
        match to_public(post, &mut conn, session) {
            Ok(post_id) => {
                posts.push(post_id);
            }
            Err(e) => {
                tracing::error!(target:"rustter_server",err=%e.err, post_id=?post_id, "post contains invalid data")
            }
        }
    }

    Ok(posts)
}

#[async_trait]
impl AuthorizedApiRequest for TrendingPosts {
    type Response = (StatusCode, Json<TrendingPostsOk>);

    async fn process_request(
        self,
        conn: DbConnection,
        session: UserSession,
        _state: AppState,
    ) -> ApiResult<Self::Response> {
        info!(target:"rustter_server", "fetching trending posts");
        let posts = _trending_posts(conn, Some(&session), None)?;

        Ok((StatusCode::OK, Json(TrendingPostsOk(posts))))
    }
}

pub async fn trending_posts(
    conn: DbConnection,
    session: UserSession,
) -> ApiResult<(StatusCode, Json<TrendingPostsOk>)> {
    info!(target:"rustter_server", "fetching trending posts");
    let posts = _trending_posts(conn, Some(&session), None)?;

    Ok((StatusCode::OK, Json(TrendingPostsOk(posts))))
}
