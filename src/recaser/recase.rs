use recase::ReCase;

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

    #[test]
    fn recase_test<'a>() {
        struct TestCaseStruct<'a> {
            test_name: &'a str,
            input: String,
            output: String,
            case: Case,
        }

        let test_cases = vec![
            TestCaseStruct {
                test_name: "Alternating",
                input: "Example String".to_string(),
                output: "eXaMpLe StRiNg".to_string(),
                case: Case::Alternating,
            },
            TestCaseStruct {
                test_name: "Snake",
                input: "Example String".to_string(),
                output: "example_string".to_string(),
                case: Case::Snake,
            },
            TestCaseStruct {
                test_name: "Camel",
                input: "Example String".to_string(),
                output: "exampleString".to_string(),
                case: Case::Camel,
            },
            TestCaseStruct {
                test_name: "Kebab",
                input: "Example String".to_string(),
                output: "example-string".to_string(),
                case: Case::Kebab,
            },
            TestCaseStruct {
                test_name: "Dot",
                input: "Example String".to_string(),
                output: "example.string".to_string(),
                case: Case::Dot,
            },
            TestCaseStruct {
                test_name: "Header",
                input: "Example String".to_string(),
                output: "Example-String".to_string(),
                case: Case::Header,
            },
            TestCaseStruct {
                test_name: "Normal",
                input: "Example String".to_string(),
                output: "example string".to_string(),
                case: Case::Normal,
            },
            TestCaseStruct {
                test_name: "Original",
                input: "Example String".to_string(),
                output: "Example String".to_string(),
                case: Case::Original,
            },
            TestCaseStruct {
                test_name: "Pascal",
                input: "Example String".to_string(),
                output: "ExampleString".to_string(),
                case: Case::Pascal,
            },
            TestCaseStruct {
                test_name: "Path",
                input: "Example String".to_string(),
                output: "example/string".to_string(),
                case: Case::Path,
            },
            TestCaseStruct {
                test_name: "Sentence",
                input: "example string".to_string(),
                output: "Example string".to_string(),
                case: Case::Sentence,
            },
            TestCaseStruct {
                test_name: "Title",
                input: "example string".to_string(),
                output: "Example String".to_string(),
                case: Case::Title,
            },
            TestCaseStruct {
                test_name: "UpperSnake",
                input: "Example String".to_string(),
                output: "EXAMPLE_STRING".to_string(),
                case: Case::UpperSnake,
            },
            TestCaseStruct {
                test_name: "WindowsPath",
                input: "Example String".to_string(),
                output: "example\\string".to_string(),
                case: Case::WindowsPath,
            },
        ];

        for test_case in test_cases {
            let test_name = test_case.test_name;
            let output = test_case.output.clone();

            let result = recase(test_case.input, test_case.case);

            assert!(
                result == test_case.output,
                "\n\nTest case {} failed \nGot: {}\nExpected: {}",
                test_name,
                result,
                output
            );
        }
    }
}
