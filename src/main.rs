use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "craftant", bin_name = "craftant", long_about = None)]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
#[command(disable_help_subcommand = true)]
enum Action {
    #[command(subcommand)]
    Template(TemplateAction),
    Apply {
        #[arg(long = "name", required = true)]
        name: String,
        #[arg(long = "approve", required = false, default_value_t = false)]
        approve: bool,
    },
}

#[derive(Subcommand)]
enum TemplateAction {
    Add {
        #[arg(long = "name", required = true)]
        name: String,
    },
    Remove {
        #[arg(long = "name", required = true)]
        name: String,
    },
}

fn main() {
    let _args = Cli::parse();
}