use crate::QueryError;
use diesel::prelude::*;
use password_hash::PasswordHashString;
use rustter_domain::ids::UserId;

pub fn new<T: AsRef<str>>(
    conn: &mut PgConnection,
    _hash: PasswordHashString,
    _handle: T,
) -> Result<UserId, QueryError> {
    use crate::schema::users::dsl::*;

    let user_id = UserId::new();

    diesel::insert_into(users)
        .values((
            id.eq(user_id),
            password_hash.eq(_hash.as_str()),
            handle.eq(_handle.as_ref()),
        ))
        .execute(conn)?;

    Ok(user_id)
}
