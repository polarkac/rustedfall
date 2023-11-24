use serde_json::Value;
use reqwest::blocking::Client;

use crate::{
    BASE_API_URL,
    objects::Error,
    Result,
};

pub fn make_request(endpoint: &str, query: &[(&str, &str)]) -> Result<Value> {
    let client = Client::new();
    let response = client.get(format!("{}{}", BASE_API_URL, endpoint))
        .query(query)
        .send()?;
    let value: Value = response.json()?;
    let object_name = match value.get("object") {
        Some(name) => name,
        None => {
            return Err(Box::from("Object type field missing."));
        }
    };
    if object_name == "error" {
        let scryfall_error: Error = serde_json::from_value(value)?;

        return Err(Box::new(scryfall_error));
    }

    Ok(value)
}
