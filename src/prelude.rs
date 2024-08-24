pub use crate::error::Error;

pub type Result<T> = core::result::Result<T,Error>;

// wrapper for the newtype pattern
pub struct W<T>(pub T);

pub use std::format as fmt;