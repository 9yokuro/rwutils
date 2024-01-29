use crate::{Error::WlsError, Result};
use chrono::{Local, TimeZone};
use colored::Colorize;
use filey::{FileTypes, Filey};
use std::{
    fs::{metadata, read_dir},
    os::unix::fs::PermissionsExt,
    path::Path,
    time::UNIX_EPOCH,
};
use walkdir::WalkDir;

pub fn list_all_files<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    let mut files = vec![];
    for i in read_dir(path).map_err(|e| e.into()).map_err(WlsError)? {
        files.push(
            i.map_err(|e| e.into())
                .map_err(WlsError)?
                .path()
                .display()
                .to_string(),
        );
    }
    Ok(files)
}

pub fn list_files<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    let mut files = list_all_files(path)?;
    files.retain(|p| {
        !Filey::new(p)
            .file_name()
            .map(|s| s.starts_with('.'))
            .unwrap_or(false)
    });
    Ok(files)
}

pub fn list_all_files_recursively<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    let mut files = vec![];
    for i in WalkDir::new(path) {
        files.push(
            i.map_err(|e| e.into())
                .map_err(WlsError)?
                .path()
                .display()
                .to_string(),
        );
    }
    Ok(files)
}

pub fn list_files_recursively<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    let mut files = vec![];
    for i in WalkDir::new(path).into_iter().filter_entry(|p| {
        !p.file_name()
            .to_str()
            .map(|s| s.starts_with('.'))
            .unwrap_or(false)
    }) {
        files.push(
            i.map_err(|e| e.into())
                .map_err(WlsError)?
                .path()
                .display()
                .to_string(),
        );
    }
    Ok(files)
}

const KIB: usize = 2_usize.pow(10);
const MIB: usize = 2_usize.pow(20);
const GIB: usize = 2_usize.pow(30);
const TIB: usize = 2_usize.pow(40);
const PIB: usize = 2_usize.pow(50);
const EIB: usize = 2_usize.pow(60);

fn size<P: AsRef<Path>>(path: P) -> Result<String> {
    let size = metadata(path)
        .map_err(|e| e.into())
        .map_err(WlsError)?
        .len() as usize;
    if size < KIB {
        Ok(format!("{}", size))
    } else if (KIB..MIB).contains(&size) {
        Ok(format!("{}K", size / KIB))
    } else if (MIB..GIB).contains(&size) {
        Ok(format!("{}M", size / MIB))
    } else if (GIB..TIB).contains(&size) {
        Ok(format!("{}G", size / GIB))
    } else if (TIB..PIB).contains(&size) {
        Ok(format!("{}T", size / TIB))
    } else if (PIB..EIB).contains(&size) {
        Ok(format!("{}P", size / PIB))
    } else if EIB <= size {
        Ok(format!("{}E", size / EIB))
    } else {
        Ok(format!("{}", size))
    }
}

pub fn size_styled<P: AsRef<Path>>(path: P) -> Result<String> {
    let size = size(path)?;
    let length = size.len();
    if length == 1 {
        Ok(format!("   {}", size))
    } else if length == 2 {
        Ok(format!("  {}", size))
    } else if length == 3 {
        Ok(format!(" {}", size))
    } else {
        Ok(size)
    }
}

pub fn permissions_styled<P: AsRef<Path>>(path: P) -> Result<String> {
    let permissions = permissions(path)?;
    let length = permissions.len();
    Ok(format!(
        "u: {}  g: {}  o: {}",
        format_permissions(permissions[length - 3]),
        format_permissions(permissions[length - 2]),
        format_permissions(permissions[length - 1])
    ))
}

fn format_permissions(c: char) -> String {
    if c == '1' {
        "--x".to_string()
    } else if c == '2' {
        "-w-".to_string()
    } else if c == '3' {
        "-wx".to_string()
    } else if c == '4' {
        "r--".to_string()
    } else if c == '5' {
        "r-x".to_string()
    } else if c == '6' {
        "rw-".to_string()
    } else if c == '7' {
        "rwx".to_string()
    } else {
        "---".to_string()
    }
}

fn permissions<P: AsRef<Path>>(path: P) -> Result<Vec<char>> {
    let permissions = format!(
        "{:o}",
        metadata(path)
            .map_err(|e| e.into())
            .map_err(WlsError)?
            .permissions()
            .mode()
    );
    Ok(permissions.chars().skip(2).collect::<Vec<char>>())
}

pub fn colorize<P: AsRef<Path>>(path: P) -> Result<String> {
    let filey = Filey::new(&path);
    match filey.file_type().unwrap() {
        FileTypes::File if is_executable(path)? => Ok(format!("{}", filey.to_string().green())),
        FileTypes::File => Ok(filey.to_string()),
        FileTypes::Directory => Ok(format!("{}", filey.to_string().blue())),
        FileTypes::Symlink => Ok(format!("{}", filey.to_string().cyan())),
    }
}

pub fn format_path(p: String, q: String) -> String {
    p.replacen(&q, "", 1).replacen("/", "", 1)
}

fn is_executable<P: AsRef<Path>>(path: P) -> Result<bool> {
    let permissions = permissions(path)?;
    let length = permissions.len();
    Ok(parse_char_to_usize(permissions[length - 1])? % 2 == 1
        || parse_char_to_usize(permissions[length - 2])? % 2 == 1
        || parse_char_to_usize(permissions[length - 3])? % 2 == 1)
}

fn parse_char_to_usize(c: char) -> Result<usize> {
    c.to_string()
        .parse::<usize>()
        .map_err(|e| e.into())
        .map_err(WlsError)
}

pub fn time<P: AsRef<Path>>(path: P) -> Result<String> {
    let unix_time = metadata(path)
        .map_err(|e| e.into())
        .map_err(WlsError)?
        .modified()
        .map_err(|e| e.into())
        .map_err(WlsError)?
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.into())
        .map_err(WlsError)?;
    Ok(Local
        .timestamp_nanos(unix_time.as_nanos() as i64)
        .format("%m/%d %R")
        .to_string())
}
