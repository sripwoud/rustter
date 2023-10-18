use crate::{DieselError, QueryError};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use password_hash::PasswordHashString;
use rustter_domain::ids::UserId;
use rustter_domain::Username;

#[derive(Debug, Queryable)]
pub struct User {
    pub id: UserId,
    pub email: Option<String>,
    pub email_confirm: Option<DateTime<Utc>>,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub handle: String,
    pub created_at: DateTime<Utc>,
    pub profile_image: Option<String>,
}

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
pub fn get_password_hash(
    conn: &mut PgConnection,
    username: &Username,
) -> Result<String, QueryError> {
    use crate::schema::users::dsl::*;

    Ok(users
        .filter(handle.eq(username.as_ref()))
        .select(password_hash)
        .get_result(conn)?)
}

pub fn get(conn: &mut PgConnection, user_id: UserId) -> Result<User, DieselError> {
    use crate::schema::users::dsl::*;
    users.find(user_id).first(conn)
}

pub fn find(conn: &mut PgConnection, username: &Username) -> Result<User, DieselError> {
    use crate::schema::users::dsl::*;
    users.filter(handle.eq(username.as_ref())).get_result(conn)
}
