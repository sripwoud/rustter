use crate::{schema, DieselError};
use chrono::{DateTime, Utc};
use diesel::insert_into;
use diesel::prelude::*;
use rustter_domain::ids::{PostId, UserId};
use rustter_endpoint::post;
use serde::{Deserialize, Serialize};
use rustter_endpoint::post::types::PublicPost;

#[derive(Clone, Debug, DieselNewType, Deserialize, Serialize)]
pub struct Content(pub serde_json::Value);

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = schema::posts)]
pub struct Post {
    pub id: PostId,
    pub user_id: UserId,
    pub content: Content,
    pub time_posted: DateTime<Utc>,
    pub direct_message_to: Option<UserId>,
    pub reply_to: Option<PostId>,
    pub created_at: DateTime<Utc>,
}

pub fn new(conn: &mut PgConnection, post: Post) -> Result<PostId, DieselError> {
    use crate::schema::posts::dsl::*;

    conn.transaction::<PostId, DieselError, _>(|conn| {
        insert_into(posts).values(&post).execute(conn)?;

        posts.select(id).order(created_at.desc()).first(conn)
    })
}

pub fn get_trending_posts(
    conn: &mut PgConnection,
    limit: Option<i64>,
) -> Result<Vec<PublicPost>, DieselError> {
    use crate::schema::posts::dsl::*;

  posts
        .order(created_at.desc())
        .limit(limit.unwrap_or(20))
        .load::<Post>(conn).unwrap().iter().map(|post| {
            let user = crate::user::get(conn, post.user_id).unwrap();
            PublicPost {
                id: post.id,
                author: user.handle,
                user: user.into(),
                content: post.content.0.clone(),
                time_posted: post.time_posted,
                direct_message_to: post.direct_message_to,
                reply_to: post.reply_to,
            }
        }).collect(
}

impl Post {
    pub fn new(
        author: UserId,
        content: post::types::Content,
        post::types::NewPostOptions {
            reply_to,
            direct_message_to,
            time_posted,
        }: post::types::NewPostOptions,
    ) -> Result<Self, serde_json::Error> {
        Ok(Self {
            id: PostId::new(),
            user_id: author,
            content: Content(serde_json::to_value(content)?),
            time_posted,
            direct_message_to,
            reply_to,
            created_at: Utc::now(),
        })
    }
}
