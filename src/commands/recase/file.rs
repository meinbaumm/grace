use colored::*;

use crate::arguments;
use crate::core::case::recase;
use crate::core::file::{self, FileErr};

pub fn extract_file_name_and_extension(file: &file::File) -> (String, String) {
    let file_name = file.file_stem().unwrap_or_default();
    let extension = file.extension().unwrap_or_default();

    (file_name, extension)
}

pub fn recase_file(
    file: Option<String>,
    into: &arguments::Into,
    is_sanitize: &bool,
) -> Result<(), FileErr> {
    let binding = file.unwrap();
    let file = file::File::new(binding.as_str());

    if !file.exist() {
        let err = Err(FileErr::FileDoesNotExist);
        println!("{:?}", err);
        return err;
    }

    if !file.is_file() {
        let err = Err(FileErr::NotAFile);
        println!("{:?}", err);
        return err;
    }

    let (file_name, file_extension) = extract_file_name_and_extension(&file);
    let file_name_to_recase = arguments::maybe_sanitize(file_name, is_sanitize);

    let recased_file_name = recase(file_name_to_recase, arguments::map_case(&into));
    let to_rename = format!("{}.{}", recased_file_name, file_extension);

    let _ = file.rename(&to_rename);

    println!("Renamed to: {}", to_rename.green());
    Ok(())
}

pub fn recase_directory(
    directory: Option<String>,
    into: &arguments::Into,
    is_sanitize: &bool,
) -> Result<(), FileErr> {
    let binding = directory.unwrap();
    let directory = file::File::new(binding.as_str());

    if !directory.exist() {
        let err = Err(FileErr::DirectoryDoesNotExist);
        println!("{:?}", err);
        return err;
    }

    if !directory.is_dir() {
        return Err(FileErr::NotADirectory);
    }

    let dir_name = directory.file_stem().unwrap_or_default();
    let dir_name_to_recase = arguments::maybe_sanitize(dir_name, is_sanitize);

    let recased_dir_name = recase(dir_name_to_recase, arguments::map_case(&into));

    let _ = directory.rename(&recased_dir_name);

    println!("Renamed to: {}", recased_dir_name.green());
    Ok(())
}
