use std::vec::Vec;

use clap::{Parser, Subcommand};

use grace::arguments::into;
use grace::commands::recase::{string, file, files};

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
        into: into::IntoPossibleValues,
        #[arg(long)]
        sanitize: bool,
    },
    File {
        #[arg(short, long, value_parser = clap::builder::NonEmptyStringValueParser::new())]
        file: Option<String>,
        #[arg(short, long)]
        into: into::IntoPossibleValues,
        #[arg(short, long)]
        sanitize: bool,
    },
    Files {
        #[arg(short, long, value_parser = clap::builder::NonEmptyStringValueParser::new())]
        directory: Option<String>,
        #[arg(short, long)]
        into: into::IntoPossibleValues,
        #[arg(short, long, value_delimiter = ',')]
        formats: Vec<String>,
        #[arg(short, long)]
        sanitize: bool,
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
            } => string::recase_string(string.clone(), &into_arg, is_sanitize),

            Recase::File {
                file,
                into: into_arg,
                sanitize: is_sanitize,
            } => {
                let _ = file::recase_file(file.clone(), &into_arg, is_sanitize);
            }
            Recase::Files {
                directory,
                into: into_arg,
                sanitize: is_sanitize,
                formats,
            } => {
                let _ = files::recase_files(directory.clone(), &into_arg, is_sanitize, &formats);
            }
        },
    }
}
