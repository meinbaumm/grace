use crate::commands::recase::file::{extract_file_name_and_extension, recase_file};
use crate::core::file::file::{self, FileErr};
use crate::arguments::into;

pub fn recase_files(
    directory: Option<String>,
    into: &into::IntoPossibleValues,
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
