use walkdir::WalkDir;

use crate::arguments;
use crate::commands::recase::file::{extract_file_name_and_extension, recase_file};
use crate::core::file::{self, FileErr};

pub fn recase_files(
    directory: Option<String>,
    into: &arguments::Into,
    is_sanitize: &bool,
    formats_to_recase: &Vec<String>,
    is_recursive: &bool,
) -> Result<(), FileErr> {
    let formats = arguments::preprocess_formats(formats_to_recase);

    if *is_recursive {
        let _ = recase_recursively(directory.clone(), &into, is_sanitize, &formats);
    } else {
        let _ = recase(directory.clone(), &into, is_sanitize, &formats);
    }

    Ok(())
}

fn recase(
    directory: Option<String>,
    into: &arguments::Into,
    is_sanitize: &bool,
    formats_to_recase: &Vec<String>,
) -> Result<(), FileErr> {
    let provided_directory = directory.as_deref().unwrap();
    let file_path = file::File::new(provided_directory);

    let files_to_recase = {
        match file_path.read_dir() {
            Ok(files) => filter_files_by_formats(files, &formats_to_recase),
            Err(_err) => {
                unimplemented!()
            }
        }
    };

    let directory_with_slash = maybe_add_slash_to_directory(&provided_directory);

    for file in files_to_recase {
        let _ = recase_file(
            Some(format!("{}{}", directory_with_slash, file)),
            &into,
            is_sanitize,
        );
    }

    Ok(())
}

fn maybe_add_slash_to_directory(directory: &str) -> String {
    if directory.ends_with('/') {
        directory.to_string()
    } else {
        format!("{}/", directory)
    }
}

fn filter_files_by_formats(files: Vec<String>, files_formats: &Vec<String>) -> Vec<String> {
    if files_formats.is_empty() {
        return files;
    }

    files
        .into_iter()
        .filter(|file| {
            let (_, file_extension) =
                extract_file_name_and_extension(&file::File::new(file.as_str()));

            files_formats.contains(&file_extension)
        })
        .collect()
}

fn recase_recursively(
    directory: Option<String>,
    into: &arguments::Into,
    is_sanitize: &bool,
    formats_to_recase: &Vec<String>,
) -> Result<(), FileErr> {
    for entry in WalkDir::new(directory.unwrap())
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file = file::File::new(entry.path().to_str().unwrap());
        let entry_path_string = entry.path().to_str().unwrap().to_string();

        let (_, file_extension) = extract_file_name_and_extension(&file);

        if formats_to_recase.len() > 0 && formats_to_recase.contains(&file_extension) {
            let _ = recase_file(Some(entry_path_string), &into, is_sanitize);
        } else if formats_to_recase.len() == 0 {
            let _ = recase_file(Some(entry_path_string), &into, is_sanitize);
        }
    }
    Ok(())
}
