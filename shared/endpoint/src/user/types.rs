use chrono::{DateTime, Utc};
use rustter_domain::ids::UserId;
use rustter_domain::user::DisplayName;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PublicUserProfile {
    pub id: UserId,
    pub display_name: Option<DisplayName>,
    pub handle: String,
    pub profile_image: Option<Url>,
    pub created_at: DateTime<Utc>,
    pub am_following: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub enum FollowAction {
    Follow,
    Unfollow,
}

impl From<FollowAction> for bool {
    fn from(action: FollowAction) -> Self {
        match action {
            FollowAction::Follow => true,
            FollowAction::Unfollow => false,
        }
    }
}
