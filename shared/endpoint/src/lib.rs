use load_dotenv::try_load_dotenv;
use serde::{Deserialize, Serialize};
pub mod post;
pub use post::endpoint::{Bookmark, Boost, NewPost, NewPostOk, TrendingPosts};
pub use post::types::LikeStatus;

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

try_load_dotenv!();

pub mod app_url {
    use std::str::FromStr;
    use url::Url;

    #[cfg(debug_assertions)]
    pub fn api_url() -> String {
        std::env::var("API_URL").expect("API_URL must be set")
    }

    #[cfg(not(debug_assertions))]
    pub fn api_url() -> String {
        std::env!("API_URL").to_string()
    }

    pub fn domain_and(fragment: &str) -> Url {
        Url::from_str(api_url().as_str())
            .and_then(|url| url.join(fragment))
            .unwrap()
    }

    pub mod user_content {
        pub const ROOT: &str = "usercontent/";
        pub const IMAGES: &str = "img/";

        pub const IMAGE_ROUTE: &str = "usercontent/img/";
    }
}

route!(CreateUser => "/register");
route!(Login => "/login");

route!(NewPost => "/post");
route!(TrendingPosts => "/posts");
route!(Bookmark => "/bookmark");
route!(Reaction => "/react");
route!(Boost => "/boost");
