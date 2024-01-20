use crate::{
    actions::{hardlink, symlink},
    Error::{LambError, NotADirectory},
    Result,
};
use clap::{error::ErrorKind, Command, Parser};
use filey::{FileTypes, Filey};

/// lanb
///
/// Create symbolic links and hard links.
///
/// If two files are specified, create symbolic link 1st -> 2nd.
/// If three files or more are specified, create symbolic nth -> last/name_of_nth_file.
/// If you do not use the option --hardlink, create symbolic links by default.
#[derive(Debug, Parser)]
#[clap(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"), arg_required_else_help = true, verbatim_doc_comment)]
pub struct Args {
    file: Vec<String>,
    /// Make backup of each existing destination file.
    #[clap(short, long)]
    backup: bool,
    /// Override the usual backup suffix.
    #[clap(short, long = "suffix", default_value = "~")]
    suffix: String,
    /// Do not prompt whether to remove destinations.
    #[clap(short, long)]
    noninteractive: bool,
    /// Make hard links instead of symbolic links.
    #[clap(short = 'H', long)]
    hardlink: bool,
    /// Do not print name of each linked file.
    #[clap(short, long)]
    quiet: bool,
}

impl Args {
    pub fn file(&self) -> &Vec<String> {
        &self.file
    }

    pub fn backup(&self) -> bool {
        self.backup
    }

    pub fn suffix(&self) -> &String {
        &self.suffix
    }

    pub fn noninteractive(&self) -> bool {
        self.noninteractive
    }

    pub fn quiet(&self) -> bool {
        self.quiet
    }
}

pub fn argparse() -> Result<()> {
    let args = Args::parse();
    let mut cmd = Command::new("lamb");
    let length = args.file.len();
    let tail = Filey::new(&args.file[length - 1]);
    if length < 2 {
        let e = cmd.error(ErrorKind::DisplayHelp, "Incorrect Arguments");
        Err(e).map_err(|e| e.into()).map_err(LambError)
    } else if length > 2 && tail.file_type().unwrap_or(FileTypes::File) != FileTypes::Directory {
        Err(NotADirectory {
            path: tail.to_string(),
        })
    } else {
        if args.hardlink {
            hardlink(&args)?;
        } else {
            symlink(&args)?;
        }
        Ok(())
    }
}
