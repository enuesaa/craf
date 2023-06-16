use crate::repository::files::FilesRepository;
use crate::cli::commands::RemoveCommandArgs;
use crate::service::cmd::CmdService;

pub fn remove_command<R: FilesRepository>(files: R, args: RemoveCommandArgs) -> i32 {
    let mut registry = CmdService { files };
    registry.remove(&args.name);
    0
}
