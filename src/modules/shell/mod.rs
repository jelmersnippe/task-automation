use crate::{
    fs::get_absolute_path,
    interpreter::{
        builtin::ExecutionError,
        coerce::{Args, DictArgs, OptionalValue},
        datatype::{DataType, SharedDataType},
    },
    modules::Module,
    RuntimeContext,
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
    let absolute_cwd = get_absolute_path(&cwd)?;

    let cmd = dict_args.string("cmd").optional()?;

    spawn_open(&absolute_cwd, cmd.as_deref())
}

fn run(
    _: Option<SharedDataType>,
    args: Vec<SharedDataType>,
    context: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let args = Args::new("run", &args);
    args.exact(1)?;
    let dict = args.dictionary(0)?;

    let dict_args = DictArgs::new("run", dict);
    let cwd = dict_args
        .string("cwd")
        .optional()?
        .unwrap_or(context.cwd.clone());
    let absolute_cwd = get_absolute_path(&cwd)?;

    let cmd = dict_args.string("cmd")?;

    spawn_run(&absolute_cwd, &cmd)
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

    Command::new("osascript")
        .args(["-e", "tell application \"iTerm2\""])
        .args(["-e", "set w to (create window with default profile)"])
        .args(["-e", "tell current session of w"])
        .args(["-e", &format!("write text \"{}\"", cmd_string)])
        .args(["-e", "end tell"])
        .args(["-e", "end tell"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|err| {
            ExecutionError::new(
                CallInfo::new("open"),
                &format!("Spawning child for open failed: {}", err),
            )
        })?;

    Ok(DataType::Undefined.to_shared())
}

#[cfg(target_os = "windows")]
fn spawn_open(path: &str, cmd: Option<&str>) -> Result<SharedDataType, ExecutionError> {
    use std::process::{Command, Stdio};

    use crate::interpreter::builtin::CallInfo;

    let mut command = format!("cd '{}'", path);
    if let Some(cmd) = cmd {
        command += &format!(" && {}", cmd);
    }
    command += " && exec bash";

    println!("Opening: wt.exe wsl bash -lc \"{}\"", command);

    Command::new("wt.exe")
        .args(["wsl", "bash", "-lc", &command])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|err| {
            ExecutionError::new(
                CallInfo::new("open"),
                &format!("Spawning child for open failed: {}", err),
            )
        })?;

    Ok(DataType::Undefined.to_shared())
}

#[cfg(target_os = "macos")]
fn spawn_run(path: &str, cmd: &str) -> Result<SharedDataType, ExecutionError> {
    use std::process::{Command, Stdio};

    use crate::interpreter::builtin::CallInfo;

    // Append a self-closing osascript call so the window closes when the
    // command exits, without requiring any iTerm2 profile settings.
    let cmd_string = format!(
        "cd '{}' && {}; osascript -e 'tell application \"iTerm2\" to close front window'",
        path, cmd
    );

    println!("Running an iterm2 window with command '{}'", cmd_string);

    Command::new("osascript")
        .args(["-e", "tell application \"iTerm2\""])
        .args(["-e", "set w to (create window with default profile)"])
        .args(["-e", "tell current session of w"])
        .args(["-e", &format!("write text \"{}\"", cmd_string)])
        .args(["-e", "end tell"])
        .args(["-e", "end tell"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|err| {
            ExecutionError::new(
                CallInfo::new("run"),
                &format!("Spawning child for run failed: {}", err),
            )
        })?;

    Ok(DataType::Undefined.to_shared())
}

#[cfg(target_os = "windows")]
fn spawn_run(path: &str, cmd: &str) -> Result<SharedDataType, ExecutionError> {
    use std::process::{Command, Stdio};

    use crate::interpreter::builtin::CallInfo;

    let command = format!("cd '{}' && {}", path, cmd);

    println!("Running: wsl bash -lc \"{}\"", command);

    Command::new("wsl")
        .args(["bash", "-lc", &command])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|err| {
            ExecutionError::new(
                CallInfo::new("run"),
                &format!("Spawning child for run failed: {}", err),
            )
        })?;

    Ok(DataType::Undefined.to_shared())
}
