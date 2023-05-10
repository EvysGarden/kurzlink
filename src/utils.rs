use core::fmt;
use reqwest::StatusCode;
use std::{
    collections::HashSet,
    error::Error,
    path::{Path, PathBuf},
    time::Duration,
};

use crate::config::url::AbsoluteUrl;
use crate::error::ValidationError;

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

pub fn check_url(url: &AbsoluteUrl, timeout: u64) -> anyhow::Result<()> {
    let client = reqwest::blocking::Client::new();
    match client
        .get(url.inner())
        .timeout(Duration::new(timeout, 0))
        .send()
    {
        Ok(result) => {
            if result.status().is_success() {
                Ok(())
            } else {
                Err(ValidationError::HttpStatusError {
                    url: url.inner().clone(),
                    status: result.status(),
                }
                .into())
            }
        }
        Err(err) => Err(anyhow::Error::msg(err.to_string()))
    }
}

pub fn check_urls(urls: &Vec<AbsoluteUrl>, timeout: u64) -> anyhow::Result<()> {
    for url in urls {
        check_url(url, timeout)?;
    }
    Ok(())
}

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

    vec!["~/.config/kurzlink/", "/etc/kurzlink/"]
        .iter()
        .map(|dir| Path::new(dir).join(&query))
        .find(|path| path.exists())
}
