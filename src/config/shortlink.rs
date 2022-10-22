use yaml_rust::Yaml;

#[derive(Debug)]
pub struct Shortlink {
    pub sources: Vec<String>,
    pub destination: String,
    pub tags: Vec<String>,
    pub check_connection: bool,
}

impl From<&Yaml> for Shortlink {
    fn from(yaml: &Yaml) -> Self {
        Shortlink {
            sources: yaml["sources"]
                .as_vec()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap().to_owned())
                .collect(),
            destination: yaml["destination"].as_str().unwrap().to_owned(),
            tags: yaml["tags"]
                .as_vec()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap().to_owned())
                .collect(),
            check_connection: yaml["check"].as_bool().unwrap_or(true),
        }
    }
}
