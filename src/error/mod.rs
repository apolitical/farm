use std::result;

mod error;

pub type Result<T> = result::Result<T, error::Error>;
