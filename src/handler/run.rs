use crate::repos::OwnRepositories;
use crate::service::cmd::CmdService;
use std::process::Command;

/**
 * see https://keens.github.io/blog/2016/12/02/rustnopurosesu/
 * see https://www.howtosolutions.net/2021/10/rust-creating-simple-terminal-interactive-input-prompt-app/
 * see https://stackoverflow.com/questions/72750736/run-command-stream-stdout-stderr-and-capture-results
 * see https://stackoverflow.com/questions/66060139/how-to-tee-stdout-stderr-from-a-subprocess-in-rust
 */
pub fn run_handler<R: OwnRepositories>(repos: R, name: &str) -> i32 {
    let registry = CmdService {
        files: repos.files(),
    };
    if let Ok(commanddef) = registry.get(name) {
        println!("Run following command..");
        println!("  {}", commanddef.command);
        println!("");
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(commanddef.command)
            .spawn()
            .unwrap();

        let status = child.wait().unwrap();
        println!("");
        println!("Command completed with status {}", status.code().unwrap());
        0
    } else {
        println!("");
        println!("Command not found.");
        1
    }
}

#[cfg(test)]
mod tests {
    use super::run_handler;
    use crate::repos::MockRepo;

    #[test]
    fn test_run() {
        let repos = MockRepo {};
        // TODO: create shell repository and do not run a command on test.
        let status = run_handler(repos, "aa");
        assert!(status == 0);
    }
}
