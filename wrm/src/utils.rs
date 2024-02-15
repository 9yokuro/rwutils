use crate::{Error::WrmError, Result};
use filey::Filey;
use inquire::Confirm;
use std::{fmt::Display, path::Path};

pub fn absolutize<P: AsRef<Path>>(path: P) -> Result<String> {
    let absolutized = Filey::new(path)
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
