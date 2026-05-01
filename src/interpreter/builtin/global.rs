use std::{
    process::{Command, Stdio},
    rc::Rc,
    thread::{self, JoinHandle},
};

use crate::{
    RuntimeContext,
    interpreter::{
        builtin::{BuiltinFn, CallInfo, ExecutionError},
        coerce::{Args, ArgumentError, DataKind, expect_callable},
        datatype::DataType,
    },
};

pub static BUILTINS: &[(&str, BuiltinFn)] = &[
    ("print", print),
    ("spawn_terminal", spawn_terminal),
    ("register_task", register_task),
    ("parallel", parallel),
];

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

fn parallel(
    _: Option<Rc<DataType>>,
    data: Vec<Rc<DataType>>,
    context: &mut RuntimeContext,
) -> Result<Rc<DataType>, ExecutionError> {
    let args = Args::new("parallel", &data);
    args.exact(1)?;
    let list = args.list(0)?;

    let callables = list
        .values
        .borrow()
        .iter()
        .enumerate()
        .map(|(i, x)| {
            expect_callable(x).map_err(|e| ArgumentError::InvalidType {
                fn_name: String::from("parallel"),
                index: i,
                expected_type: DataKind::Callable,
                found_type: e,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let handles: Vec<JoinHandle<Result<(), ExecutionError>>> = callables
        .iter()
        .map(|x| {
            let cloned_context = context.clone();
            thread::spawn(x.execute(vec![], cloned_context))
        })
        .collect();

    for handle in handles {
        match handle.join() {
            Ok(Err(_)) => todo!(),
            Ok(Ok(_)) => todo!(),
            Err(_) => todo!(),
        }
    }

    Ok(Rc::new(DataType::Undefined))
}
