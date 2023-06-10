use std::io::Result;
use crate::service::registry::create_item;

pub fn create_command(name: String) -> Result<()> {
    println!("{:?}", name);
    create_item(&name);
    Ok(())
}