use serde::{Deserialize, Serialize};
use toml::value::Table;

#[derive(Deserialize, Serialize)]
pub struct BuildpackPlan {
    pub entries: Vec<Entry>,
}

impl BuildpackPlan {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Entry {
    name: String,
    metadata: Table,
}
