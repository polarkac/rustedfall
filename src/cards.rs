use crate::{
    objects::{Card, List},
    requests::make_request,
    commands::{SearchArgs, NamedArgs},
    Result,
};

pub fn fuzzy(args: NamedArgs) -> Result<Card> {
    let mut query = vec![("fuzzy", args.name.as_str())];
    if let Some(set) = &args.set {
        query.push(("set", set));
    }
    let value = make_request("/cards/named", &query)?;

    Ok(serde_json::from_value(value)?)
}

pub fn exact(args: NamedArgs) -> Result<Card> {
    let mut query = vec![("exact", args.name.as_str())];
    if let Some(set) = &args.set {
        query.push(("set", set));
    }
    let value = make_request("/cards/named", &query)?;

    Ok(serde_json::from_value(value)?)
}

pub fn search(args: SearchArgs) -> Result<List<Card>> {
    let query = vec![
        ("q", args.q.as_str()),
        ("order", args.order.as_str()),
        ("dir", args.direction.as_str()),
    ];
    let value = make_request("/cards/search", &query)?;

    Ok(serde_json::from_value(value)?)
}
