use grace::case::string::{recase, Case};

fn main() {
    let test_string = "Example String";

    println!("{}", recase(test_string, Case::Snake));
}
