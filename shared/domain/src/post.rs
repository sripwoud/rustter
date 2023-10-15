use crate::UserFacingError;
use nutype::nutype;

#[nutype(validate(max_len = 30))]
#[derive(AsRef, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Headline(String);
impl Headline {
    pub const MAX_CHARS: usize = 30;
}

impl UserFacingError for HeadlineError {
    fn formatted_error(&self) -> &'static str {
        match self {
            HeadlineError::TooLong => "Headline is too long (max 30 characters)",
        }
    }
}

#[nutype(validate(min_len = 1, max_len = 100))]
#[derive(AsRef, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Message(String);
impl Message {
    pub const MAX_CHARS: usize = 100;
}

impl UserFacingError for MessageError {
    fn formatted_error(&self) -> &'static str {
        match self {
            MessageError::TooShort => "Message is too short (min 1 characters)",
            MessageError::TooLong => "Message is too long (max 100 characters)",
        }
    }
}
