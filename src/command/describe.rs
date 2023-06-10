use clap::Args;

#[derive(Debug, Args)]
pub struct DescribeArgs {
    #[arg(long)]
    name: String,
}
