use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Network {
    pub timeout: u64,
    pub check: bool,
    pub ogp: bool,
}
