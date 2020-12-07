use serde::Serialize;
use toml::value::Table;

/// Build Plan (TOML)
#[derive(Serialize)]
pub struct BuildPlan {
    pub provides: Vec<Provide>,
    pub requires: Vec<Require>,
    pub or: Vec<Or>,
}

impl BuildPlan {
    pub fn new() -> Self {
        BuildPlan {
            provides: Vec::new(),
            requires: Vec::new(),
            or: Vec::new(),
        }
    }
}

#[derive(Serialize)]
pub struct Provide {
    name: String,
}

impl Provide {
    pub fn new(name: impl Into<String>) -> Self {
        Provide { name: name.into() }
    }
}

#[derive(Serialize)]
pub struct Require {
    name: String,
    metadata: Table,
}

impl Require {
    pub fn new(name: impl Into<String>) -> Self {
        Require {
            name: name.into(),
            metadata: Table::new(),
        }
    }
}

#[derive(Serialize)]
pub struct Or {
    pub provides: Vec<Provide>,
    pub requires: Vec<Require>,
}
