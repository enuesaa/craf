use std::str;
use serde::{Deserialize, Serialize};
use crate::repository::files::FilesRepository;

#[derive(Serialize, Deserialize, Debug)]
pub struct Cmd {
    pub name: String,
    pub description: String,
    pub command: String,
}

#[derive(Debug, Clone)]
pub struct CmdNotFoundError;

pub struct CmdService<R: FilesRepository> {
    pub files: R,
}
impl<R: FilesRepository> CmdService<R> {
    fn get_registry_path(&self) -> String {
        String::from(".crafant/commands")
    }

    fn is_registry_exist(&self) -> bool {
        self.files.is_exist(&self.get_registry_path())
    }

    fn create_registry(&self) {
        self.files.create_dir(&self.get_registry_path());
    }

    pub fn get_path(&self, name: &str) -> String {
        format!("{}/{}.json", &self.get_registry_path(), name)
    }

    pub fn list(&self) -> Vec<String> {
        let list = self.files.list(&self.get_registry_path());
        list.iter().map(|p| { p.trim_end_matches(".json").to_string() }).collect()
    }

    pub fn create(&mut self, cmd: Cmd) {
        if !self.is_registry_exist() {
            self.create_registry();
        };

        let path = self.get_path(&cmd.name);
        let content = serde_json::to_string(&cmd).unwrap();
        self.files.create(&path, &content);
    }

    pub fn get(&self, name: &str) -> Result<Cmd, CmdNotFoundError> {
        let path = self.get_path(name);
        if let Ok(content) = self.files.read(&path) {
            if let Ok(cmd) = serde_json::from_str(&content) {
                return Ok(cmd)
            };
        };
        Err(CmdNotFoundError {})
    }

    pub fn is_exist(&self, name: &str) -> bool {
        self.get(name).is_ok()
    }

    pub fn remove(&mut self, name: &str) {
        let path = self.get_path(name);
        self.files.remove(&path);
    }
}
