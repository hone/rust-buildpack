use serde::Serialize;

/// Build Plan (TOML)
#[derive(Serialize)]
pub struct BuildPlan {
    pub provides: Vec<Provide>,
    pub requires: Vec<Require>,
}

impl BuildPlan {
    pub fn new() -> Self {
        BuildPlan {
            provides: Vec::new(),
            requires: Vec::new(),
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
    metadata: toml::value::Table,
}

#[derive(Serialize)]
pub struct Or {
    pub provides: Vec<Provide>,
    pub requires: Vec<Require>,
}
