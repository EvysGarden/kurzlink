use std::time::Duration;

use minijinja::{context, Template};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

use crate::error::ValidationError;

use super::{
    network::Network,
    url::{AbsoluteUrl, RelativeUrl},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Shortlink {
    pub sources: Vec<RelativeUrl>,
    pub destination: AbsoluteUrl,
    pub tags: Option<Vec<String>>,
    pub check: Option<bool>,
    pub description: Option<String>,
}

impl Shortlink {
    pub fn checked_html(&self, template: Template, network_config: &Network) -> anyhow::Result<String> {
        let mut meta = String::new();
        if self.check.unwrap_or(network_config.check) {
            let result = reqwest::blocking::Client::new()
                .get(self.destination.inner())
                .timeout(Duration::new(network_config.timeout, 0))
                .send()?;

            if !result.status().is_success() {
                return Err(ValidationError::HttpStatusError {
                    url: self.destination.inner().clone(),
                    status: result.status(),
                }
                .into());
            }

            if network_config.ogp {
                let dom = Html::parse_document(&result.text()?);
                let selector = Selector::parse(
                    r#"meta[property^="og:"]"#,
                ).unwrap();
                meta = dom
                    .select(&selector)
                    .map(|element| element.html() + "\n")
                    .collect::<String>();
                println!("meta: {meta}");
            }
        }

        let rendered =
            template.render(context!(redirect_uri => self.destination, ogp_meta => meta))?;

        Ok(rendered)
    }
}
