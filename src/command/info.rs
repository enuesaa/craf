use clap::Args;

#[derive(Debug, Args)]
pub struct InfoArgs {
    #[arg(long)]
    name: String,
}
