use crate::{utils::*, Error::WrmError, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub const TRASH: &str = "~/.wrm/trash";
pub const FILES_IN_TRASH: &str = "~/.wrm/files_in_trash.json";

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct File {
    path: String,
    trash: String,
}

impl File {
    /// Constracts new File
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut trash = PathBuf::from(TRASH);
        let path = absolutize(path)?;
        let file_name = file_name(&path).unwrap();
        trash.set_file_name(file_name);
        let file = Self {
            path,
            trash: trash.to_string_lossy().to_string(),
        };
        Ok(file)
    }

    /// Returns path
    pub fn path(&self) -> &String {
        &self.path
    }

    /// Returns trash
    pub fn trash(&self) -> &String {
        &self.trash
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilesInTrash {
    files_in_trash: Vec<File>,
}

impl FilesInTrash {
    /// Constracts new FilesInTrash
    pub fn new(files_in_trash: Vec<File>) -> Self {
        Self { files_in_trash }
    }

    /// Returns files_in_trash
    pub fn files_in_trash(&self) -> &Vec<File> {
        &self.files_in_trash
    }

    /// Read FilesInTrash from a file
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self> {
        let f = fs::File::open(path)
            .map_err(|e| e.into())
            .map_err(WrmError)?;
        let files_in_trash = serde_json::from_reader(f)
            .map_err(|e| e.into())
            .map_err(WrmError)?;
        Ok(files_in_trash)
    }

    /// Write  FilesInTrash to a file
    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let f = fs::File::create(path)
            .map_err(|e| e.into())
            .map_err(WrmError)?;
        serde_json::to_writer_pretty(f, &self)
            .map_err(|e| e.into())
            .map_err(WrmError)?;
        Ok(())
    }

    /// Removes an element
    pub fn remove(&mut self, file: &File) -> &mut Self {
        self.files_in_trash.retain(|f| f != file);
        self
    }

    /// Adds an element
    pub fn add(&mut self, file: File) -> &mut Self {
        self.files_in_trash.push(file);
        self
    }
}
