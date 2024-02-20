mod files_in_trash;
mod initialize;
mod parse_arguments;
mod subcommands;
#[cfg(test)]
mod test;
pub mod utils;

pub use crate::{
    files_in_trash::{File, FilesInTrash, FILES_IN_TRASH, TRASH, WRM_PATH},
    parse_arguments::Options,
};

use initialize::initialize;
use parse_arguments::parse_arguments;
use std::process::exit;
use utils::eprintln;

fn main() {
    if let Err(e) = initialize() {
        eprintln(e);

        exit(1);
    }

    if let Err(e) = parse_arguments() {
        eprintln(e);

        exit(1);
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum Error {
    WrmError(anyhow::Error),
    #[error("'{}' no such file or directory", path)]
    NotFound {
        path: String,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
