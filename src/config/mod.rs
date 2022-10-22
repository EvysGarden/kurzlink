use crate::{
    config::{shortlink::Shortlink, tag::Tag},
    utils::{yaml_from_file, check_urls},
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

    pub fn check_links(& self) {
        let mut links: Vec<&str> = Vec::new();
        // get all dst links
        for sl in &self.shortlinks {
            links.push(&sl.destination);
        }

        let _ = check_urls(links, self.timeout);
    }
}
