use std::{
    process::{Command, Stdio},
    rc::Rc,
};

use crate::{
    RuntimeContext,
    interpreter::{
        builtin::{BuiltinFn, Executable},
        coerce::{self, expect_string, expect_user_function},
        datatype::DataType,
    },
};

pub static BUILTINS: &[(&str, BuiltinFn)] = &[
    ("print", print),
    ("len", len),
    ("spawn_terminal", spawn_terminal),
    ("register_task", register_task),
    ("run", run),
];

fn len(_: Option<Rc<DataType>>, data: Vec<Rc<DataType>>, _: &mut RuntimeContext) -> Rc<DataType> {
    let [arg] = data.as_slice() else {
        panic!(
            "len only takes 1 argument. Received: {:?}",
            data.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    };

    match arg.as_ref() {
        DataType::String(string) => Rc::new(DataType::Number(string.len() as f32)),
        DataType::List(list_declaration) => list_declaration.length(),
        DataType::Dictionary(dict) => dict.length(),
        _ => panic!("Can't get length for '{}'", arg),
    }
}

fn print(_: Option<Rc<DataType>>, data: Vec<Rc<DataType>>, _: &mut RuntimeContext) -> Rc<DataType> {
    let [arg] = data.as_slice() else {
        panic!(
            "print only takes 1 argument. Received: {:?}",
            data.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    };

    println!("{}", arg);

    Rc::new(DataType::Undefined)
}

// wt.exe wsl bash -c "cd ~/dev/task-automation && exec bash"
fn spawn_terminal(
    _: Option<Rc<DataType>>,
    data: Vec<Rc<DataType>>,
    _: &mut RuntimeContext,
) -> Rc<DataType> {
    let path = data.iter().nth(0).expect(
        format!(
            "spawn_terminal takes 1-2 arguments. Received: {:?}",
            data.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
        .as_str(),
    );

    let mut command: String;

    command = format!("cd {}", coerce::expect_string(path));

    if let [_, cmd] = data.as_slice() {
        command += format!(" && {}", coerce::expect_string(cmd)).as_str();
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

    Rc::new(DataType::Undefined)
}

fn register_task(
    _: Option<Rc<DataType>>,
    data: Vec<Rc<DataType>>,
    context: &mut RuntimeContext,
) -> Rc<DataType> {
    let [arg1, arg2] = data.as_slice() else {
        panic!(
            "register_task expects 2 arguments. Received: {:?}",
            data.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    };

    let task_name = expect_string(arg1);
    let task_block = expect_user_function(arg2);

    context
        .task_registry
        .register(task_name, task_block.clone());

    Rc::new(DataType::Undefined)
}

fn run(
    _: Option<Rc<DataType>>,
    data: Vec<Rc<DataType>>,
    context: &mut RuntimeContext,
) -> Rc<DataType> {
    let arg = data.iter().nth(0).expect(
        format!(
            "run expects 1 argument. Received: {:?}",
            data.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
        .as_str(),
    );

    // TODO: Validate string values
    let task_args: Vec<Rc<DataType>> = data[1..].iter().cloned().collect();

    let task = expect_string(arg);

    // TODO: Propogate error
    let task_result = context.task_registry.get(task);

    match task_result {
        Err(err) => println!("Error: {}", err),
        Ok(task) => {
            task.execute(task_args, context);
        }
    }

    Rc::new(DataType::Undefined)
}
