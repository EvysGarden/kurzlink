use std::collections::HashSet;

use crate::utils::BoxError;

pub enum ValidationError<'a> {
    DuplicateSources(HashSet<&'a String>),
    DuplicateDestinations(HashSet<&'a String>),
    NetworkError(BoxError),
}

impl ValidationError<'_> {
    pub fn error_code(&self) -> i32 {
        match self {
            ValidationError::DuplicateSources(_) => -2,
            ValidationError::DuplicateDestinations(_) => -3,
            ValidationError::NetworkError(_) => -4,
        }
    }
}
