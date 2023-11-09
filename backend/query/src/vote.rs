use crate::DieselError;
use diesel::insert_into;
use diesel::prelude::*;
use rustter_domain::ids::{PollChoiceId, PostId, UserId};
use rustter_endpoint::post::types::VoteCast;

pub fn vote(
    conn: &mut PgConnection,
    user_id: UserId,
    post_id: PostId,
    choice_id: PollChoiceId,
) -> Result<VoteCast, DieselError> {
    let uid = user_id;
    let pid = post_id;
    let cid = choice_id;

    {
        use crate::schema::poll_votes::dsl::*;

        insert_into(poll_votes)
            .values((user_id.eq(uid), post_id.eq(pid), choice_id.eq(cid)))
            .on_conflict((user_id, post_id))
            .do_nothing()
            .execute(conn)
            .map(|n| {
                if n == 1 {
                    VoteCast::Yes
                } else {
                    VoteCast::AlreadyVoted
                }
            })
    }
}

// pub fn remove(
//     conn: &mut PgConnection,
//     user_id: UserId,
//     post_id: PostId,
// ) -> Result<DeleteStatus, DieselError> {
//     let uid = user_id;
//     let pid = post_id;
//
//     {
//         use crate::schema::bookmarks::dsl::*;
//
//         delete(bookmarks)
//             .filter(post_id.eq(pid))
//             .filter(user_id.eq(uid))
//             .execute(conn)
//             .map(|row_count| {
//                 if row_count == 0 {
//                     DeleteStatus::NotFound
//                 } else {
//                     DeleteStatus::Deleted
//                 }
//             })
//     }
// }

pub fn did_vote(
    conn: &mut PgConnection,
    user_id: UserId,
    post_id: PostId,
) -> Result<Option<PollChoiceId>, DieselError> {
    let uid = user_id;
    let pid = post_id;

    {
        use crate::schema::poll_votes::dsl::*;

        poll_votes
            .filter(post_id.eq(pid))
            .filter(user_id.eq(uid))
            .select(choice_id)
            .get_result(conn)
            .optional()
    }
}

pub struct PollResults {
    pub post_id: PostId,
    pub results: Vec<(PollChoiceId, i64)>,
}

pub fn get_poll_results(
    conn: &mut PgConnection,
    post_id: PostId,
) -> Result<PollResults, DieselError> {
    let pid = post_id;

    {
        use crate::schema::poll_votes::dsl::*;
        use diesel::dsl::count;

        let results = poll_votes
            .filter(post_id.eq(pid))
            .group_by(choice_id)
            .select((choice_id, count(choice_id)))
            .load::<(PollChoiceId, i64)>(conn)?;

        Ok(PollResults {
            post_id: pid,
            results,
        })
    }
}
