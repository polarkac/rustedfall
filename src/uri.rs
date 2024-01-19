use std::marker::PhantomData;
use serde::{Deserialize, de::DeserializeOwned};

use crate::errors::Error;

#[derive(Deserialize, Debug)]
#[serde(transparent)]
pub struct URI<T> {
    uri: String,
    #[serde(skip)]
    _marker: PhantomData<T>,
}

impl<T> URI<T> where T: DeserializeOwned {
    pub fn new(uri: &str) -> Self {
        Self {
            uri: uri.to_string(),
            _marker: PhantomData,
        }
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }

    pub fn fetch(&self) -> Result<T, Error> {
        let response = reqwest::blocking::get(&self.uri)?;
        match response.status().as_u16() {
            400..=599 => Err(Error::ScryfallError(response.json()?)),
            _ => Ok(response.json()?),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::URI;
    use crate::errors::Error;
    use serde_json::Value;

    #[test]
    fn fetch_card_phantom_nishoba() {
        let uri: URI<Value> = URI::new("https://api.scryfall.com/cards/56ebc372-aabd-4174-a943-c7bf59e5028d");
        let result = uri.fetch();
        assert_eq!(result.is_ok(), true);
        let json = result.unwrap();
        assert_eq!(
            json.get("id"), Some(&Value::from("56ebc372-aabd-4174-a943-c7bf59e5028d"))
        );
    }

    #[test]
    fn fetch_non_existing_card() {
        let uri: URI<Value> = URI::new("https://api.scryfall.com/cards/6ebc372-aabd-4174-a943-c7bf59e5028d");
        let result = uri.fetch();
        assert_eq!(result.is_err(), true);
        match result {
            Err(Error::ScryfallError(_e)) => assert_eq!(true, true),
            _ => {assert_eq!(false, true)}
        }
    }
}
