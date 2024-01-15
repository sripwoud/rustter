use crate::{DieselError, QueryError};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use password_hash::PasswordHashString;
use rustter_domain::ids::UserId;
use rustter_domain::Username;
use rustter_endpoint::user::endpoint::Update;
use url::Url;

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

impl User {
    pub fn profile_image_url_from_id(&self) -> Option<Url> {
        use rustter_endpoint::app_url::{self, user_content};
        self.profile_image.as_ref().map(|id| {
            app_url::domain_and(user_content::ROOT)
                .join(user_content::IMAGES)
                .unwrap()
                .join(id)
                .unwrap()
        })
    }
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

#[derive(Debug)]
pub struct UpdateProfileParams {
    pub id: UserId,
    pub display_name: Update<String>,
    pub email: Update<String>,
    pub password_hash: Update<PasswordHashString>,
    pub profile_image: Update<String>,
}

#[derive(AsChangeset, Debug)]
#[diesel(table_name = crate::schema::users)]
struct UpdateProfileParamsInternal {
    pub display_name: Option<Option<String>>, // outer Option controls wether we will perform a DB update, inner option controls whether will store a non null or an explicilty null value
    pub email: Option<Option<String>>,
    pub password_hash: Option<String>,
    pub profile_image: Option<Option<String>>,
}

pub fn update_profile(
    conn: &mut PgConnection,
    query_params: UpdateProfileParams,
) -> Result<(), DieselError> {
    use crate::schema::users::dsl::*;

    let update = UpdateProfileParamsInternal {
        display_name: query_params.display_name.into_nullable(),
        email: query_params.email.into_nullable(),
        password_hash: query_params
            .password_hash
            .into_option()
            .map(|s| s.to_string()),
        profile_image: query_params.profile_image.into_nullable(),
    };

    diesel::update(users)
        .filter(id.eq(&query_params.id))
        .set(&update)
        .execute(conn)
        .map(|_| ())
}
