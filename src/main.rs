pub mod command;
pub mod cli;
pub mod handler;

use clap::Parser;
use crate::cli::{Cli, Commands};
use crate::handler::run::run;

fn main() {
    let args = Cli::parse();
    let command = args.command;

    match command {
        Commands::Run(_) => {
            let _ = run();
        },
        _ => {},
    }
}
