use std::fs;
use std::io::Result;
use std::path::Path;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;

use dirs::home_dir;

pub fn get_registry_path() -> PathBuf {
    let mut path = home_dir().unwrap();
    path.push(".craftant");
    path.push("commands");
    path
}

pub fn is_registry_exist() -> bool {
    let registry_path = get_registry_path();
    Path::new(&registry_path).exists()
}

pub fn create_registry_dir() {
    let registry_path = get_registry_path();
    let _ = fs::create_dir(registry_path);
}

pub fn create_registry() -> Result<()> {
    fs::create_dir(get_registry_path())?;
    Ok(())
}

pub fn list_items() -> Vec<PathBuf> {
    let registry_path = get_registry_path();
    if let Ok(files) = fs::read_dir(registry_path) {
        let list: Vec<PathBuf> = files
            .map(|p| p.unwrap().path())
            .collect();
        return list;
    };
    vec![]
}

pub fn get_item_path(name: &str) -> PathBuf {
    let mut path = get_registry_path();
    path.push(name.to_string() + ".json");
    path
}

pub fn create_item(name: &str) -> Result<()> {
    // fs::copy(name, get_item_path(name))?;
    if !is_registry_exist() {
        create_registry_dir();
    };

    let path = get_item_path(name);
    if let Ok(mut file) = File::create(&path) {
        let _ = file.write_all(b"{}");
    }
    Ok(())
}

pub fn remove_item(name: &str) -> Result<()> {
    fs::remove_file(get_item_path(name))?;
    Ok(())
}