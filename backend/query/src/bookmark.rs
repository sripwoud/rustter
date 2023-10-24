use crate::DieselError;
use diesel::prelude::*;
use diesel::{delete, insert_into};
use rustter_domain::ids::{PostId, UserId};

pub fn save(conn: &mut PgConnection, user_id: UserId, post_id: PostId) -> Result<(), DieselError> {
    let uid = user_id;
    let pid = post_id;

    {
        use crate::schema::bookmarks::dsl::*;

        insert_into(bookmarks)
            .values((user_id.eq(uid), post_id.eq(pid)))
            .on_conflict((user_id, post_id))
            .do_nothing()
            .execute(conn)
            .map(|_| ())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeleteStatus {
    Deleted,
    NotFound,
}
pub fn remove(
    conn: &mut PgConnection,
    user_id: UserId,
    post_id: PostId,
) -> Result<DeleteStatus, DieselError> {
    let uid = user_id;
    let pid = post_id;

    {
        use crate::schema::bookmarks::dsl::*;

        delete(bookmarks)
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
        use crate::schema::bookmarks::dsl::*;
        use diesel::dsl::count;

        bookmarks
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
