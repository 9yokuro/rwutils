use crate::{subcommands, utils::*, FilesInTrash, Result, FILES_IN_TRASH};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true,
    verbatim_doc_comment
)]
struct Arguments {
    #[clap(subcommand)]
    subcommand: Subcommands,
    /// Do not prompt whether to change destinations.
    #[clap(short, long)]
    noninteractive: bool,
    /// Do not print log messages.
    #[clap(short, long)]
    quiet: bool,
}

#[derive(Subcommand, Debug)]
enum Subcommands {
    /// Delete files.
    Delete { paths: Vec<String> },
    /// Empty trash.
    Empty,
    /// List files in trash.
    List,
    /// Move files to trash.
    Remove { paths: Vec<String> },
    /// Restore files to where they came from.
    Restore { paths: Vec<String> },
}

#[derive(Debug)]
pub struct Options {
    noninteractive: bool,
    quiet: bool,
}

impl Options {
    pub const fn new(noninteractive: bool, quiet: bool) -> Self {
        Self {
            noninteractive,
            quiet,
        }
    }

    pub const fn noninteractive(&self) -> bool {
        self.noninteractive
    }

    pub const fn quiet(&self) -> bool {
        self.quiet
    }
}

pub fn parse_arguments() -> Result<()> {
    let arguments = Arguments::parse();

    let options = Options::new(arguments.noninteractive, arguments.quiet);

    let mut files_in_trash = FilesInTrash::read(absolutize(FILES_IN_TRASH)?)?;

    match arguments.subcommand {
        Subcommands::Delete { paths } => subcommands::delete(&paths, &options)?,
        Subcommands::Empty => subcommands::empty(&files_in_trash, &options)?,
        Subcommands::List => subcommands::list(&files_in_trash)?,
        Subcommands::Remove { paths } => {
            subcommands::remove(&paths, &mut files_in_trash, &options)?
        }
        Subcommands::Restore { paths } => {
            subcommands::restore(&paths, &mut files_in_trash, &options)?
        }
    }

    Ok(())
}
