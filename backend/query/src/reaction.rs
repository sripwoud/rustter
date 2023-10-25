use crate::{schema, DieselError};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::insert_into;
use rustter_domain::ids::{PostId, UserId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, DieselNewType, Deserialize, Serialize)]
pub struct ReactionData(serde_json::Value);

#[derive(Clone, Debug, Queryable, Selectable, Insertable, Deserialize, Serialize)]
#[diesel(table_name = schema::reactions)]
pub struct Reaction {
    pub user_id: UserId,
    pub post_id: PostId,
    pub created_at: DateTime<Utc>,
    pub like_status: i16,
    pub reaction: Option<ReactionData>,
}

pub fn react(conn: &mut PgConnection, reaction: Reaction) -> Result<(), DieselError> {
    let reac = reaction;
    {
        use crate::schema::reactions::dsl::*;
        insert_into(reactions)
            .values(&reac)
            .on_conflict((user_id, post_id))
            .do_update()
            .set((
                like_status.eq(&reac.like_status),
                reaction.eq(&reac.reaction),
            ))
            .execute(conn)
            .map(|_| ())
    }
}

pub fn get(
    conn: &mut PgConnection,
    post_id: PostId,
    user_id: UserId,
) -> Result<Option<Reaction>, DieselError> {
    let pid = post_id;
    let uid = user_id;
    {
        use crate::schema::reactions::dsl::*;
        reactions
            .filter(post_id.eq(pid))
            .filter(user_id.eq(uid))
            .get_result(conn)
            .optional()
    }
}
