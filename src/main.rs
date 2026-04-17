use std::{
    env,
    fs::read_to_string,
    io::{self, Write, stdin},
};

use crate::{
    interpreter::Interpreter,
    lexer::lexer::{TokenKind, lexer, print_tokens},
    parser::{Parser, print_ast},
};

mod interpreter;
mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // repl();
    process_file(file_path);
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

fn process_file(path: &String) {
    let dsl = read_to_string(path).unwrap();

    let tokens = lexer(dsl);

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();
}
