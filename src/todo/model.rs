use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CreateEntryData {
    pub title: String,
}

#[derive(Deserialize, Clone)]
pub struct UpdateEntryData {
    pub title: Option<String>,
}