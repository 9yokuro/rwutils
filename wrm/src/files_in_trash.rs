use crate::{utils::*, Error::WrmError, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub const WRM_PATH: &str = "~/.config/wrm";
pub const TRASH: &str = "~/.config/wrm/trash/";
pub const FILES_IN_TRASH: &str = "~/.config/wrm/files.json";

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct File {
    path: String,
    from: String,
}

impl File {
    pub fn new<P: AsRef<Path>>(from: P) -> Result<Self> {
        let from = absolutize(from)?;

        let file_name = file_name(&from).unwrap();

        let trash = PathBuf::from(absolutize(TRASH)?).join(file_name);

        let file = Self {
            path: trash.to_string_lossy().to_string(),
            from,
        };

        Ok(file)
    }

    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn from(&self) -> &String {
        &self.from
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilesInTrash {
    files_in_trash: Vec<File>,
}

impl FilesInTrash {
    pub fn new(files_in_trash: Vec<File>) -> Self {
        Self { files_in_trash }
    }

    pub fn files_in_trash(&self) -> &Vec<File> {
        &self.files_in_trash
    }

    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self> {
        let f = fs::File::open(path)
            .map_err(|e| e.into())
            .map_err(WrmError)?;

        let files_in_trash = serde_json::from_reader(f)
            .map_err(|e| e.into())
            .map_err(WrmError)?;

        Ok(files_in_trash)
    }

    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let f = fs::File::create(path)
            .map_err(|e| e.into())
            .map_err(WrmError)?;

        serde_json::to_writer_pretty(f, &self)
            .map_err(|e| e.into())
            .map_err(WrmError)?;

        Ok(())
    }

    pub fn remove(&mut self, file: &File) -> &mut Self {
        self.files_in_trash.retain(|f| f != file);

        self
    }

    pub fn add(&mut self, file: File) -> &mut Self {
        self.files_in_trash.push(file);

        self
    }
}
