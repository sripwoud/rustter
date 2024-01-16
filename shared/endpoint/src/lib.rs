use load_dotenv::load_dotenv;
use serde::{Deserialize, Serialize};
pub mod post;
pub use post::endpoint::{
    Bookmark, BookmarkedPosts, Boost, BoostOk, HomePosts, LikedPosts, NewPost, NewPostOk,
    TrendingPosts, Vote, VoteOk,
};
pub use post::types::LikeStatus;

mod reaction;
pub mod user;
pub use reaction::{Reaction, ReactionOk};

pub use user::endpoint::{
    CreateUser, CreateUserOk, GetMyProfile, GetMyProfileOk, Login, LoginOk, Update, UpdateProfile,
    UpdateProfileOk, ViewProfile, ViewProfileOk,
};

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

load_dotenv!();

pub mod app_url {
    use std::str::FromStr;
    use url::Url;

    pub const API_URL: &str = std::env!("API_URL");

    pub fn domain_and(fragment: &str) -> Url {
        Url::from_str(API_URL)
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
route!(HomePosts => "/posts");
route!(BookmarkedPosts => "/posts/bookmarked");
route!(LikedPosts => "/posts/liked");
route!(TrendingPosts => "/posts/trending");
route!(Bookmark => "/bookmark");
route!(Reaction => "/react");
route!(Boost => "/boost");
route!(Vote => "/vote");
route!(GetMyProfile => "/profile/me");
route!(UpdateProfile => "/profile/update");
route!(ViewProfile => "/profile/view");
