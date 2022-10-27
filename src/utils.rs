use core::fmt;
use reqwest::StatusCode;
use std::{collections::HashSet, error::Error, fs, path::Path, time::Duration};

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

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

pub fn yaml_from_file(path: &Path) -> Result<serde_yaml::Value, BoxError> {
    Ok(serde_yaml::from_str(&fs::read_to_string(path)?)?)
}

pub fn check_url(url: &str, timeout: Duration) -> Result<(), BoxError> {
    let client = reqwest::blocking::Client::new();
    match client.get(url).timeout(timeout).send() {
        Ok(result) => {
            if result.status().is_success() {
                Ok(())
            } else {
                Err(Box::new(HttpStatusError {
                    url: result.url().to_string(),
                    status: result.status(),
                }))
            }
        }
        Err(err) => Err(Box::new(err)),
    }
}

pub fn check_urls(urls: &Vec<&str>, timeout: Duration) -> Result<(), BoxError> {
    for url in urls {
        if let Err(err) = check_url(url, timeout) {
            return Err(err);
        }
    }
    Ok(())
}

pub fn find_duplicates<'a, I, T>(iter: I) -> Option<HashSet<&'a T>>
where
    I: Iterator<Item = &'a T>,
    T: 'a + std::hash::Hash + std::cmp::Eq,
{
    let mut set = HashSet::new();
    let mut duplicates = HashSet::new();
    iter.for_each(|v| {
        if !set.insert(v) {
            duplicates.insert(v);
        }
    });

    if duplicates.is_empty() {
        None
    } else {
        return Some(duplicates);
    }
}
