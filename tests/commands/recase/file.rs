use grace_cli::arguments::into::Into;
use grace_cli::commands::recase::file::{
    extract_file_name_and_extension, recase_directory, recase_file,
};
use grace_cli::core::file::file::{File, FileErr};
use serial_test::serial;
use std::fs;

#[test]
fn test_extract_file_name_and_extension() {
    let file = File::new("test_file.txt");
    let (name, ext) = extract_file_name_and_extension(&file);
    assert_eq!(name, "test_file");
    assert_eq!(ext, "txt");

    let file = File::new("document.pdf");
    let (name, ext) = extract_file_name_and_extension(&file);
    assert_eq!(name, "document");
    assert_eq!(ext, "pdf");

    let file = File::new("no_extension");
    let (name, ext) = extract_file_name_and_extension(&file);
    assert_eq!(name, "no_extension");
    assert_eq!(ext, "");
}

#[test]
#[serial]
fn test_recase_file_success() {
    // Create a test file with PascalCase name
    let test_file = "tests/commands/recase/TestFileHelloWorld.txt";
    fs::write(test_file, "test content").unwrap();

    // Recase the file to snake_case
    let result = recase_file(Some(test_file.to_string()), &Into::Snake, &false);
    assert!(result.is_ok());

    // Check that the old file doesn't exist and new file exists
    assert!(!std::path::Path::new(test_file).exists());
    let new_file = "tests/commands/recase/test_file_hello_world.txt";
    assert!(std::path::Path::new(new_file).exists());

    // Clean up - remove the renamed file
    let _ = fs::remove_file(new_file);
}

#[test]
fn test_recase_file_not_found() {
    let result = recase_file(
        Some("tests/commands/recase/nonexistent.txt".to_string()),
        &Into::Snake,
        &false,
    );
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), FileErr::FileDoesNotExist);
}

#[test]
fn test_recase_file_none_input() {
    let result = recase_file(None, &Into::Snake, &false);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), FileErr::FileDoesNotExist);
}

#[test]
fn test_recase_file_is_directory() {
    let result = recase_file(
        Some("tests/commands/recase".to_string()),
        &Into::Snake,
        &false,
    );
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), FileErr::NotAFile);
}

#[test]
#[serial]
fn test_recase_file_with_sanitize() {
    // Create a test file with special characters in PascalCase
    let test_file = "tests/commands/recase/Test@File#Special.txt";
    fs::write(test_file, "test content").unwrap();

    let result = recase_file(Some(test_file.to_string()), &Into::Snake, &true);
    assert!(result.is_ok());

    // The file should have been renamed (special chars removed and snake_cased)
    assert!(!std::path::Path::new(test_file).exists());

    // Clean up - find and remove the renamed file
    if let Ok(entries) = fs::read_dir("tests/commands/recase") {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name() {
                let name_str = name.to_string_lossy();
                if name_str.contains("test")
                    && name_str.contains("file")
                    && name_str.contains("special")
                {
                    let _ = fs::remove_file(path);
                }
            }
        }
    }
}

#[test]
#[serial]
fn test_recase_directory_success() {
    // Create a test directory
    let test_dir = "tests/commands/recase/TestDirectory";
    fs::create_dir_all(test_dir).unwrap();

    let result = recase_directory(Some(test_dir.to_string()), &Into::Snake, &false);
    assert!(result.is_ok());

    // Clean up - remove the renamed directory
    if let Ok(entries) = fs::read_dir("tests/commands/recase") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && path.file_name().unwrap().to_string_lossy().contains("test") {
                let _ = fs::remove_dir_all(path);
            }
        }
    }
}

#[test]
fn test_recase_directory_not_found() {
    let result = recase_directory(
        Some("tests/commands/recase/nonexistent_dir".to_string()),
        &Into::Snake,
        &false,
    );
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), FileErr::DirectoryDoesNotExist);
}

#[test]
fn test_recase_directory_none_input() {
    let result = recase_directory(None, &Into::Snake, &false);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), FileErr::DirectoryDoesNotExist);
}

#[test]
fn test_recase_directory_is_file() {
    // Create a test file
    let test_file = "tests/commands/recase/just_a_file.txt";
    fs::write(test_file, "content").unwrap();

    let result = recase_directory(Some(test_file.to_string()), &Into::Snake, &false);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), FileErr::NotADirectory);

    // Clean up
    let _ = fs::remove_file(test_file);
}

#[test]
#[serial]
fn test_recase_directory_with_sanitize() {
    // Create a directory with special characters
    let test_dir = "tests/commands/recase/Test@Directory";
    fs::create_dir_all(test_dir).unwrap();

    let result = recase_directory(Some(test_dir.to_string()), &Into::Snake, &true);
    assert!(result.is_ok());

    // Clean up
    if let Ok(entries) = fs::read_dir("tests/commands/recase") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && path.file_name().unwrap().to_string_lossy().contains("test") {
                let _ = fs::remove_dir_all(path);
            }
        }
    }
}
