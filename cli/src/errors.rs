use miette::Diagnostic;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, CreateCliError>;

#[derive(Diagnostic, Error, Debug)]
pub enum CreateCliError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Minijinja(#[from] minijinja::Error),
}
