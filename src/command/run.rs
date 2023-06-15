use clap::{Args, ArgAction};

#[derive(Debug, Args)]
pub struct RunArgs {
    pub name: String,

    #[arg(long, action = ArgAction::Help, help = "Show help messages")]
    help: Option<bool>,
}
