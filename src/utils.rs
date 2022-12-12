use anyhow::Context;
use core::fmt;
use reqwest::StatusCode;
use serde_yaml::Value;
use std::{collections::HashSet, error::Error, fs, path::Path, time::Duration};

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

pub fn yaml_from_file(path: &Path) -> anyhow::Result<Value> {
    let yaml_as_str = &fs::read_to_string(path).with_context(|| {
        format!(
            "yaml with shotlinks not found at path : '{}'  ",
            path.to_str().unwrap()
        )
    })?;
    let result = serde_yaml::from_str(yaml_as_str)?;
    Ok(result)
}

pub fn check_url(url: &str, timeout: u64) -> anyhow::Result<()> {
    let client = reqwest::blocking::Client::new();
    match client.get(url).timeout(Duration::new(timeout, 0)).send() {
        Ok(result) => {
            if result.status().is_success() {
                Ok(())
            } else {
                Err(ValidationError::HttpStatusError {
                    url: result.url().to_string(),
                    status: result.status(),
                }
                .into())
            }
        }
        Err(err) => Err(err.into()),
    }
}

pub fn check_urls(urls: &Vec<&str>, timeout: u64) -> anyhow::Result<()> {
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
