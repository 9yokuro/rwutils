use crate::{Error::WrmError, Result};
use filey::{Error::GetFileNameError, Filey};
use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FileInfo {
    path: String,
    path_trash: String,
}

impl FileInfo {
    pub fn new<P: AsRef<Path>>(path: P, wrm_config_dir: &String) -> Result<Self> {
        let path = path.as_ref().display().to_string();
        let path_trash = format!(
            "{}/trash/{}",
            wrm_config_dir,
            Filey::new(&path)
                .file_name()
                .ok_or_else(|| GetFileNameError {
                    path: path.to_string()
                })
                .map_err(|e| e.into())
                .map_err(WrmError)?
        );
        let fileinfo = FileInfo { path, path_trash };
        Ok(fileinfo)
    }

    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn path_trash(&self) -> &String {
        &self.path_trash
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FileList {
    files: Vec<FileInfo>,
}

impl FileList {
    pub fn new() -> Self {
        FileList { files: vec![] }
    }

    pub fn files(&self) -> &Vec<FileInfo> {
        &self.files
    }

    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self> {
        let f = File::open(path).map_err(|e| e.into()).map_err(WrmError)?;
        let files = serde_json::from_reader(f)
            .map_err(|e| e.into())
            .map_err(WrmError)?;
        Ok(files)
    }

    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let f = File::create(path).map_err(|e| e.into()).map_err(WrmError)?;
        serde_json::to_writer_pretty(f, &self)
            .map_err(|e| e.into())
            .map_err(WrmError)?;
        Ok(())
    }

    pub fn add(&mut self, fileinfo: &FileInfo) -> &mut Self {
        self.files.push(fileinfo.clone());
        self
    }

    pub fn remove(&mut self, fileinfo: &FileInfo) -> &mut Self {
        self.files.retain(|x| x != fileinfo);
        self
    }
}
