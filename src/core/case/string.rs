use recase::ReCase;
use regex::Regex;

/// Case variants into which the string can be recased.
#[derive(PartialEq, Debug)]
pub enum Case {
    Alternating,
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

fn new_case<T>(string_to_recase: T) -> ReCase
where
    T: Into<String>,
{
    recase::ReCase::new(string_to_recase)
}

/// Recase a string into the specified case.
pub fn recase<T>(string_to_recase: T, into: Case) -> String
where
    T: Into<String>,
{
    let recase = new_case(string_to_recase);

    match into {
        Case::Alternating => recase.alternating_case(),
        Case::Snake => recase.snake_case(),
        Case::Camel => recase.camel_case(),
        Case::Kebab => recase.kebab_case(),
        Case::Dot => recase.dot_case(),
        Case::Header => recase.header_case(),
        Case::Normal => recase.normal_case(),
        Case::Original => recase.original_case(),
        Case::Pascal => recase.pascal_case(),
        Case::Path => recase.path_case(),
        Case::Sentence => recase.sentence_case(),
        Case::Title => recase.title_case(),
        Case::UpperSnake => recase.upper_snake_case(),
        Case::WindowsPath => recase.windows_path_case(),
    }
}

/// Sanitize a string by removing all characters that are not alphanumeric.
/// The only characters that are allowed are: `a-z`, `A-Z`, `0-9`, `_`, and `-`.
/// Used regex is `r"[\(\)’@#<>+«»~&%^—,!\.\[\]{}\?:;·']"`
pub fn sanitize(str_to_sanitize: &str) -> String {
    let pattern = r"[\(\)’@#<>+«»~&%^—,!\.\[\]{}\?:;·']";
    let regex = Regex::new(pattern).unwrap();
    let replaced_text = regex.replace_all(str_to_sanitize, "");

    replaced_text.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_case_test() {
        let test_string = "Example String";

        let new_from_str = new_case(test_string);
        let new_from_string = new_case(test_string.to_string());

        assert!(new_from_str.original_case() == "Example String");
        assert!(new_from_string.original_case() == "Example String");
    }
}
