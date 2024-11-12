pub use crate::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

// With this prelude module, we can now use the Result type alias in other modules without having to import it.
pub struct W<T>(pub T);
