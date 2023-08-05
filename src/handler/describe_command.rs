use crate::repos::OwnRepositories;
use crate::cli::commands::DescribeArgs;
use crate::service::cmd::CmdService;

pub fn describe_command_handler<R: OwnRepositories>(repos: R, args: DescribeArgs) -> i32 {
    let registry = CmdService { files: repos.files() };
    if let Ok(commanddef) = registry.get(&args.name) {
        println!("name: {}", commanddef.name);
        println!("description: {}", commanddef.description);
        println!("command: {}", commanddef.command);
        0
    } else {
        println!("`{}` not found", &args.name);
        1
    }
}

#[cfg(test)]
mod tests {
    use crate::repos::MockRepo;
    use crate::cli::commands::DescribeArgs;
    use super::describe_command_handler;

    #[test]
    fn test_describe_command() {
        let repos = MockRepo {};
        let args = DescribeArgs {
            name: "aa".to_string(),
        };
        let status = describe_command_handler(repos, args);
        assert!(status == 0);
    }
}
