use std::{
    fs::read_to_string,
    io::{self, Write, stdin},
};

use crate::{interpreter::Interpreter, parser::Parser};

mod interpreter;
mod lexer;
mod parser;

fn main() {
    let arg = std::env::args()
        .nth(1)
        .expect("Expected 'repl' or a dsl file path");

    if arg == "repl" {
        repl();
        return;
    }

    let path = std::path::PathBuf::from(arg);

    process_file(path);
}

fn repl() {
    loop {
        let mut dsl = String::new();
        print!("> ");
        let _ = io::stdout().flush();
        let _ = stdin().read_line(&mut dsl);

        interpret(dsl);
    }
}

fn process_file(path: std::path::PathBuf) {
    let dsl = read_to_string(path).unwrap();

    interpret(dsl);
}

fn interpret(input: String) {
    let tokens = lexer::lexer::lexer(input);

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();
}
