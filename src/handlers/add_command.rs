use std::process;

use crate::cli::commands::AddCommandArgs;
use crate::services::registry::{CommandDef, Registry};
use inquire::{Text, required};

pub fn add_command(_: AddCommandArgs) {
    let name = Text::new("name:")
        .with_validator(required!("Required"))
        .prompt()
        .unwrap_or_else(|_| { process::exit(1); });
    let command = Text::new("shell command (like `echo a`):")
        .with_validator(required!("Required"))
        .prompt()
        .unwrap_or_else(|_| { process::exit(1); });
    let description = Text::new("description:")
        .prompt()
        .unwrap_or_else(|_| { process::exit(1); });

    let mut registry = Registry::new();

    if registry.is_exist_command(&name) {
        println!("`{}` exists.", name);
        process::exit(1);
    };
    let command = CommandDef {
        name: name.clone(),
        command: command.clone(),
        description: description.clone(),
    };
    registry.create_command(command);
}
