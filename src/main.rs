use std::{
    env,
    fs::read_to_string,
    io::{self, Write, stdin},
};

use crate::{interpreter::Interpreter, lexer::lexer::lexer, parser::Parser};

mod interpreter;
mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // repl();
    process_file(file_path);
}

fn _repl() {
    loop {
        let mut dsl = String::new();
        print!("> ");
        let _ = io::stdout().flush();
        let _ = stdin().read_line(&mut dsl);

        interpret(dsl);
    }
}

fn process_file(path: &String) {
    let dsl = read_to_string(path).unwrap();

    interpret(dsl);
}

fn interpret(input: String) {
    let tokens = lexer(input);

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();
}
