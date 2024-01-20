mod actions;
mod argparse;

use crate::argparse::argparse;
use std::process::exit;

fn main() {
    if let Err(e) = argparse() {
        eprintln!("lamb: {}", e);
        exit(1)
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum Error {
    LambError(anyhow::Error),
    #[error("'{}' is not a directory", path)]
    NotADirectory {
        path: String,
    },
    #[error("Canceled")]
    Canceled,
}

pub type Result<T> = std::result::Result<T, Error>;
