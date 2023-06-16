use clap::Args;

#[derive(Args)]
pub struct ListCommandsArgs {}

#[derive(Args)]
pub struct DescribeCommandArgs {
    #[arg(long)]
    pub name: String,
}

#[derive(Args)]
pub struct AddCommandArgs {}

#[derive(Args)]
pub struct RemoveCommandArgs {
    #[arg(long)]
    pub name: String,
}
