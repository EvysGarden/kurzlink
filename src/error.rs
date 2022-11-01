use std::collections::HashSet;

use crate::utils::BoxError;

pub enum ValidationError<'a> {
    DuplicateSources(HashSet<&'a String>),
    DuplicateDestinations(HashSet<&'a String>),
    NetworkError(BoxError),
}
