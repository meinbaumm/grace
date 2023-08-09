use crate::arguments::{into, sanitize};
use crate::core::case::string::recase;

pub fn recase_string(
    string: Option<String>,
    into: &into::IntoPossibleValues,
    is_sanitize: &bool,
) -> () {
    let string_to_recase = {
        let string = string.clone().unwrap();
        sanitize::maybe_sanitize(string, is_sanitize)
    };

    let into = into::unwrap_into_arg(&into);

    println!("{}", recase(string_to_recase, into));
}
