use grace_cli::core::case;

#[test]
fn recase() {
    struct TestCaseStruct {
        input: String,
        expected: String,
        case: case::Case,
    }

    let test_cases = vec![
        TestCaseStruct {
            input: "Example String".to_string(),
            expected: "eXaMpLe StRiNg".to_string(),
            case: case::Case::Alternating,
        },
        TestCaseStruct {
            input: "Example String".to_string(),
            expected: "example_string".to_string(),
            case: case::Case::Snake,
        },
        TestCaseStruct {
            input: "Example String".to_string(),
            expected: "exampleString".to_string(),
            case: case::Case::Camel,
        },
        TestCaseStruct {
            input: "Example String".to_string(),
            expected: "example-string".to_string(),
            case: case::Case::Kebab,
        },
        TestCaseStruct {
            input: "Example String".to_string(),
            expected: "example.string".to_string(),
            case: case::Case::Dot,
        },
        TestCaseStruct {
            input: "Example String".to_string(),
            expected: "Example-String".to_string(),
            case: case::Case::Header,
        },
        TestCaseStruct {
            input: "Example String".to_string(),
            expected: "example string".to_string(),
            case: case::Case::Normal,
        },
        TestCaseStruct {
            input: "Example String".to_string(),
            expected: "Example String".to_string(),
            case: case::Case::Original,
        },
        TestCaseStruct {
            input: "Example String".to_string(),
            expected: "ExampleString".to_string(),
            case: case::Case::Pascal,
        },
        TestCaseStruct {
            input: "Example String".to_string(),
            expected: "example/string".to_string(),
            case: case::Case::Path,
        },
        TestCaseStruct {
            input: "example string".to_string(),
            expected: "Example string".to_string(),
            case: case::Case::Sentence,
        },
        TestCaseStruct {
            input: "example string".to_string(),
            expected: "Example String".to_string(),
            case: case::Case::Title,
        },
        TestCaseStruct {
            input: "Example String".to_string(),
            expected: "EXAMPLE_STRING".to_string(),
            case: case::Case::UpperSnake,
        },
        TestCaseStruct {
            input: "Example String".to_string(),
            expected: "example\\string".to_string(),
            case: case::Case::WindowsPath,
        },
    ];

    test_cases.into_iter().for_each(|test_case| {
        let result = case::recase(test_case.input, test_case.case);
        assert_eq!(result, test_case.expected);
    });
}

#[test]
fn sanitize() {
    struct TestCaseStruct<'a> {
        inout: &'a str,
        expected: String,
    }

    let test_cases = vec![
        TestCaseStruct {
            inout: "my'example'@str[i]ng",
            expected: "myexamplestring".to_string(),
        },
        TestCaseStruct {
            inout: "Hello my friend",
            expected: "Hello my friend".to_string(),
        },
        TestCaseStruct {
            inout: "I love my life [2022]",
            expected: "I love my life 2022".to_string(),
        },
        TestCaseStruct {
            inout: "«Publisher» (best sellet) true crime series, read it! [2023]",
            expected: "Publisher best sellet true crime series read it 2023".to_string(),
        },
    ];

    test_cases.into_iter().for_each(|test_case| {
        let result = case::sanitize(test_case.inout);
        assert_eq!(result, test_case.expected);
    });
}
