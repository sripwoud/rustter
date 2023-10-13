use serde::{Deserialize, Serialize};
use rustter_domain::ids::PostId;
use crate::post::types::NewPostOptions;
use super::types::PostType;

#[derive(Clone,Debug,Deserialize,Serialize, PartialEq)]
pub struct NewPost {
    pub r#type: PostType,
    pub options: NewPostOptions
}

#[derive(Clone,Debug,Deserialize,Serialize, PartialEq)]
pub struct NewPostOk {
    pub post_id: PostId
}