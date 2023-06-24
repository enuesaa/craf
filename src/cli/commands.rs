use clap::Args;

#[derive(Args, Debug)]
pub struct ListArgs {}

#[derive(Args, Debug)]
pub struct DescribeArgs {
    pub name: String,
}

#[derive(Args, Debug)]
pub struct AddArgs {}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    pub name: String,
}
