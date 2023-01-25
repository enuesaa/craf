use std::fs;
use std::io::Result;
use std::path::PathBuf;

use dirs::home_dir;

pub fn get_registry_path() -> PathBuf {
  let mut path = home_dir().unwrap();
  path.push(".craftant");
  println!("registry path: {:?}", path);
  path
}

pub fn get_template_path(name: &str) -> PathBuf {
  let mut path = get_registry_path();
  path.push(name);
  path
}

pub fn create_registry() -> Result<()> {
  fs::create_dir(get_registry_path())?;
  Ok(())
}

pub fn add_template(name: &str) -> Result<()> {
  fs::copy(name, get_template_path(name))?;
  Ok(())
}