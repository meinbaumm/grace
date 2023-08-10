use std::vec::Vec;

use clap::{Parser, Subcommand};

use grace::arguments;
use grace::commands::{recase, sanitize};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Recase string, file, or directory of files.
    Recase {
        #[command(subcommand)]
        what_to_recase: Recase,
    },
    /// Sanitize string of unnecessary characters.
    Sanitize {
        #[command(subcommand)]
        what_to_sanitize: Sanitize,
    },
}

#[derive(Subcommand)]
enum Recase {
    /// Recase string.
    String {
        /// String to recase.
        #[arg(value_parser = clap::builder::NonEmptyStringValueParser::new())]
        string: Option<String>,
        /// Case to recase string into.
        #[arg(short, long, value_enum)]
        into: arguments::Into,
        /// Sanitize string before recasing.
        #[arg(long)]
        sanitize: bool,
    },
    /// Recase file.
    File {
        /// Path to recasing file.
        #[arg(short, long, value_parser = clap::builder::NonEmptyStringValueParser::new())]
        file: Option<String>,
        /// Case to recase file into.
        #[arg(short, long)]
        into: arguments::Into,
        /// Sanitize file name before recasing.
        #[arg(short, long)]
        sanitize: bool,
    },
    /// Recase files in directory.
    Files {
        /// Path to directory containing files.
        #[arg(short, long, value_parser = clap::builder::NonEmptyStringValueParser::new())]
        directory: Option<String>,
        /// Case to recase files into.
        #[arg(short, long)]
        into: arguments::Into,
        /// File extensions to recase. If multiple extensions, provide it like so: --formats="pdf, epub".
        #[arg(short, long, value_delimiter = ',')]
        formats: Vec<String>,
        /// Sanitize file names before recasing.
        #[arg(short, long)]
        sanitize: bool,
        /// Rename files recursively.
        #[arg(short, long)]
        recursive: bool,
    },
}

#[derive(Subcommand)]
enum Sanitize {
    /// Sanitize string.
    String {
        /// String to sanitize.
        #[arg(value_parser = clap::builder::NonEmptyStringValueParser::new())]
        string: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Recase { what_to_recase } => match what_to_recase {
            Recase::String {
                string,
                into: into_arg,
                sanitize: is_sanitize,
            } => recase::recase_string(string.clone(), &into_arg, is_sanitize),

            Recase::File {
                file,
                into: into_arg,
                sanitize: is_sanitize,
            } => {
                let _ = recase::recase_file(file.clone(), &into_arg, is_sanitize);
            }
            Recase::Files {
                directory,
                into: into_arg,
                sanitize: is_sanitize,
                formats,
                recursive,
            } => {
                let formats = arguments::preprocess_formats(formats);

                if *recursive {
                    let _ = recase::recase_files_recursively(
                        directory.clone(),
                        &into_arg,
                        is_sanitize,
                        &formats,
                    );
                } else {
                    let _ =
                        recase::recase_files(directory.clone(), &into_arg, is_sanitize, &formats);
                }
            }
        },
        Commands::Sanitize { what_to_sanitize } => match what_to_sanitize {
            Sanitize::String { string } => sanitize::sanitize_string(string),
        },
    }
}
