use std::fmt;
use std::fs;
use std::path::Path;
use std::vec::Vec;

#[derive(Debug)]
pub enum FileErr {
    NotADirectory,
    NotAFile,
    UnhandledError(Box<dyn std::error::Error>),
    FileAlreadyExist,
}

impl std::error::Error for FileErr {}

impl fmt::Display for FileErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileErr::NotADirectory => write!(f, "Provided path is not a directory"),
            FileErr::NotAFile => write!(f, "Provided path is not a file"),
            FileErr::UnhandledError(e) => write!(f, "Unhandled error: {}", e),
            FileErr::FileAlreadyExist => write!(f, "File already exist"),
        }
    }
}

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

    pub fn read_dir(&self) -> Result<Vec<String>, FileErr> {
        if !self.is_dir() {
            return Err(FileErr::NotADirectory);
        }

        let mut files_in_dir = Vec::new();
        let paths = fs::read_dir(self.path).unwrap();

        for path in paths {
            match path {
                Ok(path) => {
                    let file_name = path.file_name().into_string().unwrap();
                    files_in_dir.push(file_name);
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        return Ok(files_in_dir);
    }

    pub fn create_file(&self) -> Result<(), FileErr> {
        if self.exist() {
            return Err(FileErr::FileAlreadyExist);
        }

        match fs::File::create(self.path) {
            Ok(_) => Ok(()),
            Err(err) => Err(FileErr::UnhandledError(Box::new(err))),
        }
    }

    pub fn rename_file(&self, new_name: &str) -> Result<(), FileErr> {
        if !self.is_file() {
            return Err(FileErr::NotAFile);
        }

        let new_path = self.path.with_file_name(new_name);
        match fs::rename(self.path, new_path) {
            Ok(_) => Ok(()),
            Err(err) => Err(FileErr::UnhandledError(Box::new(err))),
        }
    }
}
