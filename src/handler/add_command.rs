use std::process;

use crate::cli::commands::AddCommandArgs;
use crate::repository::files::FilesRepository;
use crate::service::cmd::{Cmd, CmdService};
use inquire::{Text, required};

pub fn add_command<R: FilesRepository>(files: R, _: AddCommandArgs) {
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

    let mut registry = CmdService { files };

    if registry.is_exist(&name) {
        println!("`{}` exists.", name);
        process::exit(1);
    };
    let command = Cmd {
        name: name.clone(),
        command: command.clone(),
        description: description.clone(),
    };
    registry.create(command);
}
