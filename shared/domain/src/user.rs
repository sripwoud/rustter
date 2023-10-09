use std::str::ParseBoolError;
use nutype::nutype;
use crate::UserFacingError;

#[nutype(validate(min_len = 3, max_len = 30))]
#[derive(AsRef, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Username(String);

impl UserFacingError for UsernameError {
    fn formatted_error(&self) -> &'static str {
        match self {
            UsernameError::TooShort => "Username is too short (min 3 characters)",
            UsernameError::TooLong => "Username is too long (max 30 characters)",
        }
    }
}

#[nutype(validate(min_len = 8))]
#[derive(AsRef, Clone, Deserialize, PartialEq, Serialize)]
pub struct Password(String);

impl UserFacingError for PasswordError {
    // TODO: consider changing result type to have a dynamic error (that would e.g. display length of current pwd)
    fn formatted_error(&self) -> &'static str {
        match self {
            PasswordError::TooShort => "Password is too short (min 8 characters)"
        }
    }
}
