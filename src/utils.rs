use core::fmt;
use reqwest::StatusCode;
use std::{
    collections::HashSet,
    error::Error,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct HttpStatusError {
    pub url: String,
    pub status: StatusCode,
}

impl fmt::Display for HttpStatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Got status code {} for {}", self.status, self.url)
    }
}

impl Error for HttpStatusError {}

pub fn find_duplicates<'a, I, T>(iter: I) -> Option<HashSet<T>>
where
    I: Iterator<Item = &'a T>,
    T: 'static + std::hash::Hash + std::cmp::Eq + Clone,
{
    let mut set = HashSet::new();
    let mut duplicates = HashSet::<T>::new();
    iter.for_each(|v| {
        if !set.insert(v) {
            duplicates.insert(v.clone());
        }
    });

    if duplicates.is_empty() {
        None
    } else {
        Some(duplicates)
    }
}

pub fn search_common_paths(query: impl AsRef<Path>) -> Option<PathBuf> {
    if query.as_ref().exists() {
        return Some(PathBuf::from(query.as_ref()));
    }

    ["~/.config/kurzlink/", "/etc/kurzlink/"]
        .iter()
        .map(|dir| Path::new(dir).join(&query))
        .find(|path| path.exists())
}
