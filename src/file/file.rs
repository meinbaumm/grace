use std::fs;
use std::path::Path;

pub struct File<'a> {
    path: &'a Path,
}

impl File<'_> {
    pub fn new(path_str: &str) -> File {
        File {
            path: Path::new(path_str),
        }
    }

    pub fn extension(&self) -> Option<String> {
        self.path.extension().map(|os_str| {
            os_str
                .to_str()
                .map(|str| str.to_string())
                .unwrap_or("".to_string())
        })
    }

    pub fn file_name(&self) -> Option<String> {
        self.path.file_name().map(|os_str| {
            os_str
                .to_str()
                .map(|str| str.to_string())
                .unwrap_or("".to_string())
        })
    }

    pub fn file_stem(&self) -> Option<String> {
        self.path.file_stem().map(|os_str| {
            os_str
                .to_str()
                .map(|str| str.to_string())
                .unwrap_or("".to_string())
        })
    }

    pub fn is_dir(&self) -> bool {
        self.path.is_dir()
    }

    pub fn is_file(&self) -> bool {
        self.path.is_file()
    }

    pub fn exist(&self) -> bool {
        self.path.exists()
    }

    pub fn read_dir(&self) -> Option<fs::ReadDir> {
        if self.is_dir() {
            return Some(fs::read_dir(self.path).unwrap());
        }
        None
    }

    pub fn create_file(&self) -> bool {
        if !self.exist() {
            return fs::File::create(self.path).is_ok();
        }
        false
    }

    pub fn rename_file(&self, new_name: &str) -> bool {
        if self.is_file() {
            let new_path = self.path.with_file_name(new_name);
            return fs::rename(self.path, new_path).is_ok();
        }
        false
    }
}
