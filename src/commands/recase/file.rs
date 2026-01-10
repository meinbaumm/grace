use crate::arguments::into::{Into, map_case};
use crate::arguments::sanitize::maybe_sanitize;
use crate::core::case::string::recase;
use crate::core::file::file::{File, FileErr};

pub fn extract_file_name_and_extension(file: &File) -> (String, String) {
    let file_name = file.file_stem().unwrap_or_default();
    let extension = file.extension().unwrap_or_default();

    (file_name, extension)
}

pub fn recase_file(file: Option<String>, into: &Into, is_sanitize: &bool) -> Result<(), FileErr> {
    let file_path = file.ok_or(FileErr::FileDoesNotExist)?;
    let file = File::new(file_path.as_str());

    if !file.exist() {
        return Err(FileErr::FileDoesNotExist);
    }

    if !file.is_file() {
        return Err(FileErr::NotAFile);
    }

    let (file_name, file_extension) = extract_file_name_and_extension(&file);
    let file_name_to_recase = maybe_sanitize(file_name, is_sanitize);

    let recased_file_name = recase(file_name_to_recase, map_case(into));
    let to_rename = format!("{}.{}", recased_file_name, file_extension);

    file.rename(&to_rename)?;

    println!("Renamed to: {}", to_rename);
    Ok(())
}

pub fn recase_directory(
    directory: Option<String>,
    into: &Into,
    is_sanitize: &bool,
) -> Result<(), FileErr> {
    let dir_path = directory.ok_or(FileErr::DirectoryDoesNotExist)?;
    let directory = File::new(dir_path.as_str());

    if !directory.exist() {
        return Err(FileErr::DirectoryDoesNotExist);
    }

    if !directory.is_dir() {
        return Err(FileErr::NotADirectory);
    }

    let dir_name = directory.file_stem().unwrap_or_default();
    let dir_name_to_recase = maybe_sanitize(dir_name, is_sanitize);

    let recased_dir_name = recase(dir_name_to_recase, map_case(into));

    directory.rename(&recased_dir_name)?;

    println!("Renamed to: {}", recased_dir_name);
    Ok(())
}
