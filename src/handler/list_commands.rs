use crate::repos::OwnRepositories;
use crate::cli::commands::ListCommandsArgs;
use crate::service::cmd::CmdService;

pub fn list_commands<R: OwnRepositories>(repos: R, _: ListCommandsArgs) -> i32 {
    let registry = CmdService { files: repos.files() };
    let commands = registry.list();
    let count = commands.iter().count();
    if count > 0 {
        println!("{} command(s) found.", count);
        println!("");
        println!("Commands:");
        for command in commands {
            println!("{}", command);
        };
    } else {
        println!("0 command(s) found.");
    };
    0
}
