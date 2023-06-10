use crate::service::registry::Resgietry;

pub fn list_commands() {
    let registry = Resgietry::new();
    for command in registry.list_commands() {
        println!("{}", command);
    }
}
