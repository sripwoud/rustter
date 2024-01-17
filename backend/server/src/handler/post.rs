use crate::{
    error::{ApiError, ApiResult},
    extractor::{DbConnection::DbConnection, UserSession::UserSession},
    handler::{save_image, AuthorizedApiRequest},
    AppState,
};
use axum::{async_trait, http::StatusCode, Json};
use rustter_domain::{ids::ImageId, Username};
use rustter_endpoint::{
    app_url,
    app_url::user_content,
    post::{
        endpoint::{NewPost, NewPostOk, TrendingPostsOk},
        types::{Content, ImageKind, LikeStatus, PublicPost},
    },
    RequestFailed, TrendingPosts,
};
use rustter_query::{
    bookmark as bookmark_query, post as post_query, reaction as reaction_query,
    reaction::AggregatePostInfo, user, vote as vote_query, AsyncConnection,
};

use rustter_domain::ids::UserId;
use rustter_endpoint::post::endpoint::{BookmarkedPostsOk, HomePostsOk, LikedPostsOk};
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
        let mut content = self.content;

        if let Content::Image(ref mut img) = content {
            if let ImageKind::DataUrl(data) = &img.kind {
                let id = ImageId::new();
                save_image(id, &data).await?;
                img.kind = ImageKind::Id(id);
            }
        }

        let post = post_query::Post::new(session.user_id, content, self.options)?;
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
    let author = super::user::to_public(conn, session, user)?;
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

    if let Ok(mut content) = serde_json::from_value(post.content.0) {
        match content {
            Content::Image(ref mut image) => {
                if let ImageKind::Id(id) = image.kind {
                    let url = app_url::domain_and(user_content::ROOT)
                        .join(user_content::IMAGES)
                        .unwrap()
                        .join(&id.to_string())
                        .unwrap();
                    image.kind = ImageKind::Url(url);
                }
            }
            Content::Poll(ref mut poll) => {
                for (id, result) in vote_query::get_poll_results(conn, post.id)?.results {
                    for choice in poll.choices.iter_mut() {
                        if choice.id == id {
                            choice.num_votes = result;
                            break;
                        }
                    }
                }

                if let Some(session) = session {
                    poll.voted = vote_query::did_vote(conn, session.user_id, post.id)?;
                }
            }
            _ => (),
        }

        let AggregatePostInfo {
            boosts,
            likes,
            dislikes,
            ..
        } = reaction_query::aggregate(conn, post.id)?;

        Ok(PublicPost {
            id: post.id,
            author,
            content,
            time_posted: post.time_posted,
            reply_to,
            like_status: {
                match session {
                    Some(session) => match reaction_query::get(conn, post.id, session.user_id)? {
                        Some(reaction) => match reaction.like_status {
                            1 => LikeStatus::Like,
                            -1 => LikeStatus::Dislike,
                            _ => LikeStatus::NoReaction,
                        },
                        None => LikeStatus::NoReaction,
                    },
                    None => LikeStatus::NoReaction,
                }
            },
            bookmarked: match session {
                Some(session) => bookmark_query::get(conn, session.user_id, post.id)?,
                None => false,
            },
            boosted: !matches!(boosts, 0),
            boosts,
            likes,
            dislikes,
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

pub fn public_posts(
    DbConnection(mut conn): DbConnection,
    session: Option<&UserSession>,
    for_user: UserId,
) -> ApiResult<Vec<PublicPost>> {
    let mut posts = vec![];

    for post in post_query::public_posts(&mut conn, for_user, None)? {
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

pub async fn home_posts(
    DbConnection(mut conn): DbConnection,
    session: UserSession,
) -> ApiResult<(StatusCode, Json<HomePostsOk>)> {
    info!(target:"rustter_server", "fetching home posts");
    let mut posts = vec![];

    for post in post_query::get_home_posts(&mut conn, session.user_id)? {
        let post_id = post.id;
        match to_public(post, &mut conn, Some(&session)) {
            Ok(post_id) => {
                posts.push(post_id);
            }
            Err(e) => {
                tracing::error!(target:"rustter_server",err=%e.err, post_id=?post_id, "post contains invalid data")
            }
        }
    }

    Ok((StatusCode::OK, Json(HomePostsOk(posts))))
}

pub async fn bookmarked_posts(
    DbConnection(mut conn): DbConnection,
    session: UserSession,
) -> ApiResult<(StatusCode, Json<BookmarkedPostsOk>)> {
    info!(target:"rustter_server", "fetching bookmarked posts");
    let mut posts = vec![];

    for post in post_query::get_bookmarked_posts(&mut conn, session.user_id)? {
        let post_id = post.id;
        match to_public(post, &mut conn, Some(&session)) {
            Ok(post_id) => {
                posts.push(post_id);
            }
            Err(e) => {
                tracing::error!(target:"rustter_server",err=%e.err, post_id=?post_id, "post contains invalid data")
            }
        }
    }

    Ok((StatusCode::OK, Json(BookmarkedPostsOk(posts))))
}

pub async fn liked_posts(
    DbConnection(mut conn): DbConnection,
    session: UserSession,
) -> ApiResult<(StatusCode, Json<LikedPostsOk>)> {
    info!(target:"rustter_server", "fetching liked posts");
    let mut posts = vec![];

    for post in post_query::get_liked_posts(&mut conn, session.user_id)? {
        let post_id = post.id;
        match to_public(post, &mut conn, Some(&session)) {
            Ok(post_id) => {
                posts.push(post_id);
            }
            Err(e) => {
                tracing::error!(target:"rustter_server",err=%e.err, post_id=?post_id, "post contains invalid data")
            }
        }
    }

    Ok((StatusCode::OK, Json(LikedPostsOk(posts))))
}
