use crate::{utils::*, Options};
use colored::Colorize;
use filey::{FileTypes, Filey};
use std::fmt::Display;

pub fn create_dir(paths: &Vec<String>, options: &Options) {
    for path in paths {
        if let Err(e) = Filey::new(path).create_dir() {
            print_error_message(e);
            continue;
        }

        if !options.quiet() {
            print_log_message(path, FileTypes::Directory);
        }
    }
}

pub fn create_file(paths: &Vec<String>, options: &Options) {
    for path in paths {
        if let Err(e) = Filey::new(path).create_file() {
            print_error_message(e);
            continue;
        }

        if !options.quiet() {
            print_log_message(path, FileTypes::File);
        }
    }
}

fn print_log_message<D: Display, E: Display>(path: D, file_type: E) {
    eprintln!("{} {} '{}'", "Created".green().bold(), file_type, path);
}
