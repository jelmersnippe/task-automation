use crate::{
    interpreter::{
        builtin::{CallInfo, ExecutionError},
        coerce::Args,
        datatype::{DataType, SharedDataType},
    },
    modules::{cmd::resolve_cmd, Module},
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

    let resolved = resolve_cmd(dict, "open", &context.cwd)?;

    spawn_open(&resolved.to_open_cmd())
}

fn run(
    _: Option<SharedDataType>,
    args: Vec<SharedDataType>,
    context: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let args = Args::new("run", &args);
    args.exact(1)?;
    let dict = args.dictionary(0)?;

    let resolved = resolve_cmd(dict, "run", &context.cwd)?;

    let cmd_string = resolved.to_run_cmd().ok_or_else(|| {
        ExecutionError::new(CallInfo::new("run"), "shell.run requires a 'cmd' key")
    })?;

    spawn_run(&cmd_string)
}

#[cfg(target_os = "macos")]
fn spawn_open(cmd_string: &str) -> Result<SharedDataType, ExecutionError> {
    use std::process::{Command, Stdio};

    println!("Opening an iTerm2 window with command '{}'", cmd_string);

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
fn spawn_open(cmd_string: &str) -> Result<SharedDataType, ExecutionError> {
    use std::process::{Command, Stdio};

    println!("Opening: wt.exe wsl bash -lc \"{}\"", cmd_string);

    Command::new("wt.exe")
        .args(["wsl", "bash", "-lc", cmd_string])
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

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn spawn_open(_cmd_string: &str) -> Result<SharedDataType, ExecutionError> {
    Err(ExecutionError::new(
        CallInfo::new("open"),
        "shell module is not supported on this platform",
    ))
}

#[cfg(target_os = "macos")]
fn spawn_run(cmd_string: &str) -> Result<SharedDataType, ExecutionError> {
    use std::process::{Command, Stdio};

    println!("Running an iTerm2 window with command '{}'", cmd_string);

    // Close the window by its ID (captured at creation time) rather than
    // "front window", so focus changes during the run don't close the wrong window.
    let write_text = format!(
        "write text \"{} ; osascript -e 'tell application \\\"iTerm2\\\" to close (first window whose id is \" & wid & \")'\"",
        cmd_string
    );

    Command::new("osascript")
        .args(["-e", "tell application \"iTerm2\""])
        .args(["-e", "set w to (create window with default profile)"])
        .args(["-e", "set wid to id of w"])
        .args(["-e", "tell current session of w"])
        .args(["-e", &write_text])
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
fn spawn_run(cmd_string: &str) -> Result<SharedDataType, ExecutionError> {
    use std::process::{Command, Stdio};

    println!("Running: wsl bash -lc \"{}\"", cmd_string);

    Command::new("wsl")
        .args(["bash", "-lc", cmd_string])
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

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn spawn_run(_cmd_string: &str) -> Result<SharedDataType, ExecutionError> {
    Err(ExecutionError::new(
        CallInfo::new("run"),
        "shell module is not supported on this platform",
    ))
}
