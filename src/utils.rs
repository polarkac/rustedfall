use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct UUID(pub String);
