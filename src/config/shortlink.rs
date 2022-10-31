use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Shortlink {
    pub sources: Vec<String>,
    pub destination: String,
    pub tags: Vec<String>,
    pub check_connection: bool,
}

impl From<&serde_yaml::Value> for Shortlink {
    fn from(yaml: &serde_yaml::Value) -> Self {
        Shortlink {
            sources: yaml["sources"]
                .as_sequence()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap().to_string())
                .collect(),
            destination: yaml["destination"].as_str().unwrap().to_string(),
            tags: yaml["tags"]
                .as_sequence()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap().to_string())
                .collect(),
            check_connection: yaml["check"].as_bool().unwrap_or(true),
        }
    }
}
