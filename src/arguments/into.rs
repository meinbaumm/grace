use crate::case::string::Case;
use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum IntoPossibleValues {
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

pub fn unwrap_into_arg(value: &IntoPossibleValues) -> Case {
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
