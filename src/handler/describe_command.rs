use crate::repository::files::FilesRepository;
use crate::cli::commands::DescribeCommandArgs;
use crate::service::cmd::CmdService;

pub fn describe_command<R: FilesRepository>(files: R, args: DescribeCommandArgs) -> i32 {
    let registry = CmdService { files };
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
    use crate::cli::commands::DescribeCommandArgs;
    use crate::repository::files::FilesRepository;
    use super::describe_command;
    use std::error::Error;

    struct FilesMock {}
    impl FilesRepository for FilesMock {
        fn list(&self, _: &str) -> Vec<String> {
            todo!()
        }
        fn create_dir(&self, _: &str) {
            todo!()
        }
        fn create(&self, _: &str, _: &str) {
            todo!()
        }
        fn is_exist(&self, _: &str) -> bool {
            todo!()
        }
        fn read(&self, _: &str) -> Result<String, Box<dyn Error>> {
            // {\"name\":\"aa\",\"description\":\"aa\",\"command\":\"echo a\"}
            Ok("aa".to_string())
        }
        fn remove(&self, _: &str) {
            todo!()
        }
    }

    #[test]
    fn test_normal() {
        let files = FilesMock {};
        let args = DescribeCommandArgs {
            name: "aa".to_string(),
        };
        let status = describe_command(files, args);
        assert!(status == 1);
    }
}