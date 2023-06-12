use crate::service::registry::Registry;

pub fn remove_command_handler(name: &str) {
    let mut registry = Registry::new();
    registry.remove_command(name);
}
