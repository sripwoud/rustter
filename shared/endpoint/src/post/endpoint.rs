use super::types::Content;
use crate::post::types::NewPostOptions;
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
