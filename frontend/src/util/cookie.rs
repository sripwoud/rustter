#![allow(dead_code)]

use super::document;
use chrono::{DateTime, Duration, Utc};
use rustter_cookie::CookieKey;
use rustter_domain::ids;
use rustter_domain::ids::SessionId;
use std::str::FromStr;

pub fn get_session() -> Option<ids::SessionId> {
    let cookies = document().cookie().unwrap();
    rustter_cookie::get_from_str(&cookies, "session_id")
        .and_then(|id| ids::SessionId::from_str(id).ok())
}

fn set_session_cookie(cookie_type: CookieKey) -> impl Fn(String, DateTime<Utc>) {
    move |cookie_value, expires| {
        let cookie = format_cookie(format_kv(cookie_type.clone(), cookie_value), expires);
        document().set_cookie(&cookie).unwrap()
    }
}

fn set_session_id(session_id: SessionId, expires: DateTime<Utc>) {
    set_session_cookie(rustter_cookie::SESSION_ID)(session_id.into(), expires)
}

fn set_session_signature(session_signature: String, expires: DateTime<Utc>) {
    set_session_cookie(rustter_cookie::SESSION_SIGNATURE)(session_signature, expires)
}

pub fn set_session(session_id: SessionId, session_signature: String, expires: DateTime<Utc>) {
    set_session_id(session_id, expires);
    set_session_signature(session_signature, expires);
}

pub fn remove_session() {
    let cookie = format_cookie(
        format_kv(rustter_cookie::SESSION_ID, ""),
        Utc::now() - Duration::days(1),
    );
    document().set_cookie(&cookie).unwrap()
}

fn standard_options() -> &'static str {
    "SameSite=None; Path=/; Secure"
}

// #[cfg(not(debug_assertions))]
// fn standard_options() -> &'static str {
//     "SameSite=Strict; Path=/; Secure"
// }
//
// #[cfg(debug_assertions)]
// fn standard_options() -> &'static str {
//     "SameSite=Strict; Path=/;"
// }

fn format_expiration(expires: DateTime<Utc>) -> String {
    expires.format("expires=%a, %d %b %Y %T GMT").to_string()
}

fn format_kv<K, V>(key: K, value: V) -> String
where
    K: AsRef<str>,
    V: AsRef<str>,
{
    let key = key.as_ref();
    let value = value.as_ref();
    format!("{key}={value}")
}

fn format_cookie<S: AsRef<str>>(payload: S, expires: DateTime<Utc>) -> String {
    let expires = format_expiration(expires);
    let options = standard_options();
    let payload = payload.as_ref();

    let cookie = format!("{payload}; {expires}; {options}");
    cookie
}
