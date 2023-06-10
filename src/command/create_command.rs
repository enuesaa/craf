use clap::Args;

#[derive(Debug, Args)]
pub struct CreateCommandArgs {
    #[arg(long)]
    pub name: String, // temp. this is for development.
}
