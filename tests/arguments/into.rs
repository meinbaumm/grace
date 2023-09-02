use grace_cli::arguments;
use grace_cli::core::case::Case;

#[test]
fn map_case() {
    assert_eq!(
        arguments::map_case(&arguments::Into::Altering),
        Case::Alternating
    );
    assert_eq!(arguments::map_case(&arguments::Into::Snake), Case::Snake);
    assert_eq!(arguments::map_case(&arguments::Into::Camel), Case::Camel);
    assert_eq!(arguments::map_case(&arguments::Into::Kebab), Case::Kebab);
    assert_eq!(arguments::map_case(&arguments::Into::Dot), Case::Dot);
    assert_eq!(arguments::map_case(&arguments::Into::Header), Case::Header);
    assert_eq!(arguments::map_case(&arguments::Into::Normal), Case::Normal);
    assert_eq!(
        arguments::map_case(&arguments::Into::Original),
        Case::Original
    );
    assert_eq!(arguments::map_case(&arguments::Into::Pascal), Case::Pascal);
    assert_eq!(arguments::map_case(&arguments::Into::Path), Case::Path);
    assert_eq!(
        arguments::map_case(&arguments::Into::Sentence),
        Case::Sentence
    );
    assert_eq!(arguments::map_case(&arguments::Into::Title), Case::Title);
    assert_eq!(
        arguments::map_case(&arguments::Into::UpperSnake),
        Case::UpperSnake
    );
    assert_eq!(
        arguments::map_case(&arguments::Into::WindowsPath),
        Case::WindowsPath
    );
}
