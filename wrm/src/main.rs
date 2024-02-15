mod actions;
mod files_in_trash;
mod parse_arguments;
pub mod utils;

pub use crate::{
    files_in_trash::{FilesInTrash, FILES_IN_TRASH, TRASH},
    parse_arguments::Options,
};

use filey::Filey;
use parse_arguments::parse_arguments;
use std::process::exit;

fn main() {
    if let Err(e) = init() {
        eprintln!("error: {}", e);
        exit(1);
    }

    if let Err(e) = parse_arguments() {
        eprintln!("error: {}", e);
        exit(1);
    }
}

fn init() -> filey::Result<()> {
    Filey::new(TRASH)
        .expand_user()?
        .absolutize()?
        .create_dir()?;

    let files_in_trash = Filey::new(FILES_IN_TRASH)
        .expand_user()?
        .absolutize()?
        .to_string();

    FilesInTrash::new(vec![])
        .write(files_in_trash)
        .map_err(|e| e.into())
        .map_err(filey::Error::FileyError)?;
    Ok(())
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum Error {
    WrmError(anyhow::Error),
    #[error("Incorrect arguments")]
    IncorrectArguments,
    #[error("No such file or directory (os error 2)")]
    NotFound,
}

pub type Result<T> = std::result::Result<T, Error>;
