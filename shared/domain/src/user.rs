use crate::UserFacingError;
use nutype::nutype;

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
            PasswordError::TooShort => "Password is too short (min 8 characters)",
        }
    }
}

#[nutype(validate(max_len = 30))]
#[derive(AsRef, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DisplayName(String);

impl DisplayName {
    pub const MAX_CHARACTERS: usize = 30;
}

impl UserFacingError for DisplayNameError {
    fn formatted_error(&self) -> &'static str {
        match self {
            DisplayNameError::TooLong => "Display name is too long (max 30 characters)",
        }
    }
}
