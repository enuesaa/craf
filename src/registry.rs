use std::fs;
use std::io::Result;

pub fn create_registry() -> Result<()> {
  fs::create_dir(".craftant")?;
  Ok(())
}
