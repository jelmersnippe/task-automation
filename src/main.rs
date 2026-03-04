use std::{
    fs::read_to_string,
    io::{self, Write, stdin},
};

use crate::lexer::lexer::{lexer, print_tokens};
use crate::parser::parser::{parse, print_ast};

mod lexer;
mod parser;

fn main() {
    loop {
        let mut input = String::new();
        print!("> ");
        let _ = io::stdout().flush();
        let _ = stdin().read_line(&mut input);

        let tokens = lexer(input);
        print_tokens(&tokens);

        let ast = parse(&tokens);
        print_ast(&ast);

        println!();
    }
    // process_file("./dsl/variables.dsl");
}

fn process_file(path: &'static str) {
    let dsl = read_to_string(path).unwrap();
    println!("Found DSL:\n{dsl}");
}
