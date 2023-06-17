use std::process;

use crate::cli::commands::AddCommandArgs;
use crate::repos::OwnRepositories;
use crate::service::cmd::{Cmd, CmdService};
use inquire::{Text, required};

pub fn add_command<R: OwnRepositories>(repos: R, _: AddCommandArgs) -> i32 {
    let res = Text::new("name:")
        .with_validator(required!("Required"))
        .prompt();
    if res.is_err() {
        return 1;
    };
    let name = res.unwrap();

    let res = Text::new("shell command (like `echo a`):")
        .with_validator(required!("Required"))
        .prompt();
    if res.is_err() {
        return 1;
    };
    let command = res.unwrap();
    
    let res = Text::new("description:")
        .with_validator(required!("Required"))
        .prompt();
    if res.is_err() {
        return 1;
    };
    let description = res.unwrap();

    let mut registry = CmdService { files: repos.files() };

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
    0
}
