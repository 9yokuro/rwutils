use crate::{Error::WrmError, Result};
use colored::Colorize;
use filey::Filey;
use inquire::Confirm;
use std::{error::Error, fmt::Display, path::Path};

pub fn absolutize<P: AsRef<Path>>(path: P) -> Result<String> {
    let absolutized = Filey::new(path)
        .expand_user()
        .map_err(|e| e.into())
        .map_err(WrmError)?
        .absolutize()
        .map_err(|e| e.into())
        .map_err(WrmError)?
        .to_string();

    Ok(absolutized)
}

pub fn file_name<P: AsRef<Path>>(path: P) -> Option<String> {
    let file_name = Filey::new(path).file_name()?;

    Some(file_name)
}

pub fn remove<P: AsRef<Path>>(path: P) -> Result<()> {
    Filey::new(path)
        .remove()
        .map_err(|e| e.into())
        .map_err(WrmError)?;

    Ok(())
}

pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()> {
    Filey::new(from)
        .move_to(to)
        .map_err(|e| e.into())
        .map_err(WrmError)?;

    Ok(())
}

pub fn confirm<D: Display>(message: D) -> Result<bool> {
    Confirm::new(message.to_string().as_str())
        .with_default(false)
        .prompt()
        .map_err(|e| e.into())
        .map_err(WrmError)
}

pub fn asref_path_to_string<P: AsRef<Path>>(path: P) -> String {
    path.as_ref().to_string_lossy().to_string()
}

pub fn eprintln<E: Error>(error: E) {
    eprintln!("{}: {}", "error".red().bold(), error);
}

#[macro_export]
macro_rules! exists_or_else {
    ( $path:expr ) => {
        if !Filey::new($path).exists() {
            let not_found = NotFound {
                path: $path.to_string(),
            };

            eprintln(not_found);

            continue;
        }
    };
}
