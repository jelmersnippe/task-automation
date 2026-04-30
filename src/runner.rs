use std::{
    env, fmt, fs,
    io::{self, Write},
    path::{Path, PathBuf},
    rc::Rc,
};

use crate::{
    RuntimeContext,
    interpreter::{Interpreter, builtin::ExecutionError, datatype::DataType},
    lexer::Lexer,
    parser::Parser,
};

#[derive(Debug)]
pub struct RuntimeError {
    reason: String,
}

impl From<ExecutionError> for RuntimeError {
    fn from(value: ExecutionError) -> Self {
        Self {
            reason: value.to_string(),
        }
    }
}

impl From<std::io::Error> for RuntimeError {
    fn from(value: std::io::Error) -> Self {
        Self {
            reason: value.to_string(),
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Runtime failure: {}", self.reason)
    }
}

#[derive(Debug)]
struct RunArgs {
    pub recursive: bool,
    pub directory: Option<String>,
    pub task_name: String,
    pub task_args: Vec<String>,
}

pub fn repl(runtime_context: &mut RuntimeContext) -> Result<(), RuntimeError> {
    loop {
        let mut dsl = String::new();
        print!("> ");
        let _ = io::stdout().flush();
        let _ = io::stdin().read_line(&mut dsl);

        interpret(dsl, runtime_context)?;
    }
}

pub fn run(args: &[String], runtime_context: &mut RuntimeContext) -> Result<(), RuntimeError> {
    let run_args = parse_run_arguments(args);

    let dsl_directory = if let Some(directory) = run_args.directory {
        PathBuf::from(directory)
    } else {
        env::current_dir()?
    };
    let dsl_files = get_dsl_files_from_dir(&dsl_directory, run_args.recursive)?;

    for file in dsl_files {
        process_file(&file, runtime_context)?;
    }

    // TODO: Use interpreter to parse arguments?
    let task_args: Vec<Rc<DataType>> = run_args
        .task_args
        .iter()
        .map(|x| Rc::new(DataType::String(x.clone())))
        .collect();

    let task_result = runtime_context.task_registry.get(&run_args.task_name);

    match task_result {
        Err(err) => println!("Error: {}", err),
        Ok(task) => _ = task.execute(task_args, runtime_context),
    };

    Ok(())
}

fn parse_run_arguments(args: &[String]) -> RunArgs {
    let mut recursive = false;
    let mut directory: Option<String> = None;
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
        } else if arg == "--dir" {
            i += 1;
            directory = Some(args[i].clone());
        } else if task_name.is_empty() {
            task_name = args[i].clone();
        } else {
            task_args.push(args[i].clone())
        }

        i += 1
    }

    RunArgs {
        recursive,
        directory,
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

fn process_file(
    path: &std::path::Path,
    runtime_context: &mut RuntimeContext,
) -> Result<(), RuntimeError> {
    println!("Processing file {}", path.display());

    let dsl = fs::read_to_string(path)?;

    interpret(dsl, runtime_context)?;

    Ok(())
}

pub fn interpret(
    input: String,
    runtime_context: &mut RuntimeContext,
) -> Result<Interpreter, ExecutionError> {
    let tokens = Lexer::new().tokenize(input);

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    let mut interpreter = Interpreter::new(ast, runtime_context)?;
    interpreter.interpret(runtime_context)?;

    Ok(interpreter)
}
