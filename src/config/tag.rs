use yaml_rust::Yaml;

#[derive(Debug)]
pub struct Tag {
    pub name: String,
    pub description: String,
    pub image: String,
}

impl From<&Yaml> for Tag {
    fn from(yaml: &Yaml) -> Self {
        Tag {
            name: yaml["name"].as_str().unwrap().to_owned(),
            description: yaml["description"].as_str().unwrap().to_owned(),
            image: yaml["image"].as_str().unwrap().to_owned(),
        }
    }
}
