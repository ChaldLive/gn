use dirs::home_dir;
use std::fs;
use std::path::{PathBuf};

#[derive(Debug)]
pub struct PathEx {
    pub name: String,
    pub current_directory: String,
    pub sub_directories: Vec<PathEx>,
    pub exists: bool
}

impl PathEx {
    pub fn new(current_directory: &str) -> Self {
        let local_curdir = Self::get_path_or_home(current_directory);

        PathEx {
            name: local_curdir.file_name().unwrap().to_str().unwrap().to_string(),
            current_directory: local_curdir.to_str().unwrap().to_string(),
            sub_directories: vec![],
            exists: local_curdir.is_dir()
        }
    }
    pub fn scan_directory(&mut self){
        if !self.sub_directories.is_empty(){
            self.sub_directories.clear();
        }
        let temp_dir = &PathBuf::from(&self.current_directory);
        self.sub_directories = Self::internal_initsub_dirs(temp_dir);
    }

    fn internal_initsub_dirs(dir: &PathBuf) -> Vec<PathEx> {
        let mut result_dirs: Vec<PathEx> = Vec::new();

        if dir.is_dir() {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.is_dir() {
                            let sub_dir = PathEx::new(path.to_str().unwrap());
                            result_dirs.push(sub_dir);
                        }
                    }
                }
            }
        }
        result_dirs
    }
    fn get_path_or_home(current_path: &str) -> PathBuf {
        if current_path.is_empty() {
            match home_dir() {
                Some(path) => {
                    path
                }
                None => {
                    PathBuf::from("")
                }
            }
        } else {
            if fs::metadata(current_path).map(|meta|meta.is_dir()).unwrap_or(false) {
                PathBuf::from(current_path)
            } else {
                PathBuf::from("")
            }
        }
    }
}

#[cfg(test)]
mod dir_utils_test {
    use super::*;
    #[test]
    fn root_dir_test(){
        let mut path = PathEx::new("");
        path.scan_directory();
        assert_eq!(path.sub_directories.len() > 0, true);

    }
}





