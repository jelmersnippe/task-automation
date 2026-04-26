use std::{
    process::{Command, Stdio},
    rc::Rc,
};

use crate::interpreter::{builtin::BuiltinFn, coerce, scope::DataType};

pub static BUILTINS: &[(&str, BuiltinFn)] = &[
    ("print", print),
    ("len", len),
    ("spawn_terminal", spawn_terminal),
];

fn len(_: Option<Rc<DataType>>, data: Vec<Rc<DataType>>) -> Rc<DataType> {
    let [arg] = data.as_slice() else {
        panic!("len only takes 1 argument. Received: {:?}", data)
    };

    match arg.as_ref() {
        DataType::String(string) => Rc::new(DataType::Number(string.len() as f32)),
        DataType::List(list_declaration) => list_declaration.length(),
        DataType::Dictionary(dict) => dict.length(),
        _ => panic!("Can't get length for '{}'", arg),
    }
}

fn print(_: Option<Rc<DataType>>, data: Vec<Rc<DataType>>) -> Rc<DataType> {
    let [arg] = data.as_slice() else {
        panic!("print only takes 1 argument. Received: {:?}", data)
    };

    println!("{}", arg);

    Rc::new(DataType::Undefined)
}

// wt.exe wsl bash -c "cd ~/dev/task-automation && exec bash"
fn spawn_terminal(_: Option<Rc<DataType>>, data: Vec<Rc<DataType>>) -> Rc<DataType> {
    let [path, rest @ ..] = data.as_slice() else {
        panic!("spawn_terminal takes 1-2 arguments. Received: {:?}", data)
    };
    let mut command: String;

    command = format!("cd {}", coerce::expect_string(path));

    let [cmd] = rest else {
        panic!("spawn_terminal takes 1-2 arguments. Received: {:?}", data)
    };

    command += format!(" && {}", coerce::expect_string(cmd)).as_str();

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
