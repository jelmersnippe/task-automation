use std::path::{Path, PathBuf};

use std::{
    env,
    fs::{self, read_to_string},
    io::{self, Write, stdin},
};

use crate::task_management::TaskRegistry;
use crate::{interpreter::Interpreter, parser::Parser};

mod interpreter;
mod lexer;
mod parser;
mod task_management;

pub struct RuntimeContext {
    pub task_registry: TaskRegistry,
}

impl RuntimeContext {
    pub fn new() -> Self {
        Self {
            task_registry: TaskRegistry::new(),
        }
    }
}

fn main() -> std::io::Result<()> {
    let runtime_context = RuntimeContext::new();

    let arg = std::env::args()
        .nth(1)
        .expect("Expected 'repl' or 'run' with a task name");

    match arg.as_str() {
        "repl" => repl(&runtime_context),
        "run" => {
            let arg2 = std::env::args()
                .nth(2)
                .expect("Expected a task_name as the second argument");
            let arg3 = std::env::args().nth(3);

            let cwd = env::current_dir()?; // Path
            let dsl_files = get_dsl_files_from_dir(&cwd, arg3 == Some("-r".to_string()))?;

            for file in dsl_files {
                process_file(&file, &runtime_context)?;
            }

            // TODO: Propogate error
            let _ = runtime_context.task_registry.run(arg2, &runtime_context);
        }
        _ => {}
    }

    Ok(())
}

fn get_dsl_files_from_dir(dir: &Path, recursive: bool) -> std::io::Result<Vec<PathBuf>> {
    println!("Processing dir {}", dir.display());

    let mut dsl_files = vec![];

    for entry in fs::read_dir(dir)? {
        let path = entry?.path();

        if recursive && path.is_dir() {
            let nested_dsl_files = get_dsl_files_from_dir(&path, recursive)?;
            dsl_files = [dsl_files, nested_dsl_files].concat();
            continue;
        }

        if path.extension().and_then(|ext| ext.to_str()) == Some("dsl") {
            dsl_files.push(path);
        }
    }

    Ok(dsl_files)
}

fn repl(runtime_context: &RuntimeContext) {
    loop {
        let mut dsl = String::new();
        print!("> ");
        let _ = io::stdout().flush();
        let _ = stdin().read_line(&mut dsl);

        interpret(dsl, runtime_context);
    }
}

fn process_file(path: &std::path::Path, runtime_context: &RuntimeContext) -> std::io::Result<()> {
    println!("Processing file {}", path.display());

    let dsl = read_to_string(path)?;

    interpret(dsl, runtime_context);

    Ok(())
}

pub fn interpret(input: String, runtime_context: &RuntimeContext) -> Interpreter {
    let tokens = lexer::lexer(input);

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret(runtime_context);

    interpreter
}
