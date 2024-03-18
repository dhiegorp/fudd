
mod cli;
use crate::cli::*;
use clap::Parser;

pub fn main() {
    let _cli = FuddConnect::parse();
}
