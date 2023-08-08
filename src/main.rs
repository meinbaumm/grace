use grace::case::string::{recase, Case, sanitize};

use clap::{Parser, Subcommand};

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
        command: Recase,
    },
}

#[derive(Subcommand)]
enum Recase {
    String {
        #[arg(short, long)]
        string: Option<String>,
        #[arg(short, long)]
        into: Option<String>,
        #[arg(long)]
        sanitize: bool,
    },
}

fn unwrap_into_arg(value: Option<String>) -> Case {
    match value {
        Some(v) => match v.to_lowercase().as_str() {
            "altering" => Case::Alternating,
            "snake" => Case::Snake,
            "camel" => Case::Camel,
            "kebab" => Case::Kebab,
            "dot" => Case::Dot,
            "header" => Case::Header,
            "normal" => Case::Normal,
            "original" => Case::Original,
            "pascal" => Case::Pascal,
            "path" => Case::Path,
            "sentence" => Case::Sentence,
            "title" => Case::Title,
            "upper_snake" => Case::UpperSnake,
            "windows_path" => Case::WindowsPath,
            _ => unimplemented!()
        },
        None => Case::Original,
    }
}


fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Recase { command } => match command {
            Recase::String { 
                string ,
                into: into_arg,
                sanitize: is_sanitize,
            } => {

                if *is_sanitize {
                    let string_to_sanitize = string.clone().unwrap();
                    let sanitized = sanitize(string_to_sanitize.as_str());
                    let into = unwrap_into_arg(into_arg.clone());

                    let result = recase(sanitized, into);

                    println!("{}", result);
                    return;
                }

                let string_to_recase = string.clone().unwrap();
                let into = unwrap_into_arg(into_arg.clone());

                let result = recase(string_to_recase, into);

                println!("{}", result);
            }
        },
    }
}