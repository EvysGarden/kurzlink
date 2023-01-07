use anyhow::Context;
use std::{collections::HashMap, fs, path::Path};

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    config::{
        network::Network,
        shortlink::Shortlink,
        tag::Tag,
        templating::{render_redirect_html, write_html},
    },
    error::ValidationError,
    utils::{check_urls, find_duplicates, yaml_from_file},
};

mod network;
mod shortlink;
mod tag;
mod templating;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub shortlinks: Vec<Shortlink>,
    pub tags: HashMap<String, Tag>,
    pub network: Network,
    pub index: Option<String>,
}

impl Config {
    pub fn new(config_path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let config_yaml = yaml_from_file(config_path.as_ref())?;

        Ok(serde_yaml::from_value(config_yaml)?)
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        self.check_duplicates()?;
        self.check_links()?;
        Ok(())
    }

    fn check_duplicates(&self) -> anyhow::Result<()> {
        if let Some(duplicates) = find_duplicates(self.shortlinks.iter().flat_map(|v| &v.sources)) {
            return Err(ValidationError::DuplicateSources(duplicates).into());
        }

        if let Some(duplicates) = find_duplicates(self.shortlinks.iter().map(|v| &v.destination)) {
            return Err(ValidationError::DuplicateDestinations(duplicates).into());
        }

        Ok(())
    }

    fn check_links(&self) -> anyhow::Result<()> {
        let links = self
            .shortlinks
            .iter()
            .filter(|v| v.check.unwrap_or(self.network.check))
            .map(|v| v.destination.as_str())
            .collect::<Vec<&str>>();

        check_urls(&links, self.network.timeout)
    }

    pub fn render_files(
        &self,
        output_path: impl AsRef<Path>,
        template_path: impl AsRef<Path>,
    ) -> anyhow::Result<()> {
        if !output_path.as_ref().exists() {
            fs::create_dir(&output_path).with_context(|| "Couldn't create output dir")?;
        }

        if let Some(index) = &self.index {
            let index_render = render_redirect_html(index, &template_path)?;
            write_html(&output_path, &index_render)?;
        }

        for shortlink in &self.shortlinks {
            for source in &shortlink.sources {
                let source_render = render_redirect_html(&shortlink.destination, &template_path)?;
                write_html(output_path.as_ref().join(source), &source_render)?;
            }
        }

        Ok(())
    }

    pub fn write_vanity(&self, vanity_path: impl AsRef<Path>) -> anyhow::Result<()> {
        let vanity = json!({
            "index": &self.index,
            "shortlinks": &self.shortlinks,
            "tags": &self.tags
        })
        .to_string();

        fs::write(vanity_path, vanity)?;

        Ok(())
    }
}
