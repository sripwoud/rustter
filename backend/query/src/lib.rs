#[macro_use]
extern crate diesel_derive_newtype;

#[cfg(test)]
pub mod test_db;

pub use diesel::result::Error as DieselError;

pub mod error;
pub use error::QueryError;

pub mod schema;
pub mod util;

pub use util::{AsyncConnection, AsyncConnectionPool, OwnedAsyncConnection};

pub mod bookmark;
pub mod boost;
pub mod post;
pub mod reaction;
pub mod session;
pub mod user;
pub mod vote;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeleteStatus {
    Deleted,
    NotFound,
}
