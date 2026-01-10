use grace_cli::arguments::sanitize::maybe_sanitize;

#[test]
fn maybe_sanitize_test() {
    let string = String::from("Hel.....lo Wo@r[l]d");
    let sanitized_string = maybe_sanitize(string, &true);
    assert_eq!(sanitized_string, "Hello World");

    let string = String::from("Hel.....lo Wo@r[l]d");
    let not_sanitized_string = maybe_sanitize(string, &false);
    assert_eq!(not_sanitized_string, "Hel.....lo Wo@r[l]d");
}
