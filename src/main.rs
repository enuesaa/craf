use clap::{Parser, Subcommand};

pub mod run;

use crate::run::run;

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

fn main() {
    let cli = Cli::parse();
    match cli.action {
        Action::Run {} => {
            println!("run");
            let _ = run();
        }
    }
}
