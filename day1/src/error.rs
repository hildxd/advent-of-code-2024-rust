use std::fmt::Debug;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Day1Error {
    #[error("parse text to postion error: {0}")]
    ParseTextError(String),

    #[error("parse text to i64 error: {0}")]
    ParseIntError(String),
}
