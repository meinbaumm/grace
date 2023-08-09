use grace::core::case;

#[test]
fn recase<'a>() {
    struct TestCaseStruct<'a> {
        test_name: &'a str,
        input: String,
        output: String,
        case: case::Case,
    }

    let test_cases = vec![
        TestCaseStruct {
            test_name: "Alternating",
            input: "Example String".to_string(),
            output: "eXaMpLe StRiNg".to_string(),
            case: case::Case::Alternating,
        },
        TestCaseStruct {
            test_name: "Snake",
            input: "Example String".to_string(),
            output: "example_string".to_string(),
            case: case::Case::Snake,
        },
        TestCaseStruct {
            test_name: "Camel",
            input: "Example String".to_string(),
            output: "exampleString".to_string(),
            case: case::Case::Camel,
        },
        TestCaseStruct {
            test_name: "Kebab",
            input: "Example String".to_string(),
            output: "example-string".to_string(),
            case: case::Case::Kebab,
        },
        TestCaseStruct {
            test_name: "Dot",
            input: "Example String".to_string(),
            output: "example.string".to_string(),
            case: case::Case::Dot,
        },
        TestCaseStruct {
            test_name: "Header",
            input: "Example String".to_string(),
            output: "Example-String".to_string(),
            case: case::Case::Header,
        },
        TestCaseStruct {
            test_name: "Normal",
            input: "Example String".to_string(),
            output: "example string".to_string(),
            case: case::Case::Normal,
        },
        TestCaseStruct {
            test_name: "Original",
            input: "Example String".to_string(),
            output: "Example String".to_string(),
            case: case::Case::Original,
        },
        TestCaseStruct {
            test_name: "Pascal",
            input: "Example String".to_string(),
            output: "ExampleString".to_string(),
            case: case::Case::Pascal,
        },
        TestCaseStruct {
            test_name: "Path",
            input: "Example String".to_string(),
            output: "example/string".to_string(),
            case: case::Case::Path,
        },
        TestCaseStruct {
            test_name: "Sentence",
            input: "example string".to_string(),
            output: "Example string".to_string(),
            case: case::Case::Sentence,
        },
        TestCaseStruct {
            test_name: "Title",
            input: "example string".to_string(),
            output: "Example String".to_string(),
            case: case::Case::Title,
        },
        TestCaseStruct {
            test_name: "UpperSnake",
            input: "Example String".to_string(),
            output: "EXAMPLE_STRING".to_string(),
            case: case::Case::UpperSnake,
        },
        TestCaseStruct {
            test_name: "WindowsPath",
            input: "Example String".to_string(),
            output: "example\\string".to_string(),
            case: case::Case::WindowsPath,
        },
    ];

    for test_case in test_cases {
        let test_name = test_case.test_name;
        let output = test_case.output.clone();

        let result = case::recase(test_case.input, test_case.case);

        assert!(
            result == test_case.output,
            "\n\nTest case {} failed \nGot: {}\nExpected: {}",
            test_name,
            result,
            output
        );
    }
}

#[test]
fn sanitize() {
    let test_string = "my'example'@str[i]ng";
    let result = case::sanitize(test_string);

    assert!(result == "myexamplestring");
}
