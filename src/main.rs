pub mod cli;
pub mod handler;
pub mod service;
pub mod repository;
pub mod repos;

use std::process;
use crate::cli::run::RunArgs;
use crate::cli::commands::{ListArgs, DescribeArgs, AddArgs, RemoveArgs};
use crate::handler::list_commands::list_commands_handler;
use crate::handler::run::run_handler;
use crate::handler::add_command::add_command_handler;
use crate::handler::describe_command::describe_command_handler;
use crate::handler::remove_command::remove_command_handler;
use crate::repos::Repos;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "crafant", about = "A CLI tool to shorthand long shell commands.", version, disable_help_subcommand = true)]
struct Cli {
    #[command(subcommand)]
    pub action: Actions,
}

#[derive(Subcommand, Debug)]
enum Actions {
    List(ListArgs),
    Add(AddArgs),
    Describe(DescribeArgs),
    Remove(RemoveArgs),
    Run(RunArgs),
}


fn main() {
    let repos = Repos {};

    let args = Cli::parse();
    let action = args.action;

    let status = match action {
        Actions::List(args) => list_commands_handler(repos, args),
        Actions::Describe(args) => describe_command_handler(repos, args),
        Actions::Add(args) => add_command_handler(repos, args),
        Actions::Remove(args) => remove_command_handler(repos, args),
        Actions::Run(args) => run_handler(repos, args),
    };

    process::exit(status);
}
