use clap::{Parser, Subcommand};

pub mod registry;
pub mod apply;

use crate::registry::{create_registry, add_template, remove_template};
use crate::apply::{apply_template};

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
    let cli = Cli::parse();
    match cli.action {
        Action::Template (args) => match args {
            TemplateAction::Add { name } => {
                println!("template add: {:?}", name);
                let _ = create_registry();
                let _ = add_template(&name);
            },
            TemplateAction::Remove { name } => {
                println!("template remove: {:?}", name);
                let _ = remove_template(&name);
            },
        },
        Action::Apply { name, approve } => {
            println!("apply: {:?}, {:?}", name, approve);
            let _ = apply_template(&name);
        },
    }
}