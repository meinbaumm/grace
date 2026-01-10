use grace_cli::arguments::into::{Into, map_case};
use grace_cli::core::case::string::Case;

#[test]
fn map_case_test() {
    assert_eq!(map_case(&Into::Altering), Case::Alternating);
    assert_eq!(map_case(&Into::Snake), Case::Snake);
    assert_eq!(map_case(&Into::Camel), Case::Camel);
    assert_eq!(map_case(&Into::Kebab), Case::Kebab);
    assert_eq!(map_case(&Into::Dot), Case::Dot);
    assert_eq!(map_case(&Into::Header), Case::Header);
    assert_eq!(map_case(&Into::Normal), Case::Normal);
    assert_eq!(map_case(&Into::Original), Case::Original);
    assert_eq!(map_case(&Into::Pascal), Case::Pascal);
    assert_eq!(map_case(&Into::Path), Case::Path);
    assert_eq!(map_case(&Into::Sentence), Case::Sentence);
    assert_eq!(map_case(&Into::Title), Case::Title);
    assert_eq!(map_case(&Into::UpperSnake), Case::UpperSnake);
    assert_eq!(map_case(&Into::WindowsPath), Case::WindowsPath);
}
