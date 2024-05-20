use crate::repos::OwnRepositories;
use crate::service::cmd::CmdService;

pub fn list_commands_handler<R: OwnRepositories>(repos: R) -> i32 {
    let registry = CmdService {
        files: repos.files(),
    };
    let commands = registry.list();
    let count = commands.iter().count();
    if count > 0 {
        println!("{} command(s) found.", count);
        println!("");
        println!("Commands:");
        for command in commands {
            println!("  {}", command);
        }
    } else {
        println!("0 command(s) found.");
    };
    0
}

#[cfg(test)]
mod tests {
    use super::list_commands_handler;
    use crate::repos::MockRepo;

    #[test]
    fn test_list_commands() {
        let repos = MockRepo {};
        let status = list_commands_handler(repos);
        assert!(status == 0);
    }
}
