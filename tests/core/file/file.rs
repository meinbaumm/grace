use grace::core::file::{File, FileErr};
use serial_test::{parallel, serial};

#[test]
#[parallel]
fn extension() {
    let file_name = "awesome_book.epub";

    let file: File<'_> = File::new(file_name);
    assert_eq!(file.extension().unwrap(), "epub");
}

#[test]
#[parallel]
fn file_name() {
    let file_name = "awesome_book.epub";

    let file: File<'_> = File::new(file_name);
    assert_eq!(file.file_name().unwrap(), file_name);
}

#[test]
#[parallel]
fn file_stem() {
    let file_name = "awesome_book.epub";

    let file: File<'_> = File::new(file_name);
    assert_eq!(file.file_stem().unwrap(), "awesome_book");
}

#[test]
#[parallel]
fn is_dir() {
    struct TestCaseStruct {
        input: String,
        expected: bool,
    }

    let test_cases = vec![
        TestCaseStruct {
            input: "tests/core/file".to_string(),
            expected: true,
        },
        TestCaseStruct {
            input: "tests/core/file/file.rs".to_string(),
            expected: false,
        },
    ];

    test_cases.into_iter().for_each(|test_case| {
        let file: File<'_> = File::new(&test_case.input);
        assert_eq!(file.is_dir(), test_case.expected);
    });
}

#[test]
#[parallel]
fn is_file() {
    struct TestCaseStruct {
        input: String,
        expected: bool,
    }

    let test_cases = vec![
        TestCaseStruct {
            input: "tests/core/file".to_string(),
            expected: false,
        },
        TestCaseStruct {
            input: "tests/core/file/file.rs".to_string(),
            expected: true,
        },
    ];

    test_cases.into_iter().for_each(|test_case| {
        let file: File<'_> = File::new(&test_case.input);
        assert_eq!(file.is_file(), test_case.expected);
    });
}

#[test]
#[parallel]
fn exist() {
    struct TestCaseStruct {
        input: String,
        expected: bool,
    }

    let test_cases = vec![
        TestCaseStruct {
            input: "tests/tests.rs".to_string(),
            expected: true,
        },
        TestCaseStruct {
            input: "tests/core/file/file.rs".to_string(),
            expected: true,
        },
        TestCaseStruct {
            input: "tests/core/file/does_not_exist.md".to_string(),
            expected: false,
        },
    ];

    test_cases.into_iter().for_each(|test_case| {
        let file: File<'_> = File::new(&test_case.input);
        assert_eq!(file.exist(), test_case.expected);
    });
}

#[test]
#[serial]
fn read_dir() {
    let directory = "tests/core/file";
    let expected_files_in_directory = vec!["mod.rs".to_string(), "file.rs".to_string()];

    let file: File<'_> = File::new(&directory);
    assert_eq!(file.read_dir().unwrap(), expected_files_in_directory);

    let not_a_directory = "tests/core/file/file.rs".to_string();
    let file: File<'_> = File::new(&not_a_directory);
    assert_eq!(file.read_dir().unwrap_err(), FileErr::NotADirectory);
}

#[test]
#[serial]
fn create_file() {
    let file_name = "tests/core/file/created_file.txt";

    let file: File<'_> = File::new(file_name);
    assert_eq!(file.exist(), false);

    file.create_file().unwrap();
    assert_eq!(file.exist(), true);

    std::fs::remove_file(file_name).unwrap();

    let file_name = "tests/core/file/file.rs";
    let file: File<'_> = File::new(file_name);
    assert_eq!(file.create_file().unwrap_err(), FileErr::FileAlreadyExist);
}

#[test]
#[serial]
fn rename_file() {
    let file_name: &str = "tests/core/file/created_file.txt";
    let new_file_path = "tests/core/file/renamed_file.txt";

    // test file does not exist at first
    let file: File<'_> = File::new(file_name);
    assert_eq!(file.exist(), false);

    // create file and check if it exist
    file.create_file().unwrap();
    assert_eq!(file.exist(), true);

    // rename file and check that file with old name does not exist
    file.rename("renamed_file.txt").unwrap();
    assert_eq!(file.exist(), false);

    // check that file with new name exist
    let file: File<'_> = File::new(new_file_path);
    assert_eq!(file.exist(), true);

    // remove file
    std::fs::remove_file(new_file_path).unwrap();
}
