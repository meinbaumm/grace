use crate::core::case::string::sanitize;

pub fn sanitize_string(string: &str) {
    let sanitized_string = sanitize(string);
    println!("{}", sanitized_string);
}
