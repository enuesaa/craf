use crate::repository::files::FilesRepository;
use crate::cli::commands::RemoveCommandArgs;
use crate::service::cmd::CmdService;

pub fn remove_command<R: FilesRepository>(files: R, args: RemoveCommandArgs) {
    let mut registry = CmdService { files };
    registry.remove(&args.name);
}
