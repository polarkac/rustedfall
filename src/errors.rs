use serde::Deserialize;

#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    ScryfallError(ScryfallError),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::ReqwestError(value)
    }
}

#[derive(Deserialize, Debug)]
pub struct ScryfallError {
    status: u32,
    code: String,
    details: String,
    #[serde(rename = "type")]
    kind: Option<String>,
    warnings: Option<Vec<String>>,
}
