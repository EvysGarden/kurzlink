use serde::{Deserialize, Serialize};
use url::ParseError;

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Debug)]
#[serde(try_from = "String")]
pub struct AbsoluteUrl(String);

impl AbsoluteUrl {
    pub fn try_new(url: String) -> Result<Self, ParseError> {
        if let Err(err) = reqwest::Url::parse(&url) {
            Err(err)
        } else {
            Ok(Self(url))
        }
    }

    #[inline]
    pub fn inner(&self) -> &String {
        &self.0
    }
}

impl TryFrom<String> for AbsoluteUrl {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Debug)]
#[serde(try_from = "String")]
pub struct RelativeUrl(String);

impl RelativeUrl {
    pub fn try_new(url: String) -> Result<Self, ParseError> {
        let validate_base_url: reqwest::Url =
            reqwest::Url::parse("https://www.example.com/").unwrap();

        if let Err(err) = validate_base_url.join(&url) {
            Err(err)
        } else {
            Ok(Self(url))
        }
    }

    #[inline]
    pub fn inner(&self) -> &String {
        &self.0
    }
}

impl TryFrom<String> for RelativeUrl {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}
