use std::collections::HashSet;

use crate::utils::BoxError;

#[derive(Debug)]
pub enum ValidationError<'a> {
    DuplicateSources(HashSet<&'a String>),
    DuplicateDestinations(HashSet<&'a String>),
    NetworkError(BoxError),
}
