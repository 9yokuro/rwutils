mod actions;
mod argparse;

use argparse::argparse;
use std::process::exit;

fn main() {
    if let Err(e) = argparse() {
        eprintln!("error: {}", e);
        exit(1);
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum Error {
    WlsError(anyhow::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
