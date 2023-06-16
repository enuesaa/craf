use crate::services::registry::Registry;
use crate::cli::commands::DescribeCommandArgs;

pub fn describe_command(args: DescribeCommandArgs) {
    let registry = Registry::new();
    if let Ok(commanddef) = registry.get_command(&args.name) {
        println!("`{}` found. Command information is below.", &args.name);
        println!("");

        println!("name: {}", commanddef.name);
        println!("command: {}", commanddef.command);
    } else {
        println!("`{}` not found", &args.name);
    };
}
