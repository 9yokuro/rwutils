use crate::actions;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true,
    verbatim_doc_comment
)]
struct Arguments {
    path: Vec<String>,
    /// Create directories instead of files.
    #[clap(short, long)]
    directory: bool,
    /// Do not print log messages.
    #[clap(short, long)]
    quiet: bool,
}

/// Command line options.
#[derive(Debug)]
pub struct Options {
    quiet: bool,
}

impl Options {
    /// Constructs new Options.
    pub const fn new(quiet: bool) -> Self {
        Self { quiet }
    }

    /// Returns the value of the field "quiet".
    pub const fn quiet(&self) -> bool {
        self.quiet
    }
}

pub fn parse_arguments() {
    let arguments = Arguments::parse();

    let options = Options::new(arguments.quiet);

    if arguments.directory {
        actions::create_dir(&arguments.path, &options);
    } else {
        actions::create_file(&arguments.path, &options);
    }
}
