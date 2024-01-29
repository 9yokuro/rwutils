use crate::{actions::*, Error::WlsError, Result};
use clap::Parser;
use filey::Filey;

#[derive(Debug, Parser)]
#[clap(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"), arg_required_else_help = true, verbatim_doc_comment)]
struct Args {
    path: Option<Vec<String>>,
    #[clap(short = 'r', long)]
    recursive: bool,
    #[clap(short, long)]
    all: bool,
    #[clap(short = 'c', long = "no-color")]
    no_color: bool,
    #[clap(short = 't', long = "no-time")]
    no_time: bool,
    #[clap(short = 'p', long = "no-permission")]
    no_permission: bool,
}

pub fn argparse() -> Result<()> {
    let args = Args::parse();
    if let Some(path) = args.path {
    } else {
    }
    Ok(())
}
