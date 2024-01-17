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
    CreateUser, CreateUserOk, Follow, FollowOk, GetMyProfile, GetMyProfileOk, Login, LoginOk,
    Update, UpdateProfile, UpdateProfileOk, ViewProfile, ViewProfileOk,
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

pub mod app_url {
    use std::env;
    use std::str::FromStr;
    use url::Url;

    #[cfg(debug_assertions)]
    pub fn api_url() -> String {
        if env::var("RUST_CI").is_err() {
            dotenvy::dotenv().expect(".env file not found");
        }
        env::var("API_URL").expect("API_URL must be set")
    }

    #[cfg(not(debug_assertions))]
    pub fn api_url() -> String {
        // in prod, env var is only available at runtime (deployment secret)
        env::var("API_URL").expect("API_URL must be set")
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
route!(Follow => "/user/follow");
