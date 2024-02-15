pub mod actions;
mod parse_arguments;
pub mod utils;

pub use crate::parse_arguments::Options;

use crate::parse_arguments::parse_arguments;

fn main() {
    parse_arguments();
}
