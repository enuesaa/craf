use crate::command::add_command::AddCommandArgs;
use crate::command::describe_command::DescribeCommandArgs;
use crate::command::list_commands::ListCommandsArgs;
use crate::command::remove_command::RemoveCommandArgs;
use crate::command::run_command::RunArgs;
use clap::{Parser, Subcommand, ArgAction};

#[derive(Parser)]
#[command(name = "craftant", bin_name = "craftant", about = "Command Shortener")]
#[clap(disable_help_subcommand = true, disable_help_flag = true)]
pub struct Cli {
    #[command(subcommand)]
    pub action: Actions,

    #[arg(long, action = ArgAction::Help, help = "Show help messages")]
    help: Option<bool>,
}

#[derive(Subcommand, Debug)]
pub enum Actions {
    #[command(subcommand)]
    Command(CommandAction),
    Run(RunArgs),
}

#[derive(Subcommand, Debug)]
pub enum CommandAction {
    List(ListCommandsArgs),
    Add(AddCommandArgs),
    Describe(DescribeCommandArgs),
    Remove(RemoveCommandArgs),
}
