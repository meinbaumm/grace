use crate::core::case::Case;
use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Into {
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

pub fn unwrap_into_arg(value: &Into) -> Case {
    match value {
        Into::Altering => Case::Alternating,
        Into::Snake => Case::Snake,
        Into::Camel => Case::Camel,
        Into::Kebab => Case::Kebab,
        Into::Dot => Case::Dot,
        Into::Header => Case::Header,
        Into::Normal => Case::Normal,
        Into::Original => Case::Original,
        Into::Pascal => Case::Pascal,
        Into::Path => Case::Path,
        Into::Sentence => Case::Sentence,
        Into::Title => Case::Title,
        Into::UpperSnake => Case::UpperSnake,
        Into::WindowsPath => Case::WindowsPath,
    }
}
