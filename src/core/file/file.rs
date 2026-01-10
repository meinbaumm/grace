use std::fs;
use std::path::Path;
use std::vec::Vec;
use thiserror::Error;

/// Enum to represent the different errors that can occur when working with files.
#[derive(Error, Debug, PartialEq)]
pub enum FileErr {
    #[error("Provided path is not a directory")]
    NotADirectory,
    #[error("Provided path is not a file")]
    NotAFile,
    #[error("File already exists")]
    FileAlreadyExist,
    #[error("File does not exist")]
    FileDoesNotExist,
    #[error("Directory does not exist")]
    DirectoryDoesNotExist,
    #[error("Failed to read directory: {0}")]
    ReadDirError(String),
    #[error("Failed to rename file: {0}")]
    RenameError(String),
    #[error("Failed to create file: {0}")]
    CreateFileError(String),
    #[error("I/O error: {0}")]
    IoError(String),
}

/// Struct to represent a file or directory.
/// This struct is used to abstract away the underlying file system.
pub struct File<'a> {
    path: &'a Path,
}

impl File<'_> {
    /// Create a new file.
    pub fn new(path_str: &str) -> File<'_> {
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

        let paths = fs::read_dir(self.path).map_err(|e| FileErr::ReadDirError(e.to_string()))?;

        let files_in_dir: Result<Vec<String>, FileErr> = paths
            .map(|entry| {
                entry
                    .map_err(|e| FileErr::ReadDirError(e.to_string()))
                    .and_then(|e| {
                        e.file_name()
                            .into_string()
                            .map_err(|_| FileErr::IoError("Invalid UTF-8 in filename".to_string()))
                    })
            })
            .collect();

        files_in_dir
    }

    /// Create a file. If the file already exist, an error `FileErr::FileAlreadyExist` will be returned.
    pub fn create_file(&self) -> Result<(), FileErr> {
        if self.exist() {
            return Err(FileErr::FileAlreadyExist);
        }

        fs::File::create(self.path)
            .map(|_| ())
            .map_err(|e| FileErr::CreateFileError(e.to_string()))
    }

    /// Rename file or directory.
    pub fn rename(&self, new_name: &str) -> Result<(), FileErr> {
        let new_path = self.path.with_file_name(new_name);
        fs::rename(self.path, new_path).map_err(|e| FileErr::RenameError(e.to_string()))
    }

    pub fn path(&self) -> String {
        self.path.to_str().unwrap().to_string()
    }
}
