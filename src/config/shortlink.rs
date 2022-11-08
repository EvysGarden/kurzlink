use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Shortlink {
    pub sources: Vec<String>,
    pub destination: String,
    pub tags: Option<Vec<String>>,
    pub check: Option<bool>,
}
