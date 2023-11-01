// use std::str::FromStr;
use crate::{ConstrainedText, UserFacingError};
use nutype::nutype;
// use derive_more::FromStr;

#[nutype(validate(min_len = 3, max_len = 30))]
#[derive(AsRef, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Username(String);

impl ConstrainedText for Username {
    const NAME: &'static str = "Username";
    const MAX_CHARS: usize = 30;
    fn min_chars() -> Option<usize> {
        Some(3)
    }
}

impl UserFacingError<Username> for UsernameError {
    fn formatted_error(&self) -> String {
        match self {
            UsernameError::TooShort => Username::too_short_error(),
            UsernameError::TooLong => Username::too_long_error(),
        }
    }
}

#[nutype(validate(min_len = 8))]
#[derive(AsRef, Clone, Deserialize, PartialEq, Serialize)]
pub struct Password(String);

impl ConstrainedText for Password {
    const NAME: &'static str = "Password";
    const MAX_CHARS: usize = 100;
    fn min_chars() -> Option<usize> {
        Some(8)
    }
}

impl UserFacingError<Password> for PasswordError {
    // TODO: consider changing result type to have a dynamic error (that would e.g. display length of current pwd)
    fn formatted_error(&self) -> String {
        match self {
            PasswordError::TooShort => Password::too_short_error(),
        }
    }
}

#[nutype(validate(max_len = 30))]
#[derive(AsRef, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DisplayName(String);

impl ConstrainedText for DisplayName {
    const NAME: &'static str = "Display name";
    const MAX_CHARS: usize = 30;
}

impl UserFacingError<DisplayName> for DisplayNameError {
    fn formatted_error(&self) -> String {
        match self {
            DisplayNameError::TooLong => DisplayName::too_long_error(),
        }
    }
}
