pub mod cli;
pub mod command;
pub mod handler;
pub mod service;

use crate::cli::{Actions, Cli};
use crate::handler::list_commands::list_commands_handler;
use crate::handler::run::run_handler;
use clap::Parser;
use cli::CommandAction;
use handler::add_command::add_command_handler;
use handler::describe_command::describe_command_handler;
use handler::remove_command::remove_command_handler;

fn main() {
    let args = Cli::parse();
    let command = args.action;

    match command {
        Actions::Run(args) => {
            run_handler(&args.name);
        }
        Actions::Command(command) => {
            match command {
                CommandAction::List(_) => {
                    list_commands_handler();
                },
                CommandAction::Describe(args) => {
                    describe_command_handler(&args.name);
                },
                CommandAction::Add(_) => {
                    add_command_handler();
                },
                CommandAction::Remove(args) => {
                    remove_command_handler(&args.name);
                },
            };
        }
    }
}
