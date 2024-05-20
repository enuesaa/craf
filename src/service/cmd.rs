use crate::repository::files::FilesRepository;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cmd {
    pub name: String,
    pub description: String,
    pub command: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Cmddef {
    pub commands: Vec<Cmd>,
}

#[derive(Debug, Clone)]
pub struct CmdNotFoundError;
#[derive(Debug, Clone)]
pub struct CmddefNotFoundError;

pub struct CmdService<R: FilesRepository> {
    pub files: R,
}
impl<R: FilesRepository> CmdService<R> {
    fn get_registry_path(&self) -> String {
        String::from(".craf")
    }

    fn is_registry_exist(&self) -> bool {
        self.files.is_exist(&self.get_registry_path())
    }

    fn create_registry(&self) {
        self.files.create_dir(&self.get_registry_path());
    }

    fn get_cmddef_path(&self) -> String {
        format!("{}/commands.json", &self.get_registry_path())
    }

    fn read_cmddef(&self) -> Result<Cmddef, CmddefNotFoundError> {
        if let Ok(content) = self.files.read(&self.get_cmddef_path()) {
            if let Ok(cmddef) = serde_json::from_str(&content) {
                return Ok(cmddef);
            }
        };
        Err(CmddefNotFoundError)
    }

    fn put_cmddef(&self, cmddef: Cmddef) {
        let content = serde_json::to_string(&cmddef).unwrap();
        self.files.create(&self.get_cmddef_path(), &content);
    }

    pub fn list(&self) -> Vec<String> {
        if let Ok(cmddef) = self.read_cmddef() {
            return cmddef.commands.iter().map(|c| c.name.clone()).collect();
        };
        vec![]
    }

    pub fn create(&mut self, cmd: Cmd) {
        if !self.is_registry_exist() {
            self.create_registry();
        };

        if let Ok(mut cmddef) = self.read_cmddef() {
            cmddef.commands.push(cmd);
            self.put_cmddef(cmddef);
        } else {
            let mut cmddef = Cmddef {
                commands: Vec::new(),
            };
            cmddef.commands.push(cmd);
            self.put_cmddef(cmddef);
        };
    }

    pub fn get(&self, name: &str) -> Result<Cmd, CmdNotFoundError> {
        if let Ok(cmddef) = self.read_cmddef() {
            if let Some(cmd) = cmddef.commands.iter().find(|&c| c.name == name) {
                return Ok(cmd.clone());
            }
        };
        Err(CmdNotFoundError {})
    }

    pub fn is_exist(&self, name: &str) -> bool {
        self.get(name).is_ok()
    }

    pub fn remove(&mut self, name: &str) {
        if let Ok(mut cmddef) = self.read_cmddef() {
            cmddef.commands = cmddef
                .commands
                .iter()
                .filter_map(|c| {
                    if c.name == name {
                        return None;
                    };
                    return Some(c.clone());
                })
                .collect();
            self.put_cmddef(cmddef);
        };
    }
}
