use crate::user::types::PublicUserProfile;
use chrono::{DateTime, Utc};
use rustter_domain::ids::{PostId, UserId};
use rustter_domain::post::{Headline, Message};
use rustter_domain::Username;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum Content {
    Chat(Chat),
    Image(Image),
    Poll(Poll),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct NewPostOptions {
    pub reply_to: Option<PostId>,
    /// TODO: no interface for this yet
    pub direct_message_to: Option<UserId>,
    pub time_posted: DateTime<Utc>,
}

impl Default for NewPostOptions {
    fn default() -> Self {
        Self {
            reply_to: None,
            direct_message_to: None,
            time_posted: Utc::now(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Chat {
    pub headline: Option<Headline>,
    pub message: Message,
}

impl From<Chat> for Content {
    fn from(chat: Chat) -> Self {
        Self::Chat(chat)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Image {
    pub src: String,
    pub caption: String,
}

impl From<Image> for Content {
    fn from(image: Image) -> Self {
        Self::Image(image)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Poll {
    pub headline: Option<Headline>,
    pub options: Vec<String>,
}

impl From<Poll> for Content {
    fn from(poll: Poll) -> Self {
        Self::Poll(poll)
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum LikeStatus {
    Like,
    Dislike,
    NoReaction,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PublicPost {
    pub id: PostId,
    pub author: PublicUserProfile,
    pub content: Content,
    pub time_posted: DateTime<Utc>,
    pub reply_to: Option<(Username, UserId, PostId)>,
    pub like_status: LikeStatus,
    pub bookmarked: bool,
    pub boosted: bool, // aka retweet
    pub boosts: i64,
    pub likes: i64,
    pub dislikes: i64,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub enum BookmarkAction {
    Save,
    Remove,
}

impl From<BookmarkAction> for bool {
    fn from(action: BookmarkAction) -> Self {
        match action {
            BookmarkAction::Save => true,
            BookmarkAction::Remove => false,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub enum BoostAction {
    Add,
    Remove,
}

impl From<BoostAction> for bool {
    fn from(action: BoostAction) -> Self {
        match action {
            BoostAction::Add => true,
            BoostAction::Remove => false,
        }
    }
}
