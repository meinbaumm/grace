use grace::arguments;

#[test]
fn maybe_sanitize() {
    let string = String::from("Hel.....lo Wo@r[l]d");
    let sanitized_string = arguments::maybe_sanitize(string, &true);
    assert_eq!(sanitized_string, "Hello World");

    let string = String::from("Hel.....lo Wo@r[l]d");
    let not_sanitized_string = arguments::maybe_sanitize(string, &false);
    assert_eq!(not_sanitized_string, "Hel.....lo Wo@r[l]d");
}
