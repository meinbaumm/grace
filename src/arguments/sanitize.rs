use crate::core::case::string::sanitize;

pub fn maybe_sanitize(file_name: String, is_sanitize: &bool) -> String {
    if *is_sanitize {
        sanitize(file_name.as_str())
    } else {
        file_name
    }
}
