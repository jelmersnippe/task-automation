use std::{
    fs::read_to_string,
    io::{self, Write, stdin},
};

mod automation_engine;

fn main() {
    loop {
        let mut input = String::new();
        print!("> ");
        let _ = io::stdout().flush();
        let _ = stdin().read_line(&mut input);
        automation_engine::lexer::lexer(input);
        println!();
    }
    // process_file("./dsl/variables.dsl");
}

fn process_file(path: &'static str) {
    let dsl = read_to_string(path).unwrap();
    println!("Found DSL:\n{dsl}");
}
