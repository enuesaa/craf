use std::fs;
use std::io::Result;

use dirs::home_dir;

pub fn create_registry() -> Result<()> {
  let mut registry_path = home_dir().unwrap();
  registry_path.push(".craftant");
  println!("{:?}", registry_path);
  fs::create_dir(registry_path)?;
  Ok(())
}
