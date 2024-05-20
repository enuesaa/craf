use std::process;

use crate::repos::OwnRepositories;
use crate::service::cmd::{Cmd, CmdService};
use inquire::{required, Text};

pub fn add_command_handler<R: OwnRepositories>(repos: R) -> i32 {
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

    let mut registry = CmdService {
        files: repos.files(),
    };

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
