use clap::Args;

#[derive(Debug, Args)]
pub struct DescribeCommandArgs {
    #[arg(long)]
    name: String,
}
