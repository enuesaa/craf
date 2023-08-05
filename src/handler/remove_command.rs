use crate::repos::OwnRepositories;
use crate::cli::commands::RemoveArgs;
use crate::service::cmd::CmdService;

pub fn remove_command_handler<R: OwnRepositories>(repos: R, args: RemoveArgs) -> i32 {
    let mut registry = CmdService { files: repos.files() };
    registry.remove(&args.name);
    0
}

#[cfg(test)]
mod tests {
    use crate::repos::MockRepo;
    use crate::cli::commands::RemoveArgs;
    use super::remove_command_handler;

    #[test]
    fn test_remove_command() {
        let repos = MockRepo {};
        let args = RemoveArgs {
            name: "aa".to_string(),
        };
        let status = remove_command_handler(repos, args);
        assert!(status == 0);
    }
}
