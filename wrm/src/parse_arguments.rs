use crate::{actions, Error::IncorrectArguments, FilesInTrash, Result, FILES_IN_TRASH};
use clap::Parser;

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
    path: Option<Vec<String>>,
    #[clap(short, long)]
    clean: bool,
    #[clap(short, long)]
    delete: bool,
    #[clap(short, long)]
    list: bool,
    #[clap(short, long)]
    restore: bool,
    #[clap(short, long)]
    noninteractive: bool,
    #[clap(short, long)]
    quiet: bool,
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

    if arguments.clean {
    } else if arguments.list {
        let files_in_trash = FilesInTrash::read(FILES_IN_TRASH)?;
        actions::list(&files_in_trash)?;
    } else if let Some(paths) = arguments.path {
        if arguments.delete {
            actions::delete(&paths, &options)?;
        } else if arguments.restore {
        } else {
        }
    } else {
        return Err(IncorrectArguments);
    }
    Ok(())
}
