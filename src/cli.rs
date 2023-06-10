use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "craftant", bin_name = "craftant", about = "Command Proxy.")]
#[clap(disable_help_subcommand = false, disable_help_flag = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    List {},
    Register {},
    Info {},
    Unregister {},
    Run {},
}
