use serde::{Deserialize, Serialize};

use super::url::{AbsoluteUrl, RelativeUrl};

#[derive(Serialize, Deserialize, Debug)]
pub struct Shortlink {
    pub sources: Vec<RelativeUrl>,
    pub destination: AbsoluteUrl,
    pub tags: Option<Vec<String>>,
    pub check: Option<bool>,
}
