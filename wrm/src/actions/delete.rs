use crate::{utils::*, Error::NotFound, Options, Result};
use colored::Colorize;
use std::path::Path;

pub fn delete(paths: &Vec<String>, options: &Options) -> Result<()> {
    for path in paths {
        if !Path::new(path).exists() {
            eprintln!("error: {}", NotFound);
            continue;
        }

        if options.noninteractive() || confirm(format!("{} '{}'?", "Delete".red().bold(), path))? {
            if let Err(e) = remove(path) {
                eprintln!("error: {}", e);
                continue;
            }

            if !options.quiet() {
                eprintln!("{} '{}'", "Deleted".green().bold(), path);
                continue;
            }
        }

        if !options.quiet() {
            eprintln!("Canceled");
        }
    }
    Ok(())
}
