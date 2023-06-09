use clap::{Parser, Subcommand};

pub mod run;
pub mod tokiorun;

// use crate::run::run;
use crate::tokiorun::tokiorun;

#[derive(Parser)]
#[command(name = "craftant", bin_name = "craftant", long_about = None)]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
#[command(disable_help_subcommand = true)]
enum Action {
    Run {},
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.action {
        Action::Run {} => {
            println!("run");
            let _ = tokiorun().await;
        }
    }
}
