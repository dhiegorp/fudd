mod cli;

use crate::cli::*;
use clap::Parser;

pub fn main() {
    let cli = FuddConnect::parse();
}
