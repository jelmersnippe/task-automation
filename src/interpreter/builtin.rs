use std::fmt;
use std::process::{Command, Stdio};
use std::rc::Rc;

use crate::interpreter::coerce;
use crate::interpreter::scope::DataType;

pub static BUILTINS: &[(&str, BuiltinFn)] = &[
    ("print", print),
    ("len", len),
    ("spawn_terminal", spawn_terminal),
];

#[derive(Debug, Clone)]
pub struct Builtin {
    pub name: &'static str,
    receiver: Option<Rc<DataType>>,
    function: BuiltinFn,
}

impl fmt::Display for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "builtin {}", self.name)
    }
}

impl Builtin {
    pub fn new(name: &'static str, function: BuiltinFn) -> Self {
        Self {
            name,
            function,
            receiver: None,
        }
    }

    pub fn bind(&self, receiver: Rc<DataType>) -> Self {
        Self {
            name: self.name,
            function: self.function,
            receiver: Some(receiver),
        }
    }

    pub fn execute(&self, parameters: Vec<Rc<DataType>>) -> Rc<DataType> {
        (self.function)(self.receiver.clone(), parameters)
    }
}

pub type BuiltinFn = fn(Option<Rc<DataType>>, Vec<Rc<DataType>>) -> Rc<DataType>;

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

    Rc::new(DataType::Undefined())
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

    Rc::new(DataType::Undefined())
}

pub(crate) fn dict_has(receiver: Option<Rc<DataType>>, data: Vec<Rc<DataType>>) -> Rc<DataType> {
    let [key] = data.as_slice() else {
        panic!("has only takes 1 argument. received: {:?}", data)
    };

    let x = receiver.expect("has can only be called on a dictionary");

    let arg = coerce::expect_string(key);
    let dict = coerce::expect_dict(x.as_ref());

    return Rc::new(DataType::Boolean(dict.has(&arg)));
}

pub(crate) fn dict_delete(receiver: Option<Rc<DataType>>, data: Vec<Rc<DataType>>) -> Rc<DataType> {
    let [key] = data.as_slice() else {
        panic!("delete only takes 1 argument. received: {:?}", data)
    };

    let x = receiver.expect("delete can only be called on a dictionary");

    let arg = coerce::expect_string(key);
    let dict = coerce::expect_dict(x.as_ref());

    dict.delete(&arg);

    return Rc::new(DataType::Undefined());
}

pub(crate) fn dict_clear(receiver: Option<Rc<DataType>>, data: Vec<Rc<DataType>>) -> Rc<DataType> {
    if !data.is_empty() {
        panic!("clear takes no arguments. received: {:?}", data)
    }

    let x = receiver.expect("clear can only be called on a dictionary");

    let dict = coerce::expect_dict(x.as_ref());

    dict.clear();

    return Rc::new(DataType::Undefined());
}
