use crate::{DeleteStatus, DieselError};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::{delete, insert_into};
use rustter_domain::ids::{PostId, UserId};

pub fn boost(
    conn: &mut PgConnection,
    user_id: UserId,
    post_id: PostId,
    when: DateTime<Utc>,
) -> Result<(), DieselError> {
    let uid = user_id;
    let pid = post_id;

    {
        use crate::schema::boosts::dsl::*;

        insert_into(boosts)
            .values((user_id.eq(uid), post_id.eq(pid), boosted_at.eq(when)))
            .on_conflict((user_id, post_id))
            .do_update()
            .set(boosted_at.eq(when))
            .execute(conn)
            .map(|_| ())
    }
}

pub fn remove(
    conn: &mut PgConnection,
    user_id: UserId,
    post_id: PostId,
) -> Result<DeleteStatus, DieselError> {
    let uid = user_id;
    let pid = post_id;

    {
        use crate::schema::boosts::dsl::*;

        delete(boosts)
            .filter(post_id.eq(pid))
            .filter(user_id.eq(uid))
            .execute(conn)
            .map(|row_count| {
                if row_count == 0 {
                    DeleteStatus::NotFound
                } else {
                    DeleteStatus::Deleted
                }
            })
    }
}

pub fn get(conn: &mut PgConnection, user_id: UserId, post_id: PostId) -> Result<bool, DieselError> {
    let uid = user_id;
    let pid = post_id;

    {
        use crate::schema::boosts::dsl::*;
        use diesel::dsl::count;

        boosts
            .filter(post_id.eq(pid))
            .filter(user_id.eq(uid))
            .select(count(post_id))
            .get_result(conn)
            .optional()
            .map(|n: Option<i64>| match n {
                Some(n) => n > 0,
                None => false,
            })
    }
}
