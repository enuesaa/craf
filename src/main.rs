pub mod cli;
pub mod handlers;
pub mod services;

use crate::cli::run::RunArgs;
use crate::cli::commands::{ListCommandsArgs, DescribeCommandArgs, AddCommandArgs, RemoveCommandArgs};
use crate::handlers::list_commands::list_commands;
use crate::handlers::run::run_handler;
use crate::handlers::add_command::add_command;
use crate::handlers::describe_command::describe_command;
use crate::handlers::remove_command::remove_command;
use clap::{Parser, Subcommand};

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
    let args = Cli::parse();
    let action = args.action;

    match action {
        Actions::Command(command) => {
            match command {
                CommandAction::List(args) => list_commands(args),
                CommandAction::Describe(args) => describe_command(args),
                CommandAction::Add(args) => add_command(args),
                CommandAction::Remove(args) => remove_command(args),
            };
        },
        Actions::Run(args) => run_handler(args),
    }
}
