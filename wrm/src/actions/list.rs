use crate::{utils::*, Error::WrmError, FilesInTrash, Result};
use colored::Colorize;
use filey::FileTypes;
use std::{
    fmt::Display,
    io::{stdout, BufWriter, Write},
    path::Path,
};

pub fn list() -> Result<()> {
    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    for file in files_in_trash.files_in_trash() {
        let path = colorize(file.path());
        let output = format(&path, file.trash());

        writeln!(out, "{}", output)
            .map_err(|e| e.into())
            .map_err(WrmError)?;
    }

    Ok(())
}

fn colorize<P: AsRef<Path>>(path: P) -> String {
    let path = &asref_path_to_string(path);

    match FileTypes::which(path).unwrap() {
        FileTypes::File => path.to_string(),
        FileTypes::Symlink => path.cyan().to_string(),
        FileTypes::Directory => path.blue().to_string(),
    }
}

fn format<D: Display>(path: D, trash: D) -> String {
    format!("{} ({})", trash, path)
}
