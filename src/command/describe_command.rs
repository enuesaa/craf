use clap::{Args, ArgAction};

#[derive(Debug, Args)]
pub struct DescribeCommandArgs {
    #[arg(long)]
    name: String,

    #[arg(long, action = ArgAction::Help, help = "Show help messages")]
    help: Option<bool>,
}
