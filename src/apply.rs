use crate::registry::get_template_path;

use std::io::Result;
use std::fs;

pub fn apply_template(name: &str) -> Result<()> {
  fs::copy(get_template_path(name), name)?;
  Ok(())
}
