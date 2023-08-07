use grace::recaser::recase::{Case, recase};

fn main() {
    let test_string = "Example String";

    println!("{}", recase(test_string, Case::Snake));
}
