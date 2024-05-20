use crate::repos::OwnRepositories;
use crate::service::cmd::CmdService;

pub fn remove_command_handler<R: OwnRepositories>(repos: R, name: &str) -> i32 {
    let mut registry = CmdService {
        files: repos.files(),
    };
    registry.remove(name);
    0
}

#[cfg(test)]
mod tests {
    use super::remove_command_handler;
    use crate::repos::MockRepo;

    #[test]
    fn test_remove_command() {
        let repos = MockRepo {};
        let status = remove_command_handler(repos, "aa");
        assert!(status == 0);
    }
}
