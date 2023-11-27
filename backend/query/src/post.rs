use crate::{schema, DieselError};
use chrono::{DateTime, Utc};
use diesel::insert_into;
use diesel::prelude::*;
use rustter_domain::ids::{PostId, UserId};
use rustter_endpoint::post;
use serde::{Deserialize, Serialize};

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
        use rustter_endpoint::post::types::Content as EndpointContent;

        insert_into(posts).values(&post).execute(conn)?;

        if let Ok(EndpointContent::Poll(poll)) =
            serde_json::from_value::<EndpointContent>(post.content.0)
        {
            {
                use crate::schema::poll_choices::dsl::*;
                for _choice in poll.choices {
                    insert_into(poll_choices)
                        .values((
                            id.eq(_choice.id),
                            choice.eq(_choice.description.as_ref()),
                            post_id.eq(post.id),
                        ))
                        .execute(conn)?;
                }
            }
        }

        posts.select(id).order(created_at.desc()).first(conn)
    })
}

pub fn trending_posts(
    conn: &mut PgConnection,
    limit: Option<i64>,
) -> Result<Vec<Post>, DieselError> {
    use crate::schema::posts::dsl::*;
    posts
        .filter(reply_to.is_null())
        .order(created_at.desc())
        .limit(limit.unwrap_or(20))
        .get_results(conn)
}

pub fn get(conn: &mut PgConnection, post_id: PostId) -> Result<Post, DieselError> {
    use crate::schema::posts::dsl::*;
    posts.find(post_id).first(conn)
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
