use crate::service::registry::Registry;

pub fn list_commands_handler() {
    let registry = Registry::new();
    let commands = registry.list_commands();
    let count = commands.iter().count();
    if count > 0 {
        println!("{} command(s) found.", count);
        println!("");
        println!("Commands:");
        for command in commands {
            println!("{}", command);
        };
    } else {
        println!("0 command(s) found.");
    };
}
