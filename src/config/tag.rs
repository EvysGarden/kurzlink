use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub description: String,
    pub image: Option<String>,
}
