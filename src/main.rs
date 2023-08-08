use grace::case::string::{recase, sanitize, Case};

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
        #[arg(short, long)]
        string: Option<String>,
        #[arg(short, long, value_enum)]
        into: IntoPossibleValues,
        #[arg(long)]
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

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Recase { what_to_recase } => match what_to_recase {
            Recase::String {
                string,
                into: into_arg,
                sanitize: is_sanitize,
            } => {
                if *is_sanitize {
                    let string_to_sanitize = string.clone().unwrap();
                    let sanitized = sanitize(string_to_sanitize.as_str());
                    let into = unwrap_into_arg(&into_arg);

                    let result = recase(sanitized, into);

                    println!("{}", result);
                    return;
                }

                let string_to_recase = string.clone().unwrap();
                println!("{:?}", into_arg);
                let into = unwrap_into_arg(&into_arg);

                let result = recase(string_to_recase, into);

                println!("{}", result);
            }
        },
    }
}
