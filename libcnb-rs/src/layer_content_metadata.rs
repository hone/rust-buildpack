use serde::Serialize;
use toml::value::Table;

#[derive(Serialize)]
pub struct Layer {
    pub launch: bool,
    pub build: bool,
    pub cache: bool,
    pub metadata: Table,
}
