use crate::{schema, DieselError};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::{insert_into, Insertable, PgConnection, Queryable, RunQueryDsl};
use rustter_domain::ids::{SessionId, UserId};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, DieselNewType, PartialEq, Serialize)]
pub struct Fingerprint(Value);

impl From<Value> for Fingerprint {
    fn from(value: Value) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, PartialEq, Queryable, Insertable)]
#[diesel(table_name = schema::web)]
pub struct Session {
    // order needs to match database table's in order to be able to derive the Queryable and Insertable traits
    pub id: SessionId,
    pub user_id: UserId,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub fingerprint: Fingerprint,
}

impl Session {
    pub fn new(user_id: UserId, fingerprint: Fingerprint, duration: chrono::Duration) -> Self {
        let now = Utc::now();
        Self {
            id: SessionId::new(),
            user_id,
            expires_at: now + duration,
            created_at: now,
            fingerprint,
        }
    }
}

pub fn new(
    conn: &mut PgConnection,
    user_id: UserId,
    duration: chrono::Duration,
    fingerprint: Fingerprint,
) -> Result<Session, DieselError> {
    let _user_id = user_id;
    let new_session = Session::new(_user_id, fingerprint, duration);

    {
        use crate::schema::web::dsl::*;
        insert_into(web)
            .values(&new_session)
            .on_conflict((user_id, fingerprint))
            .do_update()
            .set(expires_at.eq(new_session.expires_at))
            .get_result(conn)
    }
}

pub fn get(conn: &mut PgConnection, session_id: SessionId) -> Result<Option<Session>, DieselError> {
    use crate::schema::web::dsl::*;
    web.filter(id.eq(session_id)).get_result(conn).optional()
}

pub fn find(
    conn: &mut PgConnection,
    user_id: UserId,
    fingerprint: Fingerprint,
) -> Result<Session, DieselError> {
    let _user_id = user_id;
    let _fingerprint = fingerprint;

    {
        use crate::schema::web::dsl::*;
        web.filter(id.eq(_user_id))
            .filter(fingerprint.eq(_fingerprint))
            .get_result(conn)
    }
}
