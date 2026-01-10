use grace_cli::arguments::into::Into;
use grace_cli::commands::recase::string::recase_string;

#[test]
fn test_recase_string_to_snake_case() {
    let input = Some("Hello World".to_string());
    recase_string(input, &Into::Snake, &false);
    // Note: This function prints to stdout, so we're just testing it doesn't panic
}

#[test]
fn test_recase_string_to_camel_case() {
    let input = Some("hello world".to_string());
    recase_string(input, &Into::Camel, &false);
}

#[test]
fn test_recase_string_to_pascal_case() {
    let input = Some("hello world".to_string());
    recase_string(input, &Into::Pascal, &false);
}

#[test]
fn test_recase_string_to_kebab_case() {
    let input = Some("Hello World".to_string());
    recase_string(input, &Into::Kebab, &false);
}

#[test]
fn test_recase_string_with_sanitize() {
    let input = Some("Hello@#$World".to_string());
    recase_string(input, &Into::Snake, &true);
}

#[test]
fn test_recase_string_without_sanitize() {
    let input = Some("Hello@World".to_string());
    recase_string(input, &Into::Snake, &false);
}

#[test]
fn test_recase_string_all_cases() {
    let cases = vec![
        Into::Altering,
        Into::Snake,
        Into::Camel,
        Into::Kebab,
        Into::Dot,
        Into::Header,
        Into::Normal,
        Into::Original,
        Into::Pascal,
        Into::Path,
        Into::Sentence,
        Into::Title,
        Into::UpperSnake,
        Into::WindowsPath,
    ];

    for case in cases {
        let input = Some("test string".to_string());
        recase_string(input, &case, &false);
    }
}
