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

    let (file_name, file_extension) = extract_file_name_and_extension(&file);
    let file_name_to_recase = arguments::maybe_sanitize(file_name, is_sanitize);

    let recased_file_name = recase(file_name_to_recase, arguments::map_case(&into));
    let to_rename = format!("{}.{}", recased_file_name, file_extension);

    let _ = file.rename_file(&to_rename);

    println!("Renamed to: {}", to_rename.green());
    Ok(())
}
