use crate::registry::get_template_path;

use std::fs;
use std::io::Result;

pub fn apply_template(name: &str) -> Result<()> {
    fs::copy(get_template_path(name), name)?;
    Ok(())
}
