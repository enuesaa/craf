pub mod commands;
pub mod cli;
pub mod run;
pub mod tokiorun;

use clap::Parser;
use crate::cli::Cli;

fn main() {
    let args = Cli::parse();
    let command = args.command;
    println!("{:?}", command);
}
