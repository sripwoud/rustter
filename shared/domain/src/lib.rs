#[cfg(feature = "query")]
#[macro_use]
extern crate diesel_derive_newtype;

pub mod ids;
pub mod post;
pub mod user;

pub use user::{Password, Username};

pub trait ConstrainedText {
    const NAME: &'static str;
    const MAX_CHARS: usize;

    fn min_chars() -> Option<usize> {
        None
    }
    fn too_short_error() -> String {
        match Self::min_chars() {
            Some(min_length) => match min_length {
                1 => format!("{} cannot be empty", Self::NAME),
                _ => format!(
                    "{} is too short (min {} characters)",
                    Self::NAME,
                    min_length
                ),
            },
            None => format!("{} is too short", Self::NAME),
        }
    }

    fn too_long_error() -> String {
        format!(
            "{} is too long (max {} characters)",
            Self::NAME,
            Self::MAX_CHARS
        )
    }
}

pub trait ConstrainedUserFacingError<T: ConstrainedText> {
    fn formatted_error(&self) -> String;
}

pub trait UserFacingError {
    fn formatted_error(&self) -> String;
}
