use grace_cli::arguments::formats_to_recase::preprocess_formats;

#[test]
fn preprocess_formats_test() {
    let formats = vec!["pdf, epub".to_string()];

    assert_eq!(preprocess_formats(&formats), vec!["pdf,epub".to_string()]);
}
