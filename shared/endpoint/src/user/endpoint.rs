use super::types::{FollowAction, PublicUserProfile};
use crate::post::types::PublicPost;
use chrono::{DateTime, Utc};
use rustter_domain::{ids::*, Password, Username};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Deserialize, Serialize)]
pub struct CreateUser {
    pub username: Username,
    pub password: Password,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct CreateUserOk {
    pub user_id: UserId,
    pub username: Username,

    pub session_signature: String,
    pub session_id: SessionId,
    pub session_expires: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Login {
    pub username: Username,
    pub password: Password,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct LoginOk {
    pub session_signature: String,
    pub session_id: SessionId,
    pub session_expires: DateTime<Utc>,

    pub display_name: Option<String>,
    pub email: Option<String>,
    pub profile_image: Option<Url>,
    pub user_id: UserId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Update<T> {
    Change(T),
    NoChange,
    SetNull,
}

impl<T> Update<T> {
    pub fn into_option(self) -> Option<T> {
        match self {
            Self::Change(data) => Some(data),
            Self::NoChange => None,
            Self::SetNull => None,
        }
    }

    pub fn into_nullable(self) -> Option<Option<T>> {
        match self {
            Self::Change(data) => Some(Some(data)),
            Self::NoChange => None,
            Self::SetNull => Some(None),
        }
    }
}

// Get profile of authed user
#[derive(Clone, Deserialize, Serialize)]
pub struct GetMyProfile;

#[derive(Clone, Deserialize, Serialize)]
pub struct GetMyProfileOk {
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub profile_image: Option<Url>,
    pub user_id: UserId,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct UpdateProfile {
    pub display_name: Update<String>,
    pub email: Update<String>,
    pub profile_image: Update<String>,
    pub password: Update<Password>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct UpdateProfileOk {
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub profile_image: Option<Url>,
    pub user_id: UserId,
}

// View public profile of other user
#[derive(Clone, Deserialize, Serialize)]
pub struct ViewProfile {
    pub for_user: UserId,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ViewProfileOk {
    pub profile: PublicUserProfile,
    pub posts: Vec<PublicPost>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Follow {
    pub user_id: UserId,
    pub action: FollowAction,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FollowOk {
    pub status: FollowAction,
}
