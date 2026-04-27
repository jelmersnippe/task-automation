use std::path::{Path, PathBuf};

use std::rc::Rc;
use std::{
    env,
    fs::{self, read_to_string},
    io::{self, Write, stdin},
};

use crate::interpreter::scope::DataType;
use crate::lexer::Lexer;
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
            let cli_args: Vec<String> = std::env::args().collect();
            let run_args = parse_run_arguments(&cli_args[2..]);
            println!("{:?}", run_args);

            let cwd = env::current_dir()?;
            let dsl_files = get_dsl_files_from_dir(&cwd, run_args.recursive)?;

            for file in dsl_files {
                process_file(&file, &runtime_context)?;
            }

            // TODO: Use interpreter to parse arguments?
            let task_args: Vec<Rc<DataType>> = run_args
                .task_args
                .iter()
                .map(|x| Rc::new(DataType::String(x.clone())))
                .collect();

            // TODO: Propogate error
            let run_result =
                runtime_context
                    .task_registry
                    .run(run_args.task_name, task_args, &runtime_context);

            match run_result {
                Err(err) => println!("Error: {}", err),
                _ => {}
            }
        }
        _ => panic!("Invalid argument supplied: '{}'. Expect repl or run", arg),
    }

    Ok(())
}

#[derive(Debug)]
struct RunArgs {
    pub recursive: bool,
    pub task_name: String,
    pub task_args: Vec<String>,
}

fn parse_run_arguments(args: &[String]) -> RunArgs {
    let mut recursive = false;
    let mut task_name: String = String::new();
    let mut task_args: Vec<String> = vec![];

    let options = ["-r", "--recursive"];

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        println!("{}", arg);

        if options.contains(&arg.as_str()) {
            recursive = true;
        } else if arg == "--task" {
            i += 1;
            task_name = args[i].clone();
        } else if task_name.is_empty() {
            task_name = args[i].clone();
        } else {
            task_args.push(args[i].clone())
        }

        i += 1
    }

    RunArgs {
        recursive,
        task_name,
        task_args,
    }
}

fn get_dsl_files_from_dir(dir: &Path, recursive: bool) -> std::io::Result<Vec<PathBuf>> {
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
    let tokens = Lexer::new().tokenize(input);

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret(runtime_context);

    interpreter
}
