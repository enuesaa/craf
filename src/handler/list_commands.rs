use crate::repos::OwnRepositories;
use crate::cli::commands::ListArgs;
use crate::service::cmd::CmdService;

pub fn list_commands<R: OwnRepositories>(repos: R, _: ListArgs) -> i32 {
    let registry = CmdService { files: repos.files() };
    let commands = registry.list();
    let count = commands.iter().count();
    if count > 0 {
        println!("{} command(s) found.", count);
        println!("");
        println!("Commands:");
        for command in commands {
            println!("  {}", command);
        };
    } else {
        println!("0 command(s) found.");
    };
    0
}

#[cfg(test)]
mod tests {
    use crate::repos::MockRepo;
    use crate::cli::commands::ListArgs;
    use super::list_commands;

    #[test]
    fn test_list_commands() {
        let repos = MockRepo {};
        let args = ListArgs {};
        let status = list_commands(repos, args);
        assert!(status == 0);
    }
}
