use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Crate "id", to be serialized into Base64. For now, just the crate name.
pub type Id = String;

pub struct Libraries {
    pub list: HashMap<Id, Library>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Library {
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub axo_use: bool,
    /// Can we use this library with create-cli?
    #[serde(default)]
    pub create_cli: bool,
    pub category: Category,
}

/// Dummy type to match the TOML schema.
#[derive(Deserialize, Debug)]
struct TomlSchema {
    pub libraries: Vec<Library>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialOrd, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    /// Argument parsing
    ArgParsing,
    /// Command generation
    CommandGen,
    /// Formatting
    Formatting,
    /// Structured output
    StructOutput,
    /// Errors, logging, diagnostics
    ErrLogDiag,
    /// Interactivity
    Interactivity,
    /// Testing
    Testing,
    /// Other
    Other,
}

impl Libraries {
    pub fn new() -> Self {
        let toml_file = include_str!("../libs.toml");
        let parsed: TomlSchema = toml::from_str(toml_file).unwrap();
        let list: HashMap<Id, Library> = parsed
            .libraries
            .iter()
            .cloned()
            .map(|lib| (lib.name.clone(), lib))
            .collect();
        Self { list }
    }

    pub fn get_for_category(&self, category: Category) -> HashMap<Id, Library> {
        self.list
            .iter()
            .filter(|(_, lib)| category == lib.category)
            .map(|(id, lib)| (id.clone(), lib.clone()))
            .collect()
    }
}

impl Default for Libraries {
    fn default() -> Self {
        Self::new()
    }
}
