use clap::{Parser, Subcommand};
use crate::command::register::ResgiterArgs;
use crate::command::info::InfoArgs;
use crate::command::unregister::UnresgiterArgs;
use crate::command::list::ListArgs;
use crate::command::run::RunArgs;

#[derive(Parser)]
#[command(name = "craftant", bin_name = "craftant", about = "Command Proxy.")]
#[clap(disable_help_subcommand = false, disable_help_flag = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    List(ListArgs),
    Register(ResgiterArgs),
    Info(InfoArgs),
    Unregister(UnresgiterArgs),
    Run(RunArgs),
}
