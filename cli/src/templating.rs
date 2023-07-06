use crate::errors::Result;
use crate::generate::Metadata;
use camino::Utf8PathBuf;
use minijinja::Environment;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

const CARGO_TOML_TPL: &str = include_str!("../templates/Cargo.toml.j2");
const CARGO_TOML_PATH: &str = "Cargo.toml";

const MAIN_RS_TPL: &str = include_str!("../templates/main.rs.j2");
const MAIN_RS_PATH: &str = "src/main.rs";

pub struct Templating<'a> {
    pub environment: Environment<'a>,
    pub paths: HashMap<String, &'static str>,
}

impl<'a> Templating<'a> {
    pub fn new() -> Result<Self> {
        let mut env = Environment::new();
        env.add_template("Cargo.toml", CARGO_TOML_TPL)?;
        env.add_template("main.rs", MAIN_RS_TPL)?;

        let mut paths = HashMap::new();
        paths.insert("Cargo.toml".to_string(), CARGO_TOML_PATH);
        paths.insert("main.rs".to_string(), MAIN_RS_PATH);

        Ok(Self {
            environment: env,
            paths,
        })
    }

    pub fn render(&self, name: &str, context: &Metadata) -> Result<String> {
        let template = self.environment.get_template(name)?;
        Ok(template.render(context)?)
    }

    pub fn render_and_write(
        &self,
        name: &str,
        context: &Metadata,
        base_path: &Utf8PathBuf,
    ) -> Result<()> {
        let rendered = self.render(name, context)?;
        let path = self
            .paths
            .get(name)
            .expect("Invalid template name supplied!");
        let mut dest_path = base_path.clone();
        dest_path.push(path);
        fs::create_dir_all(dest_path.parent().unwrap())?;
        let mut file = fs::File::create(dest_path)?;
        write!(file, "{rendered}")?;
        Ok(())
    }
}
