use crate::repos::OwnRepositories;
use crate::service::cmd::CmdService;

pub fn describe_command_handler<R: OwnRepositories>(repos: R, name: &str) -> i32 {
    let registry = CmdService {
        files: repos.files(),
    };
    if let Ok(commanddef) = registry.get(name) {
        println!("name: {}", commanddef.name);
        println!("description: {}", commanddef.description);
        println!("command: {}", commanddef.command);
        0
    } else {
        println!("`{}` not found", name);
        1
    }
}

#[cfg(test)]
mod tests {
    use super::describe_command_handler;
    use crate::repos::MockRepo;

    #[test]
    fn test_describe_command() {
        let repos = MockRepo {};
        let status = describe_command_handler(repos, "aa");
        assert!(status == 0);
    }
}
