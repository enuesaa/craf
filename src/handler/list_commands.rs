use crate::service::registry::Registry;

pub fn list_commands_handler() {
    let registry = Registry::new();
    for command in registry.list_commands() {
        println!("{}", command);
    }
}
