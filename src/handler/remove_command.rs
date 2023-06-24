use crate::repos::OwnRepositories;
use crate::cli::commands::RemoveArgs;
use crate::service::cmd::CmdService;

pub fn remove_command<R: OwnRepositories>(repos: R, args: RemoveArgs) -> i32 {
    let mut registry = CmdService { files: repos.files() };
    registry.remove(&args.name);
    0
}
