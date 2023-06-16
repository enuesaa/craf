use crate::services::registry::Registry;
use crate::cli::commands::RemoveCommandArgs;

pub fn remove_command(args: RemoveCommandArgs) {
    let mut registry = Registry::new();
    registry.remove_command(&args.name);
}
