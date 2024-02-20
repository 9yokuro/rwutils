use crate::{exists_or_else, utils::*, Error::NotFound, Options, Result};
use colored::Colorize;
use filey::Filey;

pub fn delete(paths: &Vec<String>, options: &Options) -> Result<()> {
    for path in paths {
        exists_or_else!(path);

        if !options.noninteractive() {
            let message = format!("{} '{}'?", "Delete".red().bold(), path);

            if !confirm(message)? {
                eprintln!("Canceled");

                continue;
            }
        }

        if let Err(e) = remove(path) {
            eprintln(e);

            continue;
        }

        if !options.quiet() {
            eprintln!("{} '{}'", "Deleted".green().bold(), path);
        }
    }

    Ok(())
}
