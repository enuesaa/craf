use clap::{Args, ArgAction};

#[derive(Debug, Args)]
pub struct RemoveCommandArgs {
    #[arg(long)]
    pub name: String,

    #[arg(long, action = ArgAction::Help, help = "Show help messages")]
    help: Option<bool>,
}
