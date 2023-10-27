use super::types::Content;
use crate::post::types::{BookmarkAction, BoostAction, NewPostOptions, PublicPost};
use rustter_domain::ids::PostId;
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
pub struct TrendingPosts; // no request data, we just return the most recent posts

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TrendingPostsOk(pub Vec<PublicPost>);

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
