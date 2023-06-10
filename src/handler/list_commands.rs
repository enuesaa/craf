use crate::service::registry::Registry;

pub fn list_commands() {
    let registry = Registry::new();
    for command in registry.list_commands() {
        println!("{}", command);
    }
}
