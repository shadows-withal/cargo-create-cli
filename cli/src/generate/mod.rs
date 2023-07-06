use crate::errors::Result;
use crate::templating::Templating;
use crate::Args;
use camino::Utf8PathBuf;
use cargo_create_cli_libraries::*;
use clap::arg;
use dialoguer::console::Style;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, FuzzySelect, Input, MultiSelect, Select};
use serde::Serialize;
use std::collections::HashMap;
use std::ops::Index;
use std::str::FromStr;

/// Meta-struct to hold information learned during either the dialogue, from an info string,
/// or from command-line arguments. This is passed to the templating system.
#[derive(Default, Serialize)]
pub struct Metadata {
    /// Name of your project
    pub name: String,
    /// Description of the project
    pub description: String,
    /// Do we want to use defaults?
    pub use_defaults: bool,
    /// Library for argparsing
    pub lib_argparsing: Option<Library>,
}

pub struct Generate<'a> {
    pub metadata: Metadata,
    pub libraries: Libraries,
    pub templating: Templating<'a>,
    pub base_path: Option<Utf8PathBuf>,
}

impl<'a> Generate<'a> {
    pub fn new_with_args(args: Args) -> Result<Self> {
        let metadata = Metadata::default();
        let libraries = Libraries::new();
        let templating = Templating::new()?;

        Ok(Self {
            metadata,
            libraries,
            templating,
            base_path: args.path,
        })
    }

    pub fn do_dialogue(&mut self) -> Result<()> {
        let theme = ColorfulTheme {
            checked_item_prefix: console::style("  [x]".to_string()).for_stderr().green(),
            unchecked_item_prefix: console::style("  [ ]".to_string()).for_stderr().dim(),
            active_item_style: console::Style::new().for_stderr().cyan().bold(),
            ..ColorfulTheme::default()
        };
        tracing::info!("Creating a new Rust CLI project!");
        self.metadata.name = Input::with_theme(&theme)
            .with_prompt("Let's get started: What do you want your project to be called?")
            .interact_text()?;
        self.metadata.description = Input::with_theme(&theme)
            .with_prompt("Enter a short description of your project:")
            .interact_text()?;
        self.metadata.use_defaults = Confirm::with_theme(&theme)
            .with_prompt("Do you want to use the default libraries recommended by Axo?")
            .default(false)
            .interact()?;

        self.metadata.lib_argparsing =
            Some(self.libs_single_select(Category::ArgParsing, "Argument Parsing")?);

        self.base_path = match &self.base_path {
            Some(p) => Some(p.clone()),
            None => Some(Utf8PathBuf::from_str(&format!("./{}", self.metadata.name)).unwrap()),
        };

        Ok(())
    }

    pub fn libs_single_select(&self, category: Category, name: &str) -> Result<Library> {
        // FIXME: Deduplicate this
        let theme = ColorfulTheme {
            checked_item_prefix: console::style("  [x]".to_string()).for_stderr().green(),
            unchecked_item_prefix: console::style("  [ ]".to_string()).for_stderr().dim(),
            active_item_style: console::Style::new().for_stderr().cyan().bold(),
            ..ColorfulTheme::default()
        };
        let all_libs: HashMap<Id, Library> = self
            .libraries
            .get_for_category(category)
            .iter()
            .filter(|(_, lib)| lib.create_cli)
            .map(|(id, lib)| (id.clone(), lib.clone()))
            .collect();
        let all_keys: Vec<&Id> = all_libs.iter().map(|(id, _)| id).collect();
        let idx = Select::with_theme(&theme)
            .with_prompt(format!("Select a library for {name}"))
            .items(&all_keys)
            .interact()?;
        let lib = all_libs
            .iter()
            .find(|(id, _)| id == &all_keys[idx])
            .expect("Error while selecting library.");

        Ok(lib.1.clone())
    }

    pub fn write_to_disk(&self) -> Result<()> {
        let base_path = self.base_path.as_ref().expect("Base path should be set!");
        self.templating
            .render_and_write("Cargo.toml", &self.metadata, base_path)?;
        self.templating
            .render_and_write("main.rs", &self.metadata, base_path)?;

        Ok(())
    }
}
