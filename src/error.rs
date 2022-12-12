use std::{collections::HashSet, error::Error, fmt::Display};

use reqwest::StatusCode;

use crate::utils::BoxError;

#[derive(Debug)]
pub enum ValidationError {
    DuplicateSources(HashSet<String>),
    DuplicateDestinations(HashSet<String>),
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
