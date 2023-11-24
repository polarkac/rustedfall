use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Card {
    id: String,
    name: String,
    type_line: String,
    lang: String,
    oracle_text: Option<String>,
    layout: String,
    scryfall_uri: String,
    flavor_text: Option<String>,
    card_faces: Option<Vec<CardFace>>,
}

impl Card {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn oracle_text(&self) -> Option<&String> {
        self.oracle_text.as_ref()
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "= {} =\n", self.name)?;
        write!(f, "- {} -\n", self.type_line)?;
        if let Some(oracle) = &self.oracle_text {
            write!(f, "{}", oracle)?;
        }
        if let Some(flavor) = &self.flavor_text {
            write!(f, "\n\"{}\"", flavor)?;
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct CardFace {
    name: String,
    type_line: String,
    oracle_text: Option<String>,
    flavor_text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RelatedCard {}

#[derive(Serialize, Deserialize)]
pub struct Ruling {}

#[derive(Serialize, Deserialize)]
pub struct CardSymbol {}

#[derive(Deserialize, Serialize)]
pub struct List {
    next_page: String,
    pub total_cards: u32,
    warnings: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Set {}

#[derive(Serialize, Deserialize)]
pub struct Catalog {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    status: u16,
    code: String,
    details: String,
    #[serde(rename = "type", default)]
    kind: Option<String>, // type in Scryfall
    warnings: Option<Vec<String>>,
}

impl Error {
    pub fn new(status: u16, code: &str, details: &str) -> Self {
        Self {
            status,
            code: code.to_string(),
            details: details.to_string(),
            kind: None,
            warnings: None,
        }
    }

    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn details(&self) -> &str {
        &self.details
    }

    pub fn kind(&self) -> Option<&String> {
        self.kind.as_ref()
    }

    pub fn warnings(&self) -> Option<&Vec<String>> {
        self.warnings.as_ref()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Scryfall error: {}", self.details())
    }
}

impl std::error::Error for Error {}
