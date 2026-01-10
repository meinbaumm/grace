use crate::arguments::into::{Into, map_case};
use crate::arguments::sanitize::maybe_sanitize;
use crate::core::case::string::recase;

pub fn recase_string(string: Option<String>, into: &Into, is_sanitize: &bool) {
    let string_to_recase = {
        let string = string.clone().unwrap();
        maybe_sanitize(string, is_sanitize)
    };

    let into = map_case(into);

    println!("{}", recase(string_to_recase, into));
}
