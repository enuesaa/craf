use clap::{Parser, Subcommand};

pub mod apply;
pub mod registry;

use crate::apply::apply_template;
use crate::registry::{add_template, create_registry, remove_template};

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
    List {},
}

fn main() {
    let cli = Cli::parse();
    match cli.action {
        Action::Template(args) => match args {
            TemplateAction::Add { name } => {
                println!("template add: {:?}", name);
                let _ = create_registry();
                let _ = add_template(&name);
            }
            TemplateAction::Remove { name } => {
                println!("template remove: {:?}", name);
                let _ = remove_template(&name);
            }
            TemplateAction::List {} => {}
        },
        Action::Apply { name, approve } => {
            println!("apply: {:?}, {:?}", name, approve);
            let _ = apply_template(&name);
        }
    }
}
