use std::{
    process::{Command, Stdio},
    rc::Rc,
};

use crate::{
    interpreter::{
        builtin::{BuiltinFn, CallInfo, ExecutionError},
        coerce::Args,
        datatype::DataType,
    },
    RuntimeContext,
};

pub static BUILTINS: &[(&str, BuiltinFn)] = &[
    ("print", print),
    ("len", len),
    ("spawn_terminal", spawn_terminal),
    ("register_task", register_task),
    ("run", run),
];

fn len(
    _: Option<Rc<DataType>>,
    data: Vec<Rc<DataType>>,
    _: &mut RuntimeContext,
) -> Result<Rc<DataType>, ExecutionError> {
    // TODO: Make methods on data types
    let [arg] = data.as_slice() else {
        return Err(ExecutionError::new(
            CallInfo::new("len"),
            "Invalid argument count",
        ));
    };

    Ok(match arg.as_ref() {
        DataType::String(string) => Rc::new(DataType::Number(string.len() as f32)),
        DataType::List(list_declaration) => list_declaration.length(),
        DataType::Dictionary(dict) => dict.length(),
        _ => panic!("Can't get length for '{}'", arg),
    })
}

fn print(
    _: Option<Rc<DataType>>,
    data: Vec<Rc<DataType>>,
    _: &mut RuntimeContext,
) -> Result<Rc<DataType>, ExecutionError> {
    let args = Args::new("print", &data);

    args.exact(1)?;
    let arg = args.string(0)?;

    println!("{}", arg);

    Ok(Rc::new(DataType::Undefined))
}

// wt.exe wsl bash -c "cd ~/dev/task-automation && exec bash"
fn spawn_terminal(
    _: Option<Rc<DataType>>,
    data: Vec<Rc<DataType>>,
    _: &mut RuntimeContext,
) -> Result<Rc<DataType>, ExecutionError> {
    let args = Args::new("spawn_terminal", &data);

    args.range(1, 2)?;
    let path = args.string(0)?;
    let cmd = args.optional_string(1)?;

    let mut command: String = String::from(format!("cd {}", path));

    if let Some(x) = cmd {
        command += format!(" && {}", x).as_str();
    }

    // Retain the terminal in bash mode
    command += " && exec bash";

    println!("Running: wt.exe wsl bash -lc \"{}\"", command);

    let success = Command::new("wt.exe")
        .arg("wsl")
        .arg("bash")
        // Use a login shell so path is loaded
        .arg("-lc")
        .arg(command)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    match success {
        Err(error) => println!("{}", error),
        _ => {}
    }

    Ok(Rc::new(DataType::Undefined))
}

fn register_task(
    _: Option<Rc<DataType>>,
    data: Vec<Rc<DataType>>,
    context: &mut RuntimeContext,
) -> Result<Rc<DataType>, ExecutionError> {
    let args = Args::new("register_task", &data);
    args.exact(2)?;
    let task_name = args.string(0)?;
    let task_block = args.callable(1)?;

    context
        .task_registry
        .register(task_name, task_block.clone());

    Ok(Rc::new(DataType::Undefined))
}

fn run(
    _: Option<Rc<DataType>>,
    data: Vec<Rc<DataType>>,
    context: &mut RuntimeContext,
) -> Result<Rc<DataType>, ExecutionError> {
    let args = Args::new("run", &data);

    let task_name = args.string(0)?;

    // TODO: Validate string values
    let task_args: Vec<Rc<DataType>> = args.arguments[1..].iter().cloned().collect();

    let task_result = context.task_registry.get(&task_name);

    match task_result {
        Err(_) => {
            return Err(ExecutionError::new(
                CallInfo::new(&task_name),
                "Task not registered",
            ));
        }
        Ok(task) => {
            task.execute(task_args, context)?;
        }
    }

    Ok(Rc::new(DataType::Undefined))
}
