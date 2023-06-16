use crate::repository::files::FilesRepository;
use crate::cli::commands::ListCommandsArgs;
use crate::service::cmd::CmdService;

pub fn list_commands<R: FilesRepository>(files: R, _: ListCommandsArgs) {
    let registry = CmdService { files };
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
}
