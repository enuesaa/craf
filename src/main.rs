use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "craftant", bin_name = "craftant", long_about = None)]
struct Args {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
#[command(disable_help_subcommand = true)]
enum Action {
    Register {
        #[arg(long = "name", required = true)]
        name: String,
    },
    Unregister {
        #[arg(long = "name", required = true)]
        name: String,
    },
    Apply {
        #[arg(long = "name", required = true)]
        name: String,
        #[arg(long = "approve", required = false, default_value_t = false)]
        approve: bool,
    },
}

fn main() {
    let _args = Args::parse();
}