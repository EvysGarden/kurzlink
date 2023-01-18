use serde::{Deserialize, Serialize};
use url::ParseError;

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Debug)]
#[serde(try_from = "String")]
pub struct AbsoluteUrl(String);

impl AbsoluteUrl {
    pub fn try_new(url: String) -> Result<Self, ParseError> {
        Ok(Self(reqwest::Url::parse(&url)?.to_string()))
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
        reqwest::Url::parse("https://www.example.com/")
            .unwrap()
            .join(&url)?;

        Ok(Self(url))
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
