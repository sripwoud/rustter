use crate::{DeleteStatus, DieselError};
use diesel::prelude::*;
use diesel::{delete, insert_into};
use rustter_domain::ids::UserId;

pub fn follow(conn: &mut PgConnection, user_id: UserId, follow: UserId) -> Result<(), DieselError> {
    let uid = user_id;
    let fid = follow;

    {
        use crate::schema::followers::dsl::*;

        insert_into(followers)
            .values((user_id.eq(uid), follows.eq(fid)))
            .on_conflict((user_id, follows))
            .do_nothing()
            .execute(conn)
            .map(|_| ())
    }
}

pub fn unfollow(
    conn: &mut PgConnection,
    user_id: UserId,
    stop_following: UserId,
) -> Result<DeleteStatus, DieselError> {
    let uid = user_id;
    let fid = stop_following;

    {
        use crate::schema::followers::dsl::*;

        delete(followers)
            .filter(follows.eq(fid))
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
