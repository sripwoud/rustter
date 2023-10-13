use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use rustter_domain::ids::{PostId, UserId};
use rustter_domain::post::{Headline, Message};

pub enum PostType {
    Chat(Chat),
    Image(Image),
    // Poll(Poll),
}

pub struct NewPostOptions {
    pub reply_to: Option<PostId>,
    /// TODO: no interface for this yet
    pub direct_message_to:Option<UserId>,
    pub time_posted: DateTime<Utc>
}

impl Default for NewPostOptions {
    fn default() -> Self {
        Self {
            reply_to: None,
            direct_message_to: None,
            time_posted: Utc::now()
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Chat {
    pub headline: Option<Headline>,
    pub message: Message,
}

impl From<Chat> for PostType {
    fn from(chat: Chat) -> Self {
        Self::Chat(chat)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Image {
    pub src: String,
    pub caption: String,
}

impl From<Image> for PostType {
    fn from(image: Image) -> Self {
        Self::Image(image)
    }
}


#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Poll {
    pub headline: Option<Headline>,
    pub options: Vec<String>,
}

impl From<Poll> for PostType {
    fn from(poll: Poll) -> Self {
        Self::Poll(poll)
    }
}