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

pub fn public_posts(
    conn: &mut PgConnection,
    for_user_id:UserId,
    limit: Option<i64>,
) -> Result<Vec<Post>, DieselError> {
    use crate::schema::posts::dsl::*;
    posts
        .filter(user_id.eq(for_user_id.as_uuid()))
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

pub fn get_home_posts(conn: &mut PgConnection, user_id: UserId) -> Result<Vec<Post>, DieselError> {
    use crate::schema::{boosts, followers, posts};

    let on_schedule = posts::time_posted.lt(Utc::now());
    let public_only = posts::direct_message_to.is_null();
    let order = posts::time_posted.desc();
    let limit = 30;

    followers::table
        .filter(followers::user_id.eq(user_id))
        .inner_join(posts::table.on(followers::follows.eq(posts::user_id)))
        .filter(on_schedule)
        .filter(public_only)
        .select(Post::as_select())
        .order(order)
        .limit(limit)
        .union(
            followers::table
                .filter(followers::user_id.eq(user_id))
                .inner_join(boosts::table.on(boosts::user_id.eq(followers::follows)))
                .inner_join(posts::table.on(boosts::post_id.eq(posts::id)))
                .filter(on_schedule)
                .filter(public_only)
                .select(Post::as_select())
                .order(order)
                .limit(limit),
        )
        .get_results(conn)
}

pub fn get_liked_posts(conn: &mut PgConnection, user_id: UserId) -> Result<Vec<Post>, DieselError> {
    use crate::schema::{posts, reactions};

    reactions::table
        .inner_join(posts::table)
        .filter(reactions::user_id.eq(user_id))
        .filter(reactions::like_status.eq(1))
        .filter(posts::direct_message_to.is_null())
        .select(Post::as_select())
        .limit(30)
        .get_results(conn)
}

pub fn get_bookmarked_posts(
    conn: &mut PgConnection,
    user_id: UserId,
) -> Result<Vec<Post>, DieselError> {
    use crate::schema::{bookmarks, posts};

    bookmarks::table
        .inner_join(posts::table)
        .filter(bookmarks::user_id.eq(user_id))
        .filter(posts::direct_message_to.is_null())
        .select(Post::as_select())
        .limit(30)
        .get_results(conn)
}
