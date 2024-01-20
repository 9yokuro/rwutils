use crate::{
    argparse::Args,
    Error::{Canceled, LambError},
    Result,
};
use colored::Colorize;
use filey::{FileTypes, Filey};
use std::{
    fmt::Display,
    io::{stdin, stdout, Write},
};

fn confirm<S: Display>(message: S) -> Result<bool> {
    let mut s = String::new();
    print!("{}", message);
    stdout().flush().map_err(|e| e.into()).map_err(LambError)?;
    stdin()
        .read_line(&mut s)
        .map_err(|e| e.into())
        .map_err(LambError)?;
    let result = s.trim().to_lowercase();
    let result = result.as_str();
    if result == "y" || result == "yes" {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn check(link: &Filey, args: &Args) -> Result<()> {
    if link.exists() {
        if args.backup() {
            let backup = link
                .move_to(format!("{}{}", link, args.suffix()))
                .map_err(|e| e.into())
                .map_err(LambError)?;
            if !args.quiet() {
                eprintln!("{} backup '{}'", "Created".green().bold(), backup);
            }
            Ok(())
        } else if !args.noninteractive() {
            let message = format!("{} '{}'? [y/N] ", "Replace".red().bold(), link);
            if confirm(message)? {
                link.remove().map_err(|e| e.into()).map_err(LambError)?;
                if !args.quiet() {
                    eprintln!("{} '{}'", "Replaced".green().bold(), link);
                }
                Ok(())
            } else if !args.quiet() {
                Err(Canceled)
            } else {
                Ok(())
            }
        } else {
            link.remove().map_err(|e| e.into()).map_err(LambError)?;
            if !args.quiet() {
                eprintln!("{} '{}'", "Replaced".green().bold(), link);
            }
            Ok(())
        }
    } else {
        Ok(())
    }
}

fn prepare(path: &String, args: &Args) -> Result<String> {
    let length = args.file().len();
    let tail = Filey::new(&args.file()[length - 1]);
    let call_closure = |path: &String, file_type: &FileTypes| -> Result<String> {
        match file_type {
            FileTypes::Directory => {
                let original = Filey::new(path);
                let link = Filey::new(format!(
                    "{}/{}",
                    tail,
                    original.file_name().expect("Failed to get file name")
                ));
                check(&link, args)?;

                Ok(link.to_string())
            }
            _ => {
                check(&tail, args)?;
                Ok(tail.to_string())
            }
        }
    };
    match &tail.file_type() {
        Some(file_type) => Ok(call_closure(path, file_type)?),
        None => Ok(tail.to_string()),
    }
}

pub fn symlink(args: &Args) -> Result<()> {
    let length = args.file().len();
    if length == 2 {
        let original = &args.file()[0];
        match prepare(original, args) {
            Ok(link) => {
                Filey::new(original)
                    .symlink(&link)
                    .map_err(|e| e.into())
                    .map_err(LambError)?;
                if !args.quiet() {
                    eprintln!(
                        "{} symlink '{}' -> '{}'",
                        "Created".green().bold(),
                        original,
                        link
                    )
                }
            }
            Err(e) => match e {
                Canceled => {
                    eprintln!("Canceled");
                    return Ok(());
                }
                _ => return Err(e),
            },
        };
    } else {
        for i in 0..(length - 1) {
            let original = &args.file()[i];
            let link = prepare(original, args)?;
            Filey::new(original)
                .symlink(&link)
                .map_err(|e| e.into())
                .map_err(LambError)?;
            if !args.quiet() {
                eprintln!(
                    "{} symlink '{}' -> '{}'",
                    "Created".green().bold(),
                    original,
                    link
                )
            }
        }
    }
    Ok(())
}

pub fn hardlink(args: &Args) -> Result<()> {
    let length = args.file().len();
    if length == 2 {
        let original = &args.file()[0];
        match prepare(original, args) {
            Ok(link) => {
                Filey::new(original)
                    .hard_link(&link)
                    .map_err(|e| e.into())
                    .map_err(LambError)?;
                if !args.quiet() {
                    eprintln!(
                        "{} hard link '{}' => '{}'",
                        "Created".green().bold(),
                        original,
                        link
                    )
                }
            }
            Err(e) => match e {
                Canceled => {
                    eprintln!("Canceled");
                    return Ok(());
                }
                _ => return Err(e),
            },
        };
    } else {
        for i in 0..(length - 1) {
            let original = &args.file()[i];
            let link = prepare(original, args)?;
            Filey::new(original)
                .symlink(&link)
                .map_err(|e| e.into())
                .map_err(LambError)?;
            if !args.quiet() {
                eprintln!(
                    "{} symlink '{}' -> '{}'",
                    "Created".green().bold(),
                    original,
                    link
                )
            }
        }
    }
    Ok(())
}
