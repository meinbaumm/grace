use crate::core::case::sanitize;

pub fn sanitize_string(string: &str) {
    let sanitized_string = sanitize(string);
    println!("{}", sanitized_string);
}
