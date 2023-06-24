use crate::repos::OwnRepositories;
use crate::cli::commands::DescribeArgs;
use crate::service::cmd::CmdService;

pub fn describe_command<R: OwnRepositories>(repos: R, args: DescribeArgs) -> i32 {
    let registry = CmdService { files: repos.files() };
    if let Ok(commanddef) = registry.get(&args.name) {
        println!("`{}` found. Command information is below.", &args.name);
        println!("");

        println!("name: {}", commanddef.name);
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
    use super::describe_command;

    #[test]
    fn test_normal() {
        let repos = MockRepo {};
        let args = DescribeArgs {
            name: "aa".to_string(),
        };
        let status = describe_command(repos, args);
        assert!(status == 1);
    }
}
