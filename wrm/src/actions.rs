use crate::{
    file_list::{FileInfo, FileList},
    Error::{NotFoundError, WrmError},
    Result,
};
use colored::Colorize;
use filey::{create, remove, Error::GetFileNameError, FileTypes, Filey};
use std::{
    fmt::Display,
    io::{stdin, stdout, Write},
    path::Path,
};

// Prompt before every actions.
fn ask<D: Display>(message: D) -> Result<bool> {
    let mut s = String::new();
    print!("{}", message);
    stdout().flush().map_err(|e| e.into()).map_err(WrmError)?;
    stdin()
        .read_line(&mut s)
        .map_err(|e| e.into())
        .map_err(WrmError)?;
    let answer = s.trim().to_lowercase();
    Ok(answer.as_str() == "y" || answer.as_str() == "yes")
}

fn confirm<D: Display>(noninteractive: bool, message: D) -> Result<bool> {
    Ok(noninteractive || ask(message)?)
}

// Move files or directories to trash(~/.config/wrm/trash)
pub fn remove(
    path: Vec<String>,
    wrm_config_dir: &String,
    noninteractive: bool,
    quiet: bool,
) -> Result<()> {
    for i in path {
        let target = absolutize(i)?;
        let file_type = if let Some(t) = target.file_type() {
            t
        } else {
            eprintln!(
                "error: {}",
                NotFoundError {
                    path: target.to_string()
                }
            );
            continue;
        };
        if confirm(
            noninteractive,
            format!(
                "{} {} '{}'? [y/N] ",
                "Remove".red().bold(),
                file_type,
                &target,
            ),
        )? {
            if file_type == FileTypes::Symlink {
                if let Err(e) = target.remove() {
                    eprintln!("error: {}", e);
                    continue;
                }
                show_message(
                    quiet,
                    format!("{} {} '{}'", "Removed".green().bold(), file_type, &target),
                );
            } else {
                if let Err(e) = target.clone().move_to(format!("{}/trash/", wrm_config_dir)) {
                    eprintln!("error: {}", e);
                    continue;
                }
                add_file_info(
                    wrm_config_dir,
                    &FileInfo::new(target.path(), wrm_config_dir)?,
                )?;
                show_message(
                    quiet,
                    format!("{} {} '{}'", "Removed".green().bold(), file_type, &target),
                );
            }
        } else {
            show_message(quiet, "Canceled");
        }
        check(wrm_config_dir)?;
    }
    Ok(())
}

fn add_file_info(wrm_config_dir: &String, file_info: &FileInfo) -> Result<()> {
    let path_to_file_list = format!("{}/list.json", wrm_config_dir);
    FileList::read(&path_to_file_list)?
        .add(file_info)
        .write(&path_to_file_list)
}

fn show_message<D: Display>(quiet: bool, message: D) {
    if !quiet {
        eprintln!("{}", message);
    }
}

// Delete all files and directories in trash permanently
pub fn clean(wrm_config_dir: &String, noninteractive: bool, quiet: bool) -> Result<()> {
    let file_list = format!("{}/list.json", wrm_config_dir);
    if FileList::read(file_list)?.files().is_empty() {
        eprintln!("There are no files or directories in trash");
    } else {
        if !noninteractive {
            list(wrm_config_dir)?;
        }
        if confirm(
            noninteractive,
            format!(
                "{} these files and directories? [y/N] ",
                "Delete".red().bold()
            ),
        )? {
            remove!(wrm_config_dir);
            create!(FileTypes::Directory, wrm_config_dir);
            show_message(quiet, format!("{} trash", "Cleaned".green().bold()));
        } else {
            show_message(quiet, "Canceled");
        }
    }
    Ok(())
}

// Delete files or directories
pub fn delete(
    path: Vec<String>,
    wrm_config_dir: &String,
    noninteractive: bool,
    quiet: bool,
) -> Result<()> {
    for i in path {
        let target = absolutize(i)?;
        let file_type = if let Some(t) = target.file_type() {
            t
        } else {
            eprintln!(
                "error: {}",
                NotFoundError {
                    path: target.to_string()
                }
            );
            continue;
        };
        if confirm(
            noninteractive,
            format!(
                "{} {} '{}'? [y/N] ",
                "Delete".red().bold(),
                file_type,
                &target
            ),
        )? {
            if let Err(e) = target.remove() {
                eprintln!("error {}", e);
                continue;
            }
            show_message(
                quiet,
                format!("{} {} '{}'", "Deleted".green().bold(), file_type, &target),
            );
        } else {
            show_message(quiet, "Canceled");
        }
        check(wrm_config_dir)?;
    }
    Ok(())
}

// Restore files or directories in trash to where they came from
pub fn restore(
    path: Vec<String>,
    wrm_config_dir: &String,
    noninteractive: bool,
    quiet: bool,
) -> Result<()> {
    for i in path {
        let given = absolutize(i)?;
        for j in FileList::read(format!("{}/list.json", wrm_config_dir))?.files() {
            let original = absolutize(j.path())?;
            let mut target = absolutize(j.path_trash())?;
            let file_type = target.file_type().ok_or_else(|| NotFoundError {
                path: target.to_string(),
            })?;
            if given.path() == target.path() {
                if confirm(
                    noninteractive,
                    format!(
                        "{} {} '{}' to '{}'? [y/N] ",
                        "Restore".red().bold(),
                        file_type,
                        &target,
                        &original
                    ),
                )? {
                    if let Err(e) = target.move_to(&original) {
                        eprintln!("error: {}", e);
                        break;
                    }
                    show_message(
                        quiet,
                        format!(
                            "{} {} '{}' to '{}'",
                            "Restored".green().bold(),
                            file_type,
                            &target,
                            &original
                        ),
                    );
                } else {
                    show_message(quiet, "Canceled");
                }
                break;
            }
        }
        check(wrm_config_dir)?;
    }
    Ok(())
}

// List all files and directories in trash
pub fn list(wrm_config_dir: &String) -> Result<()> {
    let file_list = FileList::read(format!("{}/list.json", wrm_config_dir))?;
    if file_list.files().is_empty() {
        eprintln!("There are no files or directories in trash");
    } else {
        draw(&file_list)?;
    }
    Ok(())
}

fn draw(file_list: &FileList) -> Result<()> {
    for i in file_list.files() {
        println!("{}", colorize(i.path_trash(), i.path())?);
    }
    Ok(())
}

fn colorize<P: AsRef<Path>>(path: P, original: P) -> Result<String> {
    let target = Filey::new(path);
    let file_name = target
        .file_name()
        .ok_or_else(|| GetFileNameError {
            path: target.to_string(),
        })
        .map_err(|e| e.into())
        .map_err(WrmError)?;
    match target.file_type().ok_or_else(|| NotFoundError {
        path: target.to_string(),
    })? {
        FileTypes::File => Ok(format!(
            "{} ({}) {}",
            file_name,
            original.as_ref().display(),
            FileTypes::File
        )),
        FileTypes::Directory => Ok(format!(
            "{} ({}) {}",
            file_name.blue(),
            original.as_ref().display(),
            FileTypes::Directory
        )),
        FileTypes::Symlink => Ok(format!(
            "{} ({}) {}",
            file_name.cyan(),
            original.as_ref().display(),
            FileTypes::Symlink
        )),
    }
}

fn check(wrm_config_dir: &String) -> Result<()> {
    let path_to_file_list = format!("{}/list.json", wrm_config_dir);
    let file_list = FileList::read(&path_to_file_list)?;
    for i in file_list.files() {
        if !absolutize(i.path_trash())?.exists() {
            file_list.clone().remove(i).write(&path_to_file_list)?;
        }
    }
    Ok(())
}

fn absolutize<P: AsRef<Path>>(path: P) -> Result<Filey> {
    let mut temp = Filey::new(path);
    Ok(temp
        .absolutized()
        .map_err(|e| e.into())
        .map_err(WrmError)?
        .clone())
}
