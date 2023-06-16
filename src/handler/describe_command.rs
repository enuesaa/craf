use crate::repository::files::FilesRepository;
use crate::cli::commands::DescribeCommandArgs;
use crate::service::cmd::CmdService;

pub fn describe_command<R: FilesRepository>(files: R, args: DescribeCommandArgs) {
    let registry = CmdService { files };
    if let Ok(commanddef) = registry.get(&args.name) {
        println!("`{}` found. Command information is below.", &args.name);
        println!("");

        println!("name: {}", commanddef.name);
        println!("command: {}", commanddef.command);
    } else {
        println!("`{}` not found", &args.name);
    };
}
