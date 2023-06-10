use crate::service::registry::{CommandDef, Registry};
use inquire::Text;

pub fn create_command() {
    let name = Text::new("Please enter name.").prompt().unwrap();
    let bin = Text::new("Please enter bin.").prompt().unwrap();
    let args = Text::new("Please enter args.").prompt().unwrap();
    let splitted: Vec<String> = args.split(" ").map(|s| s.to_string()).collect();

    let mut registry = Registry::new();
    let command = CommandDef {
        name: name.clone(),
        bin: bin.clone(),
        description: "a".to_string(),
        args: splitted,
    };
    registry.create_command(command);
}
