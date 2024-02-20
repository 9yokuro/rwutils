use crate::{
    exists_or_else, utils::*, Error::NotFound, FilesInTrash, Options, Result, FILES_IN_TRASH,
};
use colored::Colorize;
use filey::Filey;
use std::fmt::Display;

pub fn restore(
    paths: &Vec<String>,
    files_in_trash: &mut FilesInTrash,
    options: &Options,
) -> Result<()> {
    for path in paths {
        let path = &absolutize(path)?;

        exists_or_else!(path);

        restore_inner(path, files_in_trash, options)?;
    }

    Ok(())
}

fn restore_inner(
    path: &String,
    files_in_trash: &mut FilesInTrash,
    options: &Options,
) -> Result<()> {
    for file in files_in_trash.clone().files_in_trash() {
        if path == file.path() {
            if !options.noninteractive() {
                let message = format!(
                    "{} '{}' to '{}'?",
                    "Restore".red().bold(),
                    file.path(),
                    file.from()
                );

                if !confirm(message)? {
                    eprintln!("Canceled");

                    break;
                }
            }

            if let Err(e) = rename(file.path(), file.from()) {
                eprintln(e);

                break;
            }

            files_in_trash
                .remove(file)
                .write(absolutize(FILES_IN_TRASH)?)?;

            print_log_message(file.path(), file.from(), options.quiet());

            break;
        }
    }

    Ok(())
}

fn print_log_message<D: Display, E: Display>(path: D, from: E, quiet: bool) {
    if !quiet {
        eprintln!("{} '{}' to '{}'", "Restored".green().bold(), path, from);
    }
}
