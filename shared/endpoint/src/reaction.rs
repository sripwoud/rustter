use crate::post::types::LikeStatus;
use rustter_domain::ids::PostId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Reaction {
    pub post_id: PostId,
    pub like_status: LikeStatus,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ReactionOk {
    pub post_id: PostId,
    pub likes: i64,
    pub dislikes: i64,
    pub like_status: LikeStatus,
}
