use crate::{
    config::{shortlink::Shortlink, tag::Tag},
    utils::{check_urls, yaml_from_file},
};

use std::{path::Path, time::Duration};

mod shortlink;
mod tag;

#[derive(Debug)]
pub struct Config {
    pub shortlinks: Vec<Shortlink>,
    pub tags: Vec<Tag>,
    pub timeout: Duration,
}

impl Config {
    pub fn new(path: &Path) -> Self {
        let yaml = yaml_from_file(path).unwrap();

        Config {
            shortlinks: yaml["shortlinks"]
                .as_vec()
                .unwrap()
                .iter()
                .map(|v| v.into())
                .collect(),
            tags: yaml["tags"]
                .as_vec()
                .unwrap()
                .iter()
                .map(|v| v.into())
                .collect(),
            timeout: Duration::from_millis(
                yaml["config"]["network"]["timeout"]
                    .as_i64()
                    .unwrap()
                    .try_into()
                    .unwrap(),
            ),
        }
    }

    pub fn check_links(&self) {
        let links = self
            .shortlinks
            .iter()
            .filter(|v| v.check_connection)
            .map(|v| v.destination.as_str())
            .collect::<Vec<&str>>();

        if let Err(error) = check_urls(&links, self.timeout) {
            println!("Some response failed with (1): {}", error);
        }
    }
}
