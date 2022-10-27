#[derive(Debug)]
pub struct Tag {
    pub description: String,
    pub image: String,
}

impl From<&serde_yaml::Value> for Tag {
    fn from(yaml: &serde_yaml::Value) -> Self {
        Tag {
            description: yaml["description"].as_str().unwrap().to_string(),
            image: yaml["image"].as_str().unwrap().to_string(),
        }
    }
}
