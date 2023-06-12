use clap::{Args, ArgAction};

#[derive(Debug, Args)]
pub struct ListCommandsArgs {
    #[arg(long, action = ArgAction::Help, help = "Show help messages")]
    help: Option<bool>,
}
