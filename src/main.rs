use std::path::Path;

use std::{
    env,
    fs::{self, read_to_string},
    io::{self, Write, stdin},
};

use crate::{interpreter::Interpreter, parser::Parser};

mod interpreter;
mod lexer;
mod parser;
mod task_management;

fn main() -> std::io::Result<()> {
    let arg = std::env::args()
        .nth(1)
        .expect("Expected 'repl' or 'run' with a task name");

    match arg.as_str() {
        "repl" => repl(),
        "run" => {
            let arg2 = std::env::args()
                .nth(2)
                .expect("Expected a task_name as the second argument");
            let arg3 = std::env::args().nth(3);

            let cwd = env::current_dir()?; // Path
            let _ = process_dir(&cwd, arg3 == Some("-r".to_string()));

            // TODO: Actually try to run the provided task
            todo!()
        }
        _ => {}
    }

    Ok(())
}

fn process_dir(dir: &Path, recursive: bool) -> std::io::Result<()> {
    println!("Processing dir {}", dir.display());

    for entry in fs::read_dir(dir)? {
        let path = entry?.path();

        if recursive && path.is_dir() {
            process_dir(&path, recursive)?;
            continue;
        }

        if path.extension().and_then(|ext| ext.to_str()) == Some("dsl") {
            process_file(&path)?;
        }
    }

    Ok(())
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

fn process_file(path: &std::path::Path) -> std::io::Result<()> {
    println!("Processing file {}", path.display());

    let dsl = read_to_string(path)?;

    interpret(dsl);

    Ok(())
}

fn interpret(input: String) {
    let tokens = lexer::lexer(input);

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();
}
