use chrono::{DateTime, Utc};
use rustter_domain::ids::{PostId, UserId};
use rustter_domain::post::{Headline, Message};
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
