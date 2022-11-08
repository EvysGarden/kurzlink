use std::{collections::HashMap, path::Path};

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    config::{network::Network, shortlink::Shortlink, tag::Tag},
    error::ValidationError,
    utils::{check_urls, find_duplicates, yaml_from_file, BoxError},
};

mod network;
mod shortlink;
mod tag;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub shortlinks: Vec<Shortlink>,
    pub tags: HashMap<String, Tag>,
    pub network: Network,
    pub index: Option<String>,
}

impl Config {
    pub fn new(config_path: impl AsRef<Path>) -> Result<Self, BoxError> {
        let config_yaml = yaml_from_file(config_path.as_ref());

        Ok(serde_yaml::from_value(config_yaml?).unwrap())
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        self.check_duplicates()?;
        self.check_links()?;
        Ok(())
    }

    fn check_duplicates(&self) -> Result<(), ValidationError> {
        if let Some(duplicates) = find_duplicates(self.shortlinks.iter().flat_map(|v| &v.sources)) {
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
            .filter(|v| v.check.unwrap_or(self.network.check))
            .map(|v| v.destination.as_str())
            .collect::<Vec<&str>>();

        if let Err(error) = check_urls(&links, self.network.timeout) {
            Err(ValidationError::NetworkError(error))
        } else {
            Ok(())
        }
    }

    pub fn generate_vanitymap(&self) -> serde_json::Value {
        json!({
            "index": &self.index,
            "shortlinks": &self.shortlinks,
            "tags": &self.tags
        })
    }
}
