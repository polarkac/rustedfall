use crate::{
    objects::Card,
    requests::make_request,
    Result,
};

pub fn named_fuzzy(name: &str, set: Option<&str>) -> Result<Card> {
    let mut query = vec![("fuzzy", name)];
    if let Some(set) = set {
        query.push(("set", set));
    }
    let value = make_request("/cards/named", &query)?;

    Ok(serde_json::from_value(value)?)
}

pub fn named_exact(name: &str, set: Option<&str>) -> Result<Card> {
    let mut query = vec![("exact", name)];
    if let Some(set) = set {
        query.push(("set", set));
    }
    let value = make_request("/cards/named", &query)?;

    Ok(serde_json::from_value(value)?)
}
