use std::fmt;
use std::fs;
use std::path::Path;
use std::vec::Vec;

/// Enum to represent the different errors that can occur when working with files.
#[derive(Debug, PartialEq)]
pub enum FileErr {
    NotADirectory,
    NotAFile,
    UnhandledError,
    FileAlreadyExist,
    FileDoesNotExist,
}

impl std::error::Error for FileErr {}

impl fmt::Display for FileErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileErr::NotADirectory => write!(f, "Provided path is not a directory"),
            FileErr::NotAFile => write!(f, "Provided path is not a file"),
            FileErr::UnhandledError => write!(f, "Unhandled error"),
            FileErr::FileAlreadyExist => write!(f, "File already exist"),
            FileErr::FileDoesNotExist => write!(f, "File does not exist"),
        }
    }
}

/// Struct to represent a file or directory.
/// This struct is used to abstract away the underlying file system.
pub struct File<'a> {
    path: &'a Path,
}

impl File<'_> {
    /// Create a new file.
    pub fn new(path_str: &str) -> File {
        File {
            path: Path::new(path_str),
        }
    }

    /// Return the file extension.
    pub fn extension(&self) -> Option<String> {
        self.path.extension().map(|os_str| {
            os_str
                .to_str()
                .map(|str| str.to_string())
                .unwrap_or("".to_string())
        })
    }

    /// Return the file name.
    pub fn file_name(&self) -> Option<String> {
        self.path.file_name().map(|os_str| {
            os_str
                .to_str()
                .map(|str| str.to_string())
                .unwrap_or("".to_string())
        })
    }

    /// Return the file stem.
    pub fn file_stem(&self) -> Option<String> {
        self.path.file_stem().map(|os_str| {
            os_str
                .to_str()
                .map(|str| str.to_string())
                .unwrap_or("".to_string())
        })
    }

    /// Check if the file is a directory.
    pub fn is_dir(&self) -> bool {
        self.path.is_dir()
    }

    /// Check if the file is a file.
    pub fn is_file(&self) -> bool {
        self.path.is_file()
    }

    /// Check if the file exist.
    pub fn exist(&self) -> bool {
        self.path.exists()
    }

    /// If `File` is a directory, return a vector of the files in the directory.
    /// If `File` is a file, return an error `FileErr::NotADirectory`.
    pub fn read_dir(&self) -> Result<Vec<String>, FileErr> {
        if !self.is_dir() {
            return Err(FileErr::NotADirectory);
        }

        let paths = fs::read_dir(self.path).unwrap();

        let files_in_dir: Vec<String> = paths
            .map(|path| path.unwrap().file_name().into_string().unwrap())
            .collect();

        Ok(files_in_dir)
    }

    /// Create a file. If the file already exist, an error `FileErr::FileAlreadyExist` will be returned.
    pub fn create_file(&self) -> Result<(), FileErr> {
        if self.exist() {
            return Err(FileErr::FileAlreadyExist);
        }

        match fs::File::create(self.path) {
            Ok(_) => Ok(()),
            Err(_) => Err(FileErr::UnhandledError),
        }
    }

    /// Rename file. If you try to rename a directory, an error `FileErr::NotAFile` will be returned.
    pub fn rename_file(&self, new_name: &str) -> Result<(), FileErr> {
        if !self.is_file() {
            return Err(FileErr::NotAFile);
        }

        let new_path = self.path.with_file_name(new_name);
        match fs::rename(self.path, new_path) {
            Ok(_) => Ok(()),
            Err(_) => Err(FileErr::UnhandledError),
        }
    }

    pub fn path(&self) -> String {
        self.path.to_str().unwrap().to_string()
    }
}
