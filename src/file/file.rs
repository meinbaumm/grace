use std::path::Path;
use std::ffi::OsStr;

pub struct File<'a> {
    path: &'a Path,
}

impl File<'_> {
    pub fn new(path_str: &str) -> File {
        File { 
            path: Path::new(path_str),
        }
    }

    pub fn extension(&self) -> Option<&OsStr> {
        self.path
            .extension()
    }

    pub fn file_name(&self) -> Option<&OsStr> {
        self.path
            .file_name()
    }

    pub fn file_stem(&self) -> Option<&OsStr> {
        self.path.file_stem()
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
}
