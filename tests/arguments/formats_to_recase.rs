use grace_cli::arguments;

#[test]
fn preprocess_formats() {
    let formats = vec!["pdf, epub".to_string()];

    assert_eq!(
        arguments::preprocess_formats(&formats),
        vec!["pdf,epub".to_string()]
    );
}
