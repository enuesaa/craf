use serde_json;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use std::str;

use dirs::home_dir;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CommandDef {
    pub name: String,
    pub description: String,
    pub bin: String,
    pub args: Vec<String>,
}

pub struct Registry {}
impl Registry {
    pub fn new() -> Self {
        Self {}
    }

    fn get_path(&self) -> PathBuf {
        let mut path = home_dir().unwrap();
        path.push(".craftant");
        path.push("commands");
        path
    }

    fn is_exist(&self) -> bool {
        let registry_path = self.get_path();
        Path::new(&registry_path).exists()
    }

    fn create(&self) {
        let registry_path = self.get_path();
        let _ = fs::create_dir(registry_path);
    }

    fn get_command_path(&self, name: &str) -> String {
        let mut path = self.get_path();
        path.push(name.to_string() + ".json");
        path.display().to_string()
    }

    pub fn list_commands(&self) -> Vec<String> {
        let registry_path = self.get_path();
        if let Ok(files) = fs::read_dir(registry_path) {
            let list: Vec<String> = files
                .map(|p| {
                    p.unwrap()
                        .path()
                        .file_stem()
                        .unwrap()
                        .to_os_string()
                        .into_string()
                        .unwrap()
                })
                .collect();
            return list;
        };
        vec![]
    }

    pub fn create_command(&mut self, command: CommandDef) {
        if !self.is_exist() {
            self.create();
        };

        let path = self.get_command_path(&command.name);
        if let Ok(mut file) = File::create(&path) {
            let _ = file.write_all(serde_json::to_string(&command).unwrap().as_bytes());
        };
    }

    pub fn get_command(&self, name: &str) -> CommandDef {
        let path = self.get_command_path(name);
        let buf = fs::read(path).unwrap();
        let commanddef = serde_json::from_str(str::from_utf8(&buf).unwrap());
        commanddef.unwrap()
    }

    pub fn remove_command(&mut self, name: &str) {
        let path = self.get_command_path(name);
        let _ = fs::remove_file(path);
    }
}
