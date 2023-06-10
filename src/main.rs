pub mod command;
pub mod cli;
pub mod handler;
pub mod service;

use clap::Parser;
use cli::CommandAction;
use handler::create_command::create_command;
use crate::cli::{Cli, Actions};
use crate::handler::run::run;
use crate::handler::list_commands::list_commands;

fn main() {
    let args = Cli::parse();
    let command = args.action;

    match command {
        Actions::Run(_) => {
            let _ = run();
        },
        Actions::Command(command) => {
            match command {
                CommandAction::List(_) => {
                    list_commands();
                },
                CommandAction::Create(arg) => {
                    create_command(arg.name);
                }
                _ => {},
            };
        },
    }
}
