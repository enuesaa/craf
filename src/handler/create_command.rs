use crate::service::registry::{CommandDef, Resgietry};

pub fn create_command(name: String) {
    let mut registry = Resgietry::new();
    let command = CommandDef {
        name: name.clone(),
        bin: "a".to_string(),
        description: "a".to_string(),
        args: vec![],
    };
    registry.create_command(command);
}
