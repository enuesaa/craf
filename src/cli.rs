use crate::command::create_command::CreateCommandArgs;
use crate::command::describe_command::DescribeCommandArgs;
use crate::command::list_commands::ListCommandsArgs;
use crate::command::remove_command::RemoveCommandArgs;
use crate::command::run::RunArgs;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "craftant", bin_name = "craftant", about = "Command Proxy.")]
#[clap(disable_help_subcommand = false, disable_help_flag = true)]
pub struct Cli {
    #[command(subcommand)]
    pub action: Actions,
}

#[derive(Subcommand, Debug)]
pub enum Actions {
    #[command(subcommand)]
    Command(CommandAction),
    Run(RunArgs),
}

#[derive(Subcommand, Debug)]
#[clap(disable_help_subcommand = true)]
pub enum CommandAction {
    List(ListCommandsArgs),
    Create(CreateCommandArgs),
    Describe(DescribeCommandArgs),
    Remove(RemoveCommandArgs),
}
