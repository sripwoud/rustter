use super::types::Content;
use crate::post::types::{NewPostOptions, PublicPost};
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
pub struct TrendingPostsOk(Vec<PublicPost>);
