mod objects;
mod requests;
pub mod cards;
pub mod commands;

use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const BASE_API_URL: &str = "https://api.scryfall.com";
