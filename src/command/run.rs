use clap::Args;

#[derive(Debug, Args)]
pub struct RunArgs {
    #[arg(long)]
    pub name: String, // temp. this is for development.
}
