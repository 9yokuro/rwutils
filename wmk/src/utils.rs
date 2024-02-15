use colored::Colorize;
use std::fmt::Display;

pub fn print_error_message<D: Display>(message: D) {
    eprintln!("{}: {}", "error".red().bold(), message);
}
