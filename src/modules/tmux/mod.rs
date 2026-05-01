use crate::{
    interpreter::{
        builtin::{CallInfo, ExecutionError},
        coerce::Args,
        datatype::{DataType, SharedDataType},
    },
    modules::{cmd::resolve_cmd, Module, TmuxError},
    RuntimeContext,
};

#[cfg(test)]
mod tests;

pub fn create_tmux_module() -> Module {
    Module::new("tmux")
        .function("new_session", new_session)
        .function("kill_session", kill_session)
        .function("has_session", has_session)
}

fn create_tmux_session_module(session_name: &str) -> Module {
    Module::new(session_name)
        .function("new_window", new_window)
        .function("kill_window", kill_window)
        .function("select_window", select_window)
        .function("split_pane", split_pane)
        .function("split_pane_h", split_pane_h)
        .function("kill_pane", kill_pane)
        .function("set_layout", set_layout)
        .function("send_keys", send_keys)
        .function("interrupt", interrupt)
        .function("attach", attach)
        .function("attach_cmd", attach_cmd)
}

fn session_name_from_receiver(
    receiver: &Option<SharedDataType>,
    fn_name: &str,
) -> Result<String, ExecutionError> {
    match receiver {
        Some(shared) => match shared.as_ref() {
            DataType::Module(module) => Ok(module.name.clone()),
            _ => Err(ExecutionError::new(
                CallInfo::new(fn_name),
                "expected a tmux session module as receiver",
            )),
        },
        None => Err(ExecutionError::new(
            CallInfo::new(fn_name),
            "expected a tmux session module as receiver",
        )),
    }
}

fn tmux_error_to_execution_error(e: TmuxError, fn_name: &str) -> ExecutionError {
    ExecutionError::new(CallInfo::new(fn_name), &e.to_string())
}

// Top-level session management

fn new_session(
    _: Option<SharedDataType>,
    args: Vec<SharedDataType>,
    context: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let args = Args::new("new_session", &args);
    args.exact(1)?;
    let name = args.string(0)?;

    context
        .tmux_runner
        .run(&["new-session", "-d", "-s", &name])
        .map_err(|e| tmux_error_to_execution_error(e, "new_session"))?;

    Ok(DataType::Module(create_tmux_session_module(&name)).to_shared())
}

fn kill_session(
    _: Option<SharedDataType>,
    args: Vec<SharedDataType>,
    context: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let args = Args::new("kill_session", &args);
    args.exact(1)?;
    let name = args.string(0)?;

    context
        .tmux_runner
        .run(&["kill-session", "-t", &name])
        .map_err(|e| tmux_error_to_execution_error(e, "kill_session"))?;

    Ok(DataType::Undefined.to_shared())
}

fn has_session(
    _: Option<SharedDataType>,
    args: Vec<SharedDataType>,
    context: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let args = Args::new("has_session", &args);
    args.exact(1)?;
    let name = args.string(0)?;

    let exists = context
        .tmux_runner
        .run(&["has-session", "-t", &name])
        .is_ok();

    Ok(DataType::Boolean(exists).to_shared())
}

// Session module methods — all return self for chaining

fn new_window(
    receiver: Option<SharedDataType>,
    args: Vec<SharedDataType>,
    context: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let session = session_name_from_receiver(&receiver, "new_window")?;
    let args = Args::new("new_window", &args);
    args.exact(2)?;
    let name = args.string(0)?;
    let dict = args.dictionary(1)?;
    let resolved = resolve_cmd(dict, "new_window", &context.cwd)?;

    let result = match &resolved.cmd {
        Some(cmd) => context.tmux_runner.run(&[
            "new-window",
            "-t",
            &session,
            "-n",
            &name,
            "-c",
            &resolved.cwd,
            cmd,
        ]),
        None => context.tmux_runner.run(&[
            "new-window",
            "-t",
            &session,
            "-n",
            &name,
            "-c",
            &resolved.cwd,
        ]),
    };
    result.map_err(|e| tmux_error_to_execution_error(e, "new_window"))?;

    Ok(receiver.unwrap())
}

fn kill_window(
    receiver: Option<SharedDataType>,
    args: Vec<SharedDataType>,
    context: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let session = session_name_from_receiver(&receiver, "kill_window")?;
    let args = Args::new("kill_window", &args);
    args.exact(1)?;
    let name = args.string(0)?;
    let target = format!("{}:{}", session, name);

    context
        .tmux_runner
        .run(&["kill-window", "-t", &target])
        .map_err(|e| tmux_error_to_execution_error(e, "kill_window"))?;

    Ok(receiver.unwrap())
}

fn select_window(
    receiver: Option<SharedDataType>,
    args: Vec<SharedDataType>,
    context: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let session = session_name_from_receiver(&receiver, "select_window")?;
    let args = Args::new("select_window", &args);
    args.exact(1)?;
    let name = args.string(0)?;
    let target = format!("{}:{}", session, name);

    context
        .tmux_runner
        .run(&["select-window", "-t", &target])
        .map_err(|e| tmux_error_to_execution_error(e, "select_window"))?;

    Ok(receiver.unwrap())
}

fn split_pane(
    receiver: Option<SharedDataType>,
    args: Vec<SharedDataType>,
    context: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let session = session_name_from_receiver(&receiver, "split_pane")?;
    let args = Args::new("split_pane", &args);
    args.exact(2)?;
    let window = args.string(0)?;
    let dict = args.dictionary(1)?;
    let resolved = resolve_cmd(dict, "split_pane", &context.cwd)?;
    let target = format!("{}:{}", session, window);

    let result = match &resolved.cmd {
        Some(cmd) => context.tmux_runner.run(&[
            "split-window",
            "-t",
            &target,
            "-d",
            "-c",
            &resolved.cwd,
            cmd,
        ]),
        None => {
            context
                .tmux_runner
                .run(&["split-window", "-t", &target, "-d", "-c", &resolved.cwd])
        }
    };
    result.map_err(|e| tmux_error_to_execution_error(e, "split_pane"))?;

    Ok(receiver.unwrap())
}

fn split_pane_h(
    receiver: Option<SharedDataType>,
    args: Vec<SharedDataType>,
    context: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let session = session_name_from_receiver(&receiver, "split_pane_h")?;
    let args = Args::new("split_pane_h", &args);
    args.exact(2)?;
    let window = args.string(0)?;
    let dict = args.dictionary(1)?;
    let resolved = resolve_cmd(dict, "split_pane_h", &context.cwd)?;
    let target = format!("{}:{}", session, window);

    let result = match &resolved.cmd {
        Some(cmd) => context.tmux_runner.run(&[
            "split-window",
            "-h",
            "-t",
            &target,
            "-d",
            "-c",
            &resolved.cwd,
            cmd,
        ]),
        None => context.tmux_runner.run(&[
            "split-window",
            "-h",
            "-t",
            &target,
            "-d",
            "-c",
            &resolved.cwd,
        ]),
    };
    result.map_err(|e| tmux_error_to_execution_error(e, "split_pane_h"))?;

    Ok(receiver.unwrap())
}

fn kill_pane(
    receiver: Option<SharedDataType>,
    args: Vec<SharedDataType>,
    context: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let session = session_name_from_receiver(&receiver, "kill_pane")?;
    let args = Args::new("kill_pane", &args);
    args.exact(2)?;
    let window = args.string(0)?;
    let pane = args.int(1)? as i32;
    let target = format!("{}:{}.{}", session, window, pane);

    context
        .tmux_runner
        .run(&["kill-pane", "-t", &target])
        .map_err(|e| tmux_error_to_execution_error(e, "kill_pane"))?;

    Ok(receiver.unwrap())
}

fn set_layout(
    receiver: Option<SharedDataType>,
    args: Vec<SharedDataType>,
    context: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let session = session_name_from_receiver(&receiver, "set_layout")?;
    let args = Args::new("set_layout", &args);
    args.exact(2)?;
    let window = args.string(0)?;
    let layout = args.string(1)?;
    let target = format!("{}:{}", session, window);

    context
        .tmux_runner
        .run(&["select-layout", "-t", &target, &layout])
        .map_err(|e| tmux_error_to_execution_error(e, "set_layout"))?;

    Ok(receiver.unwrap())
}

fn send_keys(
    receiver: Option<SharedDataType>,
    args: Vec<SharedDataType>,
    context: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let session = session_name_from_receiver(&receiver, "send_keys")?;
    let args = Args::new("send_keys", &args);
    args.exact(3)?;
    let window = args.string(0)?;
    let pane = args.int(1)? as i32;
    let cmd = args.string(2)?;
    let target = format!("{}:{}.{}", session, window, pane);

    context
        .tmux_runner
        .run(&["send-keys", "-t", &target, &cmd, "Enter"])
        .map_err(|e| tmux_error_to_execution_error(e, "send_keys"))?;

    Ok(receiver.unwrap())
}

fn interrupt(
    receiver: Option<SharedDataType>,
    args: Vec<SharedDataType>,
    context: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let session = session_name_from_receiver(&receiver, "interrupt")?;
    let args = Args::new("interrupt", &args);
    args.exact(2)?;
    let window = args.string(0)?;
    let pane = args.int(1)? as i32;
    let target = format!("{}:{}.{}", session, window, pane);

    context
        .tmux_runner
        .run(&["send-keys", "-t", &target, "C-c"])
        .map_err(|e| tmux_error_to_execution_error(e, "interrupt"))?;

    Ok(receiver.unwrap())
}

fn attach(
    receiver: Option<SharedDataType>,
    args: Vec<SharedDataType>,
    context: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let session = session_name_from_receiver(&receiver, "attach")?;
    let args = Args::new("attach", &args);
    args.exact(0)?;

    context
        .tmux_runner
        .run(&["attach-session", "-t", &session])
        .map_err(|e| tmux_error_to_execution_error(e, "attach"))?;

    Ok(DataType::Undefined.to_shared())
}

fn attach_cmd(
    receiver: Option<SharedDataType>,
    args: Vec<SharedDataType>,
    _: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let session = session_name_from_receiver(&receiver, "attach_cmd")?;
    let args = Args::new("attach_cmd", &args);
    args.exact(0)?;

    Ok(DataType::String(format!("tmux attach -t {}", session)).to_shared())
}
