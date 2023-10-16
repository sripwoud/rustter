use serde::{Deserialize, Serialize};

pub mod post;
pub use post::endpoint::{NewPost, NewPostOk, TrendingPosts};
pub mod user;

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

route!(CreateUser => "/account/create");
route!(Login => "/account/login");
route!(NewPost => "/post/new");
route!(TrendingPosts => "/posts");
