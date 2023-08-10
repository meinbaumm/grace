use crate::core::case::sanitize;

/// Sanitize a string if `is_sanitize` true.
pub fn maybe_sanitize(string: String, is_sanitize: &bool) -> String {
    if *is_sanitize {
        sanitize(string.as_str())
    } else {
        string
    }
}
