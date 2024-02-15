// wrm
//
// wrm is a file deletion utility.
mod actions;
mod argparse;
mod file_list;
mod test;

use crate::{argparse::argparse, file_list::FileList};
use filey::{self, create_dir, Error::FileyError, Filey};
use std::process::exit;

fn main() {
    match Filey::new("~/.config/wrm").expand_user() {
        Ok(wrm_config_dir) => {
            let wrm_config_dir = wrm_config_dir.to_string();
            if let Err(e) = prepare(&wrm_config_dir) {
                eprintln!("error: {}", e);
                exit(1);
            }
            if let Err(e) = argparse(&wrm_config_dir) {
                eprintln!("error: {}", e);
                exit(1);
            }
        }
        Err(e) => {
            eprintln!("error: {}", e);
            exit(1);
        }
    }
}

// Create $HOME/.config/wrm, $HOME/.config/wrm/trash and $HOME/.config/wrm/list.json.
fn prepare(wrm_config_dir: &String) -> filey::Result<()> {
    create_dir!(
        wrm_config_dir,
        format!("{}/trash", wrm_config_dir)
    );
    let file_list = Filey::new(format!("{}/list.json", wrm_config_dir));
    if !file_list.exists() {
        file_list.create_file()?;
        FileList::new()
            .write(file_list.to_string())
            .map_err(|e| e.into())
            .map_err(FileyError)?;
    }
    Ok(())
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum Error {
    WrmError(anyhow::Error),
    #[error("'{}' No such file or directory", path)]
    NotFoundError {
        path: String,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
