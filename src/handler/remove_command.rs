use crate::repos::OwnRepositories;
use crate::cli::commands::RemoveCommandArgs;
use crate::service::cmd::CmdService;

pub fn remove_command<R: OwnRepositories>(repos: R, args: RemoveCommandArgs) -> i32 {
    let mut registry = CmdService { files: repos.files() };
    registry.remove(&args.name);
    0
}
