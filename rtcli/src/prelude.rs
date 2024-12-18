pub use crate::error::Error;
// re-exporting the vector extensions publicly
pub use crate::vec3::{Vec3Ext, Color};

pub type Result<T> = core::result::Result<T,Error>;

// wrapper for the newtype pattern
pub struct W<T>(pub T);

