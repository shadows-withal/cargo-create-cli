pub mod errors;
pub mod generate;
pub mod templating;

use crate::errors::Result;
use crate::generate::Generate;
use camino::Utf8PathBuf;
use clap::Parser;

/// A CLI to scaffold out new Rust-based CLI tools.
#[derive(Parser, Debug)]
#[command(author, version)]
pub struct Args {
    #[arg(short, long)]
    path: Option<Utf8PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    tracing_subscriber::fmt::init();

    let mut gen = Generate::new_with_args(args)?;
    gen.do_dialogue()?;
    gen.write_to_disk()?;

    Ok(())
}
