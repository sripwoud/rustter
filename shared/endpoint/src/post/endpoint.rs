use super::types::{Content, VoteCast};
use crate::post::types::{BookmarkAction, BoostAction, NewPostOptions, PublicPost};
use rustter_domain::ids::{PollChoiceId, PostId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct NewPost {
    pub content: Content,
    pub options: NewPostOptions,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct NewPostOk {
    pub post_id: PostId,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Bookmark {
    pub post_id: PostId,
    pub action: BookmarkAction,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BookmarkOk {
    pub status: BookmarkAction,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Boost {
    pub post_id: PostId,
    pub action: BoostAction,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BoostOk {
    pub status: BoostAction,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Vote {
    pub post_id: PostId,
    pub choice_id: PollChoiceId,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct VoteOk {
    pub cast: VoteCast,
}

#[macro_export]
macro_rules! PostsEndpoint {
    ($name:ident, $name_ok:ident) => {
        #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
        pub struct $name;

        #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
        pub struct $name_ok(pub Vec<PublicPost>);
    };
}

PostsEndpoint!(TrendingPosts, TrendingPostsOk);
PostsEndpoint!(HomePosts, HomePostsOk);
PostsEndpoint!(BookmarkedPosts, BookmarkedPostsOk);
PostsEndpoint!(LikedPosts, LikedPostsOk);
