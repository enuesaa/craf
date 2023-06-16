use std::str;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use dirs::home_dir;
use std::error::Error;

pub trait FilesRepository {
    /// list filenames
    fn list(&self, path: &str) -> Vec<String>;

    /// create dir
    fn create_dir(&self, path: &str);

    /// create file
    fn create(&self, path: &str, content: &str);

    /// check file is exist
    fn is_exist(&self, path: &str) -> bool;

    /// read file
    fn read(&self, path: &str) -> Result<String, Box<dyn Error>>;

    /// remove file
    fn remove(&self, path: &str);
}

pub struct Files {}
impl Files {
    fn with_homedir(&self, path: &str) -> PathBuf {
        let mut home = home_dir().unwrap();
        home.push(path);
        home
    }
}

impl FilesRepository for Files {
    fn list(&self, path: &str) -> Vec<String> {
        let path = self.with_homedir(path);
        if let Ok(files) = fs::read_dir(path) {
            let list: Vec<String> = files
                .map(|p| {
                    p.unwrap()
                        .path()
                        .file_name()
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

    fn create_dir(&self, path: &str) {
        let path = self.with_homedir(path);
        let _ = fs::create_dir_all(path);
    }

    fn create(&self, path: &str, content: &str) {
        let path = self.with_homedir(path);
        if let Ok(mut file) = File::create(&path) {
            let _ = file.write_all(content.as_bytes());
        };
    }

    fn is_exist(&self, path: &str) -> bool {
        let path = self.with_homedir(path);
        path.exists()
    }

    fn read(&self, path: &str) -> Result<String, Box<dyn Error>> {
        let path = self.with_homedir(path);
        match fs::read(path) {
            Ok(buf) => Ok(str::from_utf8(&buf).unwrap().to_string()),
            Err(e) => Err(Box::new(e)),
        }
    }

    fn remove(&self, path: &str) {
        let path = self.with_homedir(path);
        let _ = fs::remove_file(path);
    }

}