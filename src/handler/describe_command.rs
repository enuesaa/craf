use crate::service::registry::Registry;

pub fn describe_command_handler(name: &str) {
    let registry = Registry::new();
    if let Ok(commanddef) = registry.get_command(name) {
        println!("`{}` found. Command information is below.", name);
        println!("");

        println!("name: {}", commanddef.name);
        println!("bin: {}", commanddef.bin);
        println!("args: {:?}", commanddef.args);
    } else {
        println!("`{}` not found", name);
    };
}
