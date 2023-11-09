use crate::{ConstrainedText, UserFacingError};
use nutype::nutype;

#[nutype(validate(max_len = 30))]
#[derive(AsRef, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Headline(String);
impl ConstrainedText for Headline {
    const NAME: &'static str = "Headline";
    const MAX_CHARS: usize = 30;
}

impl UserFacingError<Headline> for HeadlineError {
    fn formatted_error(&self) -> String {
        match self {
            HeadlineError::TooLong => Headline::too_long_error(),
        }
    }
}

#[nutype(validate(min_len = 1, max_len = 100))]
#[derive(AsRef, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Message(String);
impl ConstrainedText for Message {
    const NAME: &'static str = "Message";
    const MAX_CHARS: usize = 100;
    fn min_chars() -> Option<usize> {
        Some(1)
    }
}

impl UserFacingError<Message> for MessageError {
    fn formatted_error(&self) -> String {
        match self {
            MessageError::TooShort => Message::too_short_error(),
            MessageError::TooLong => Message::too_long_error(),
        }
    }
}

#[nutype(validate(min_len = 1, max_len = 100))]
#[derive(AsRef, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Caption(String);
impl ConstrainedText for Caption {
    const NAME: &'static str = "Caption";
    const MAX_CHARS: usize = 60;
    fn min_chars() -> Option<usize> {
        Some(1)
    }
}

impl UserFacingError<Caption> for CaptionError {
    fn formatted_error(&self) -> String {
        match self {
            CaptionError::TooShort => Caption::too_short_error(),
            CaptionError::TooLong => Caption::too_long_error(),
        }
    }
}

#[nutype(validate(max_len = 30))]
#[derive(AsRef, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PollHeadline(String);
impl ConstrainedText for PollHeadline {
    const NAME: &'static str = "Poll headline";
    const MAX_CHARS: usize = 50;
}

impl UserFacingError<PollHeadline> for PollHeadlineError {
    fn formatted_error(&self) -> String {
        match self {
            PollHeadlineError::TooLong => PollHeadline::too_long_error(),
        }
    }
}

#[nutype(validate(min_len = 1, max_len = 80))]
#[derive(AsRef, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PollChoiceDescription(String);
impl ConstrainedText for PollChoiceDescription {
    const NAME: &'static str = "Poll choice description";
    const MAX_CHARS: usize = 80;
    fn min_chars() -> Option<usize> {
        Some(1)
    }
}

impl UserFacingError<PollChoiceDescription> for PollChoiceDescriptionError {
    fn formatted_error(&self) -> String {
        match self {
            PollChoiceDescriptionError::TooShort => PollChoiceDescription::too_short_error(),
            PollChoiceDescriptionError::TooLong => PollChoiceDescription::too_long_error(),
        }
    }
}
