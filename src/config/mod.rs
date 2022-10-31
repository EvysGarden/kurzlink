use std::collections::HashMap;

use crate::{
    config::{shortlink::Shortlink, tag::Tag},
    error::ValidationError,
    utils::{check_urls, find_duplicates, yaml_from_file, BoxError},
};

use std::{path::Path, time::Duration};

pub(crate) mod shortlink;
mod tag;

#[derive(Debug)]
pub struct Config {
    pub shortlinks: Vec<Shortlink>,
    pub tags: HashMap<String, Tag>,
    pub timeout: Duration,
}

impl Config {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, BoxError> {
        let yaml_result = yaml_from_file(path.as_ref());
        if let Err(err) = yaml_result {
            return Err(err);
        }

        let yaml = yaml_result.unwrap();
        
        Ok(Config {
            shortlinks: yaml["shortlinks"]
                .as_sequence()
                .unwrap()
                .iter()
                .map(|v| v.into())
                .collect(),
            tags: yaml["tags"]
                .as_mapping()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.as_str().unwrap().to_string(), v.into()))
                .collect(),
            timeout: Duration::from_millis(
                yaml["config"]["network"]["timeout"]
                    .as_i64()
                    .unwrap()
                    .try_into()
                    .unwrap(),
            ),
        })
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Err(err) = self.check_duplicates() {
            return Err(err);
        }
        if let Err(err) = self.check_links() {
            return Err(err);
        }
        Ok(())
    }

    fn check_duplicates(&self) -> Result<(), ValidationError> {
        if let Some(duplicates) =
            find_duplicates(self.shortlinks.iter().map(|v| &v.sources).flatten())
        {
            return Err(ValidationError::DuplicateSources(duplicates));
        }

        if let Some(duplicates) = find_duplicates(self.shortlinks.iter().map(|v| &v.destination)) {
            return Err(ValidationError::DuplicateDestinations(duplicates));
        }

        Ok(())
    }

    fn check_links(&self) -> Result<(), ValidationError> {
        let links = self
            .shortlinks
            .iter()
            .filter(|v| v.check_connection)
            .map(|v| v.destination.as_str())
            .collect::<Vec<&str>>();

        if let Err(error) = check_urls(&links, self.timeout) {
            Err(ValidationError::NetworkError(error))
        } else {
            Ok(())
        }
    }
}
