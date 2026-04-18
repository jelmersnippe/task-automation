use std::process::{Command, Stdio};
use std::{collections::HashMap, rc::Rc, sync::OnceLock};

static BUILTINS: OnceLock<HashMap<&'static str, BuiltinFn>> = OnceLock::new();

pub(crate) fn get_builtins() -> &'static HashMap<&'static str, BuiltinFn> {
    BUILTINS.get_or_init(|| {
        HashMap::from([
            ("print", print as BuiltinFn),
            ("spawn_terminal", spawn_terminal as BuiltinFn),
        ])
    })
}

type BuiltinFn = fn(Vec<Rc<super::scope::DataType>>) -> Option<Rc<super::scope::DataType>>;

fn print(data: Vec<Rc<super::scope::DataType>>) -> Option<Rc<super::scope::DataType>> {
    if data.len() != 1 {
        panic!("print only takes 1 argument. Received: {:?}", data)
    }

    let arg = data.iter().nth(0).unwrap();

    println!("{}", arg);

    None
}

// wt.exe wsl bash -c "cd ~/dev/task-automation && exec bash"
fn spawn_terminal(data: Vec<Rc<super::scope::DataType>>) -> Option<Rc<super::scope::DataType>> {
    if data.len() < 1 || data.len() > 3 {
        panic!("spawn_terminal takes 1-3 arguments. Received: {:?}", data)
    }

    let mut command: String;

    let path = data.iter().nth(0).unwrap();
    match path.as_ref() {
        super::scope::DataType::String(x) => command = format!("cd {}", x),
        _ => panic!("Path has to be a string"),
    }

    if let Some(cmd) = data.iter().nth(1) {
        let cmd_string;

        match cmd.as_ref() {
            super::scope::DataType::String(x) => cmd_string = x.clone(),
            _ => panic!("Only string commands are supported"),
        }

        if !cmd_string.is_empty() {
            command += format!(" && {}", cmd_string).as_str();
        }
    }

    // Retain the terminal in bash mode
    command += " && exec bash";

    println!("wt.exe wsl bash -lc \"{}\"", command);

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

    None
}

pub(crate) fn execute_builtin(
    builtin: &BuiltinFn,
    arguments: Vec<Rc<super::scope::DataType>>,
) -> Option<Rc<super::scope::DataType>> {
    return builtin(arguments);
}
