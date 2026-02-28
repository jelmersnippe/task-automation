use std::fs::read_to_string;

fn main() {
    let dsl = read_to_string("./dsl/test.dsl").unwrap();
    println!("Found DSL:\n{dsl}");
}
