use grace::case::string::{recase, sanitize, Case};
use grace::file::file::{self, FileErr};
use std::vec::Vec;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Recase {
        #[command(subcommand)]
        what_to_recase: Recase,
    },
}

#[derive(Subcommand)]
enum Recase {
    String {
        #[arg(short, long, value_parser = clap::builder::NonEmptyStringValueParser::new())]
        string: Option<String>,
        #[arg(short, long, value_enum)]
        into: IntoPossibleValues,
        #[arg(long)]
        sanitize: bool,
    },
    File {
        #[arg(short, long, value_parser = clap::builder::NonEmptyStringValueParser::new())]
        file: Option<String>,
        #[arg(short, long)]
        into: IntoPossibleValues,
        #[arg(short, long)]
        sanitize: bool,
    },
    Files {
        #[arg(short, long, value_parser = clap::builder::NonEmptyStringValueParser::new())]
        directory: Option<String>,
        #[arg(short, long)]
        into: IntoPossibleValues,
        #[arg(short, long, value_delimiter = ',')]
        formats: Vec<String>,
        #[arg(short, long)]
        sanitize: bool,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum IntoPossibleValues {
    Altering,
    Snake,
    Camel,
    Kebab,
    Dot,
    Header,
    Normal,
    Original,
    Pascal,
    Path,
    Sentence,
    Title,
    UpperSnake,
    WindowsPath,
}

fn unwrap_into_arg(value: &IntoPossibleValues) -> Case {
    match value {
        IntoPossibleValues::Altering => Case::Alternating,
        IntoPossibleValues::Snake => Case::Snake,
        IntoPossibleValues::Camel => Case::Camel,
        IntoPossibleValues::Kebab => Case::Kebab,
        IntoPossibleValues::Dot => Case::Dot,
        IntoPossibleValues::Header => Case::Header,
        IntoPossibleValues::Normal => Case::Normal,
        IntoPossibleValues::Original => Case::Original,
        IntoPossibleValues::Pascal => Case::Pascal,
        IntoPossibleValues::Path => Case::Path,
        IntoPossibleValues::Sentence => Case::Sentence,
        IntoPossibleValues::Title => Case::Title,
        IntoPossibleValues::UpperSnake => Case::UpperSnake,
        IntoPossibleValues::WindowsPath => Case::WindowsPath,
    }
}

fn recase_string(string: Option<String>, into: &IntoPossibleValues, is_sanitize: &bool) -> () {
    let string_to_recase = {
        let string = string.clone().unwrap();
        maybe_sanitize(string, is_sanitize)
    };

    let into = unwrap_into_arg(&into);

    println!("{}", recase(string_to_recase, into));
}

fn recase_file(
    file: Option<String>,
    into: &IntoPossibleValues,
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
    let file_name_to_recase = maybe_sanitize(file_name, is_sanitize);

    let recased_file_name = recase(file_name_to_recase, unwrap_into_arg(&into));
    let to_rename = format!("{}.{}", recased_file_name, file_extension);

    let _ = file.rename_file(&to_rename);

    println!("Renamed to: {}", to_rename);
    Ok(())
}

fn recase_files(
    directory: Option<String>,
    into: &IntoPossibleValues,
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

fn maybe_sanitize(file_name: String, is_sanitize: &bool) -> String {
    if *is_sanitize {
        sanitize(file_name.as_str())
    } else {
        file_name
    }
}

fn extract_file_name_and_extension(file: &file::File) -> (String, String) {
    let file_name = file.file_stem().unwrap_or_default();
    let extension = file.extension().unwrap_or_default();

    (file_name, extension)
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Recase { what_to_recase } => match what_to_recase {
            Recase::String {
                string,
                into: into_arg,
                sanitize: is_sanitize,
            } => recase_string(string.clone(), &into_arg, is_sanitize),

            Recase::File {
                file,
                into: into_arg,
                sanitize: is_sanitize,
            } => {
                let _ = recase_file(file.clone(), &into_arg, is_sanitize);
            }
            Recase::Files {
                directory,
                into: into_arg,
                sanitize: is_sanitize,
                formats,
            } => {
                let _ = recase_files(directory.clone(), &into_arg, is_sanitize, &formats);
            }
        },
    }
}
