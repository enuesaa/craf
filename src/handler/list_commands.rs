use std::io::Result;
use crate::service::registry::list_items;

pub fn list_commands() -> Result<()> {
    let commands = list_items();
    println!("{:?}", commands);
    Ok(())
}
