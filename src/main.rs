pub mod handler;
pub mod repos;
pub mod repository;
pub mod service;

use crate::handler::add_command::add_command_handler;
use crate::handler::describe_command::describe_command_handler;
use crate::handler::list_commands::list_commands_handler;
use crate::handler::remove_command::remove_command_handler;
use crate::handler::run::run_handler;
use crate::repos::Repos;
use clap::{crate_version, CommandFactory, Parser};
use std::process;

#[derive(Parser, Debug)]
#[command(
    name = "craf",
    about = "A CLI tool to shorthand long shell commands.",
    disable_version_flag = true,
    disable_help_subcommand = true
)]
struct Cli {
    /// Task name
    pub name: Option<String>,

    /// List tasks
    #[arg(long)]
    pub list: bool,

    /// Create new task
    #[arg(long)]
    pub create: bool,

    /// Delete a task
    #[arg(long)]
    pub delete: bool,

    /// Describe a task
    #[arg(long)]
    pub describe: bool,

    /// Print version
    #[arg(short = 'v', long = "version", global = true)]
    pub version: bool,
}

fn main() {
    let repos = Repos {};
    let args = Cli::parse();

    let code: i32;
    if args.version {
        println!("{}", crate_version!());
        code = 0;
    } else if args.list {
        code = list_commands_handler(repos);
    } else if args.create {
        code = add_command_handler(repos);
    } else if args.describe {
        if args.name.is_none() {
            println!("<name> is required.");
            code = 1;
        } else {
            code = describe_command_handler(repos, &args.name.unwrap());
        }
    } else if args.delete {
        if args.name.is_none() {
            println!("<name> is required.");
            code = 1;
        } else {
            code = remove_command_handler(repos, &args.name.unwrap());
        }
    } else {
        if args.name.is_none() {
            let _ = Cli::command().print_help();
            code = 1;
        } else {
            code = run_handler(repos, &args.name.unwrap());
        }
    }

    process::exit(code);
}
