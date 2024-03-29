use crate::config::url::{AbsoluteUrl, RelativeUrl};
use std::{collections::HashSet, error::Error, fmt::Display};

use reqwest::StatusCode;

#[derive(Debug)]
pub enum ValidationError {
    DuplicateSources(HashSet<RelativeUrl>),
    DuplicateDestinations(HashSet<AbsoluteUrl>),
    HttpStatusError { url: String, status: StatusCode },
}

impl Error for ValidationError {}

impl Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::DuplicateSources(v) => write!(f, "Duplicate sources: {v:?}"),
            ValidationError::DuplicateDestinations(v) => write!(f, "Duplicate destinations: {v:?}"),
            ValidationError::HttpStatusError { url, status } => {
                write!(f, "Unexpected http status {status:?} for \"{url}\"")
            }
        }
    }
}
