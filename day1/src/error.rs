use std::fmt::Debug;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Day1Error {
    #[error("pase text to postion error: {0}")]
    ParseTextError(String),
}
