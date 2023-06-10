use clap::Args;

#[derive(Debug, Args)]
pub struct RemoveCommandArgs {
    #[arg(long)]
    name: String,
}
