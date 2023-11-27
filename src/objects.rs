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
    all_parts: Option<Vec<RelatedCard>>,
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
        if let Some(card_faces) = &self.card_faces {
            card_faces.iter().for_each(|face| write!(f, "{}\n", face).unwrap());
        } else {
            write!(f, "- {} -\n", self.type_line)?;
            if let Some(oracle) = &self.oracle_text {
                write!(f, "{}", oracle)?;
            }
            if let Some(flavor) = &self.flavor_text {
                write!(f, "\n\"{}\"", flavor)?;
            }
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

impl fmt::Display for CardFace {
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
pub struct RelatedCard {
    id: String,
    component: String,
    name: String,
    type_line: String,
    uri: String,
}

impl fmt::Display for RelatedCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Related: {}", self.uri)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Ruling {
    oracle_id: String,
    source: String,
    published_at: String,
    commend: String,
}

#[derive(Serialize, Deserialize)]
pub struct CardSymbol {}

#[derive(Serialize, Deserialize)]
pub struct List<T> {
    data: Vec<T>,
    has_more: bool,
    next_page: Option<String>,
    total_cards: Option<u32>,
    warnings: Option<Vec<String>>,
}

impl<T> List<T> {
    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn total_cards(&self) -> Option<u32> {
        self.total_cards
    }
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
