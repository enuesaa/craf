pub mod cli;
pub mod handler;
pub mod service;
pub mod repository;

use crate::cli::run::RunArgs;
use crate::cli::commands::{ListCommandsArgs, DescribeCommandArgs, AddCommandArgs, RemoveCommandArgs};
use crate::handler::list_commands::list_commands;
use crate::handler::run::run_handler;
use crate::handler::add_command::add_command;
use crate::handler::describe_command::describe_command;
use crate::handler::remove_command::remove_command;
use clap::{Parser, Subcommand};
use repository::files::Files;

#[derive(Parser)]
#[command(name = "craftant", about = "Command Shortener", disable_help_subcommand = true)]
struct Cli {
    #[command(subcommand)]
    pub action: Actions,
}

#[derive(Subcommand)]
enum Actions {
    #[command(subcommand)]
    Command(CommandAction),
    Run(RunArgs),
}

#[derive(Subcommand)]
enum CommandAction {
    List(ListCommandsArgs),
    Add(AddCommandArgs),
    Describe(DescribeCommandArgs),
    Remove(RemoveCommandArgs),
}


fn main() {
    let files = Files {};

    let args = Cli::parse();
    let action = args.action;

    match action {
        Actions::Command(command) => {
            match command {
                CommandAction::List(args) => {
                    list_commands(files, args);
                },
                CommandAction::Describe(args) => {
                    describe_command(files, args);
                },
                CommandAction::Add(args) => {
                    add_command(files, args);
                },
                CommandAction::Remove(args) => {
                    remove_command(files, args);
                },
            };
        },
        Actions::Run(args) => run_handler(files, args),
    }
}
