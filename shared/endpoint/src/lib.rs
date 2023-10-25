use serde::{Deserialize, Serialize};

pub mod post;
pub use post::endpoint::{Bookmark, NewPost, NewPostOk, TrendingPosts};
mod reaction;
pub mod user;
pub use reaction::{Reaction, ReactionOk};

pub use user::endpoint::{CreateUser, CreateUserOk, Login, LoginOk};

pub trait Endpoint {
    const URL: &'static str;

    fn url(&self) -> &'static str {
        Self::URL
    }
}

#[derive(thiserror::Error, Debug, Deserialize, Serialize)]
#[error("{msg}")]
pub struct RequestFailed {
    pub msg: String,
}

#[macro_export]
macro_rules! route {
    ($name:ident => $url:expr) => {
        impl $crate::Endpoint for $name {
            const URL: &'static str = $url;
        }
    };
}

route!(CreateUser => "/register");
route!(Login => "/login");

route!(NewPost => "/post");
route!(TrendingPosts => "/posts");
route!(Bookmark => "/bookmark");
route!(Reaction => "/react");
