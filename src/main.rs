pub mod cli;
pub mod command;
pub mod handler;
pub mod service;

use crate::cli::{Actions, Cli};
use crate::handler::list_commands::list_commands;
use crate::handler::run::run;
use clap::Parser;
use cli::CommandAction;
use handler::create_command::create_command;

fn main() {
    let args = Cli::parse();
    let command = args.action;

    match command {
        Actions::Run(_) => {
            let _ = run();
        }
        Actions::Command(command) => {
            match command {
                CommandAction::List(_) => {
                    list_commands();
                }
                CommandAction::Create(arg) => {
                    create_command(arg.name);
                }
                _ => {}
            };
        }
    }
}
