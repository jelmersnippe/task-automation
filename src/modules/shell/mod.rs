use crate::{
    RuntimeContext,
    interpreter::{
        builtin::ExecutionError,
        coerce::{Args, DictArgs, OptionalValue},
        datatype::{DataType, SharedDataType},
    },
    modules::Module,
};

#[cfg(test)]
mod tests;

pub fn create_shell_module() -> Module {
    Module::new("shell")
        .function("open", open)
        .function("run", run)
}
fn open(
    _: Option<SharedDataType>,
    args: Vec<SharedDataType>,
    context: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let args = Args::new("open", &args);
    args.exact(1)?;
    let dict = args.dictionary(0)?;

    let dict_args = DictArgs::new("open", dict);
    let cwd = dict_args
        .string("cwd")
        .optional()?
        .unwrap_or(context.cwd.clone());
    let cmd = dict_args.string("cmd").optional()?;

    spawn_open(&cwd, cmd.as_deref())
}

fn run(
    _: Option<SharedDataType>,
    args: Vec<SharedDataType>,
    context: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    Ok(DataType::Undefined.to_shared())
}

#[cfg(target_os = "macos")]
fn spawn_open(path: &str, cmd: Option<&str>) -> Result<SharedDataType, ExecutionError> {
    use std::process::{Command, Stdio};

    use crate::interpreter::builtin::CallInfo;

    let mut cmd_string = format!("cd '{}'", path);
    if let Some(cmd) = cmd {
        cmd_string += &format!(" && {}", cmd);
    }
    cmd_string += " && exec $SHELL";

    println!("Opening an iterm2 window with command '{}'", cmd_string);

    let _ = Command::new("osascript")
        .args(["-e", "tell application \"iTerm2\""])
        .args(["-e", "set w to (create window with default profile)"])
        .args(["-e", "tell current session of w"])
        .args(["-e", &format!("write text \"{}\"", cmd_string)])
        .args(["-e", "end tell"])
        .args(["-e", "end tell"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| {
            ExecutionError::new(
                CallInfo::new("run"),
                &format!("Spawning child for open failed: {}", err),
            )
        })?;

    Ok(DataType::Undefined.to_shared())
}

#[cfg(target_os = "windows")]
fn spawn_open(path: &str, cmd: Option<&str>) -> Result<(), ExecutionError> {
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

    Ok((DataType::Undefined).to_shared())
}
