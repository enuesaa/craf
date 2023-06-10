pub mod command;
pub mod cli;
pub mod handler;

use clap::Parser;
use crate::cli::{Cli, Actions};
use crate::handler::run::run;

fn main() {
    let args = Cli::parse();
    let command = args.action;

    match command {
        Actions::Run(_) => {
            let _ = run();
        },
        _ => {},
    }
}
