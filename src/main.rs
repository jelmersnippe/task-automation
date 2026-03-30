use std::{
    fs::read_to_string,
    io::{self, Write, stdin},
};

use crate::{
    lexer::lexer::{TokenKind, lexer, print_tokens},
    parser::{Parser, print_ast},
};

mod lexer;
mod parser;

fn main() {
    // repl();
    process_file("./dsl/test.dsl");
}

fn repl() {
    loop {
        let mut input = String::new();
        print!("> ");
        let _ = io::stdout().flush();
        let _ = stdin().read_line(&mut input);

        let tokens = lexer(input);
        print_tokens(&tokens);
        println!();

        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        print_ast(&ast);
        println!();
    }
}

fn process_file(path: &'static str) {
    let dsl = read_to_string(path).unwrap();
    println!("Found DSL:\n{dsl}");

    let tokens = lexer(dsl);
    let _ = tokens
        .iter()
        .filter(|x| x.kind == TokenKind::Illegal)
        .for_each(|x| println!("{:?}", x));

    println!();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    print_ast(&ast);
}
