pub use crate::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub struct W<T>(pub T);

pub use std::format as f;
