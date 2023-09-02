use crate::arguments;
use crate::core::case::recase;

pub fn recase_string(string: Option<String>, into: &arguments::Into, is_sanitize: &bool) {
    let string_to_recase = {
        let string = string.clone().unwrap();
        arguments::maybe_sanitize(string, is_sanitize)
    };

    let into = arguments::map_case(&into);

    println!("{}", recase(string_to_recase, into));
}
