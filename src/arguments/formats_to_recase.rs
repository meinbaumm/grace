/// Parse formats to recase from command line arguments. Now it removes spaces from formats.
pub fn preprocess_formats(formats: &[String]) -> Vec<String> {
    formats.iter().map(|format| remove_spaces(format)).collect()
}

fn remove_spaces(string: &str) -> String {
    string.replace(' ', "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_spaces_test() {
        let string = "pdf, epub, txt,      jpeg";

        assert_eq!(remove_spaces(string), "pdf,epub,txt,jpeg");
    }
}
