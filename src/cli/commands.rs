use clap::Args;

#[derive(Args, Debug)]
pub struct ListCommandsArgs {}

#[derive(Args, Debug)]
pub struct DescribeCommandArgs {
    #[arg(long)]
    pub name: String,
}

#[derive(Args, Debug)]
pub struct AddCommandArgs {}

#[derive(Args, Debug)]
pub struct RemoveCommandArgs {
    #[arg(long)]
    pub name: String,
}
