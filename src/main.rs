use std::fs::read_to_string;

mod automation_engine;

fn main() {
    process_file("./dsl/variables.dsl");
}

fn process_file(path: &'static str) {
    let dsl = read_to_string(path).unwrap();
    println!("Found DSL:\n{dsl}");

    let tokens = automation_engine::lexer::lexer(dsl);

    for automation_engine::lexer::Token {
        token_type,
        token_value,
    } in tokens
    {
        println!("{token_type}: {token_value}")
    }
}
