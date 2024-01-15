use crate::{ConstrainedText, ConstrainedUserFacingError, UserFacingError};
use nutype::nutype;
use once_cell::sync::OnceCell;
use regex::Regex;

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

impl ConstrainedUserFacingError<Username> for UsernameError {
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

impl ConstrainedUserFacingError<Password> for PasswordError {
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

impl ConstrainedUserFacingError<DisplayName> for DisplayNameError {
    fn formatted_error(&self) -> String {
        match self {
            DisplayNameError::TooLong => DisplayName::too_long_error(),
        }
    }
}

#[derive(Debug)]
struct EmailRgx(Regex);
impl EmailRgx {
    pub fn global() -> &'static Self {
        EMAIL_RGX.get().expect("email regex is not initialized")
    }

    pub fn init() -> Self {
        // TODO: can be improved
        Self(Regex::new(r#"^\S+@\S+\.\S{1,64}$"#).unwrap())
    }

    pub fn is_valid<T:AsRef<str>>(&self, text:T) -> bool {
        self.0.is_match(text.as_ref())
    }
}
static EMAIL_RGX: OnceCell<EmailRgx> = OnceCell::new();

fn is_valid_email(email: &str) -> bool {
    let email_regex = EMAIL_RGX.get_or_init(EmailRgx::init);
    email_regex.is_valid(email)
}

#[nutype(validate(with=is_valid_email))]
#[derive(AsRef, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Email(String);

impl UserFacingError for EmailError {
    fn formatted_error(&self) -> String {
        match self {
            EmailError::Invalid => "Email is not valid. Expect format: mail.domain.com".to_string()
        }
    }
}


