pub mod cli;
pub mod handler;
pub mod service;
pub mod repository;
pub mod repos;

use std::process;
use crate::cli::run::RunArgs;
use crate::cli::commands::{ListCommandsArgs, DescribeCommandArgs, AddCommandArgs, RemoveCommandArgs};
use crate::handler::list_commands::list_commands;
use crate::handler::run::run_handler;
use crate::handler::add_command::add_command;
use crate::handler::describe_command::describe_command;
use crate::handler::remove_command::remove_command;
use crate::repos::Repos;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "crafant", about = "A CLI tool to memorize long shell commands.", disable_help_subcommand = true)]
struct Cli {
    #[command(subcommand)]
    pub action: Actions,
}

#[derive(Subcommand, Debug)]
enum Actions {
    #[command(subcommand)]
    Command(CommandAction),
    Run(RunArgs),
}

#[derive(Subcommand, Debug)]
enum CommandAction {
    List(ListCommandsArgs),
    Add(AddCommandArgs),
    Describe(DescribeCommandArgs),
    Remove(RemoveCommandArgs),
}


fn main() {
    let repos = Repos {};

    let args = Cli::parse();
    let action = args.action;

    let status = match action {
        Actions::Command(command) => match command {
            CommandAction::List(args) => {
                list_commands(repos, args)
            },
            CommandAction::Describe(args) => {
                describe_command(repos, args)
            },
            CommandAction::Add(args) => {
                add_command(repos, args)
            },
            CommandAction::Remove(args) => {
                remove_command(repos, args)
            },
        },
        Actions::Run(args) => run_handler(repos, args),
    };

    process::exit(status);
}
