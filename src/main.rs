pub mod command;
pub mod cli;

use clap::Parser;
use crate::cli::Cli;

fn main() {
    let args = Cli::parse();
    let command = args.command;
    println!("{:?}", command);
}
