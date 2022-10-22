use core::fmt;
use std::{error::Error, fs, path::Path, time::Duration};
use yaml_rust::{Yaml, YamlLoader};
use reqwest::StatusCode;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug)]
pub struct HttpStatusError{
    pub url: String,
    pub status: StatusCode
}

impl fmt::Display for HttpStatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Got status code {} for {}", self.status, self.url)
    }
}

impl Error for HttpStatusError {}

pub fn yaml_from_file(path: &Path) -> Result<Yaml, BoxError> {
    let config_str = fs::read_to_string(path)?;
    let config_yml = YamlLoader::load_from_str(config_str.as_str())?;

    Ok(config_yml[0].clone())
}

pub fn check_url(url: &str, timeout: Duration) -> Result<(), BoxError> {
    let client = reqwest::blocking::Client::new();
    match client.get(url).timeout(timeout).send() {
        Ok(result) => {
            if result.status().is_success() {
                Ok(())
            } else {
                Err(Box::new(HttpStatusError{
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
