use std::{collections::HashMap, sync::Arc};

use crate::{
    interpreter::{builtin::ExecutionError, datatype::DataType, Interpreter},
    modules::{tmux_module, TmuxError, TmuxRunner},
    runner::interpret,
    RuntimeContext,
};

// ---------------------------------------------------------------------------
// MockTmuxRunner
// ---------------------------------------------------------------------------

struct MockTmuxRunner {
    /// Maps "arg0 arg1 arg2 ..." to the response string, or Err if the value is None.
    responses: HashMap<String, Option<String>>,
}

impl MockTmuxRunner {
    fn new() -> Self {
        Self {
            responses: HashMap::new(),
        }
    }

    fn with(mut self, args: &[&str], response: &str) -> Self {
        self.responses
            .insert(args.join(" "), Some(response.to_string()));
        self
    }

    fn with_failure(mut self, args: &[&str]) -> Self {
        self.responses.insert(args.join(" "), None);
        self
    }
}

impl TmuxRunner for MockTmuxRunner {
    fn run(&self, args: &[&str]) -> Result<String, TmuxError> {
        let key = args.join(" ");
        match self.responses.get(&key) {
            Some(Some(response)) => Ok(response.clone()),
            Some(None) => Err(TmuxError {
                command: format!("tmux {}", key),
                reason: format!("MockTmuxRunner: registered failure for '{}'", key),
            }),
            None => Err(TmuxError {
                command: format!("tmux {}", key),
                reason: format!("MockTmuxRunner: no response registered for '{}'", key),
            }),
        }
    }
}

// ---------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------

fn context_with_mock(mock: MockTmuxRunner) -> RuntimeContext {
    let mut context = RuntimeContext::new();
    context.tmux_runner = Arc::new(mock);
    context.module_registry.register(tmux_module());
    context
}

fn try_run(dsl: &str, mock: MockTmuxRunner) -> Result<Interpreter, ExecutionError> {
    interpret(dsl.to_string(), &mut context_with_mock(mock))
}

fn run(dsl: &str, mock: MockTmuxRunner) -> Interpreter {
    try_run(dsl, mock).unwrap()
}

fn empty_mock() -> MockTmuxRunner {
    MockTmuxRunner::new()
}

// ---------------------------------------------------------------------------
// new_session
// ---------------------------------------------------------------------------

#[test]
fn new_session_returns_session_module() {
    let mock = empty_mock().with(&["new-session", "-d", "-s", "work"], "");
    let interpreter = run(r#"var s = tmux.new_session("work")"#, mock);
    let value = interpreter
        .scope
        .lock()
        .unwrap()
        .get_variable(&"s".to_string())
        .unwrap();
    assert!(matches!(value.as_ref(), DataType::Module(_)));
}

#[test]
fn new_session_module_name_matches_session() {
    let mock = empty_mock().with(&["new-session", "-d", "-s", "work"], "");
    let interpreter = run(r#"var s = tmux.new_session("work")"#, mock);
    let value = interpreter
        .scope
        .lock()
        .unwrap()
        .get_variable(&"s".to_string())
        .unwrap();
    if let DataType::Module(module) = value.as_ref() {
        assert_eq!(module.name, "work");
    } else {
        panic!("Expected a Module");
    }
}

#[test]
fn new_session_errors_when_tmux_fails() {
    let mock = empty_mock().with_failure(&["new-session", "-d", "-s", "work"]);
    let result = try_run(r#"tmux.new_session("work")"#, mock);
    assert!(result.is_err());
}

#[test]
fn new_session_errors_with_no_args() {
    let result = try_run("tmux.new_session()", empty_mock());
    assert!(result.is_err());
}

#[test]
fn new_session_errors_with_too_many_args() {
    let result = try_run(r#"tmux.new_session("a", "b")"#, empty_mock());
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// kill_session
// ---------------------------------------------------------------------------

#[test]
fn kill_session_succeeds() {
    let mock = empty_mock().with(&["kill-session", "-t", "work"], "");
    run(r#"tmux.kill_session("work")"#, mock);
}

#[test]
fn kill_session_errors_when_tmux_fails() {
    let mock = empty_mock().with_failure(&["kill-session", "-t", "work"]);
    let result = try_run(r#"tmux.kill_session("work")"#, mock);
    assert!(result.is_err());
}

#[test]
fn kill_session_errors_with_no_args() {
    let result = try_run("tmux.kill_session()", empty_mock());
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// has_session
// ---------------------------------------------------------------------------

#[test]
fn has_session_returns_true_when_session_exists() {
    let mock = empty_mock().with(&["has-session", "-t", "work"], "");
    let interpreter = run(r#"var exists = tmux.has_session("work")"#, mock);
    let value = interpreter
        .scope
        .lock()
        .unwrap()
        .get_variable(&"exists".to_string())
        .unwrap();
    assert_eq!(value, DataType::Boolean(true).to_shared());
}

#[test]
fn has_session_returns_false_when_session_missing() {
    let mock = empty_mock().with_failure(&["has-session", "-t", "work"]);
    let interpreter = run(r#"var exists = tmux.has_session("work")"#, mock);
    let value = interpreter
        .scope
        .lock()
        .unwrap()
        .get_variable(&"exists".to_string())
        .unwrap();
    assert_eq!(value, DataType::Boolean(false).to_shared());
}

#[test]
fn has_session_errors_with_no_args() {
    let result = try_run("tmux.has_session()", empty_mock());
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// new_window (session method)
// ---------------------------------------------------------------------------

#[test]
fn new_window_sends_correct_command() {
    let mock = empty_mock()
        .with(&["new-session", "-d", "-s", "work"], "")
        .with(&["new-window", "-t", "work", "-n", "editor", "nvim ."], "");
    run(
        r#"tmux.new_session("work").new_window("editor", "nvim .")"#,
        mock,
    );
}

#[test]
fn new_window_returns_self_for_chaining() {
    let mock = empty_mock()
        .with(&["new-session", "-d", "-s", "work"], "")
        .with(&["new-window", "-t", "work", "-n", "editor", "nvim ."], "")
        .with(&["new-window", "-t", "work", "-n", "shell", "bash"], "");
    // chaining two new_window calls — would fail if receiver is not returned
    run(
        r#"tmux.new_session("work").new_window("editor", "nvim .").new_window("shell", "bash")"#,
        mock,
    );
}

#[test]
fn new_window_errors_when_tmux_fails() {
    let mock = empty_mock()
        .with(&["new-session", "-d", "-s", "work"], "")
        .with_failure(&["new-window", "-t", "work", "-n", "editor", "nvim ."]);
    let result = try_run(
        r#"tmux.new_session("work").new_window("editor", "nvim .")"#,
        mock,
    );
    assert!(result.is_err());
}

#[test]
fn new_window_errors_with_wrong_arg_count() {
    let mock = empty_mock().with(&["new-session", "-d", "-s", "work"], "");
    let result = try_run(r#"tmux.new_session("work").new_window("editor")"#, mock);
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// kill_window
// ---------------------------------------------------------------------------

#[test]
fn kill_window_sends_correct_command() {
    let mock = empty_mock()
        .with(&["new-session", "-d", "-s", "work"], "")
        .with(&["kill-window", "-t", "work:editor"], "");
    run(r#"tmux.new_session("work").kill_window("editor")"#, mock);
}

#[test]
fn kill_window_errors_when_tmux_fails() {
    let mock = empty_mock()
        .with(&["new-session", "-d", "-s", "work"], "")
        .with_failure(&["kill-window", "-t", "work:editor"]);
    let result = try_run(r#"tmux.new_session("work").kill_window("editor")"#, mock);
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// select_window
// ---------------------------------------------------------------------------

#[test]
fn select_window_sends_correct_command() {
    let mock = empty_mock()
        .with(&["new-session", "-d", "-s", "work"], "")
        .with(&["select-window", "-t", "work:editor"], "");
    run(r#"tmux.new_session("work").select_window("editor")"#, mock);
}

// ---------------------------------------------------------------------------
// split_pane / split_pane_h
// ---------------------------------------------------------------------------

#[test]
fn split_pane_sends_correct_command() {
    let mock = empty_mock()
        .with(&["new-session", "-d", "-s", "work"], "")
        .with(&["split-window", "-t", "work:editor", "-d", "npm test"], "");
    run(
        r#"tmux.new_session("work").split_pane("editor", "npm test")"#,
        mock,
    );
}

#[test]
fn split_pane_h_sends_correct_command() {
    let mock = empty_mock()
        .with(&["new-session", "-d", "-s", "work"], "")
        .with(
            &["split-window", "-h", "-t", "work:editor", "-d", "npm test"],
            "",
        );
    run(
        r#"tmux.new_session("work").split_pane_h("editor", "npm test")"#,
        mock,
    );
}

#[test]
fn split_pane_errors_when_tmux_fails() {
    let mock = empty_mock()
        .with(&["new-session", "-d", "-s", "work"], "")
        .with_failure(&["split-window", "-t", "work:editor", "-d", "npm test"]);
    let result = try_run(
        r#"tmux.new_session("work").split_pane("editor", "npm test")"#,
        mock,
    );
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// kill_pane
// ---------------------------------------------------------------------------

#[test]
fn kill_pane_sends_correct_command() {
    let mock = empty_mock()
        .with(&["new-session", "-d", "-s", "work"], "")
        .with(&["kill-pane", "-t", "work:editor.1"], "");
    run(r#"tmux.new_session("work").kill_pane("editor", 1)"#, mock);
}

// ---------------------------------------------------------------------------
// set_layout
// ---------------------------------------------------------------------------

#[test]
fn set_layout_sends_correct_command() {
    let mock = empty_mock()
        .with(&["new-session", "-d", "-s", "work"], "")
        .with(&["select-layout", "-t", "work:editor", "tiled"], "");
    run(
        r#"tmux.new_session("work").set_layout("editor", "tiled")"#,
        mock,
    );
}

#[test]
fn set_layout_errors_when_tmux_fails() {
    let mock = empty_mock()
        .with(&["new-session", "-d", "-s", "work"], "")
        .with_failure(&["select-layout", "-t", "work:editor", "tiled"]);
    let result = try_run(
        r#"tmux.new_session("work").set_layout("editor", "tiled")"#,
        mock,
    );
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// send_keys
// ---------------------------------------------------------------------------

#[test]
fn send_keys_sends_correct_command() {
    let mock = empty_mock()
        .with(&["new-session", "-d", "-s", "work"], "")
        .with(
            &["send-keys", "-t", "work:editor.0", "git status", "Enter"],
            "",
        );
    run(
        r#"tmux.new_session("work").send_keys("editor", 0, "git status")"#,
        mock,
    );
}

// ---------------------------------------------------------------------------
// interrupt
// ---------------------------------------------------------------------------

#[test]
fn interrupt_sends_correct_command() {
    let mock = empty_mock()
        .with(&["new-session", "-d", "-s", "work"], "")
        .with(&["send-keys", "-t", "work:editor.0", "C-c"], "");
    run(r#"tmux.new_session("work").interrupt("editor", 0)"#, mock);
}

// ---------------------------------------------------------------------------
// attach_cmd
// ---------------------------------------------------------------------------

#[test]
fn attach_cmd_returns_correct_string() {
    let mock = empty_mock().with(&["new-session", "-d", "-s", "work"], "");
    let interpreter = run(r#"var cmd = tmux.new_session("work").attach_cmd()"#, mock);
    let value = interpreter
        .scope
        .lock()
        .unwrap()
        .get_variable(&"cmd".to_string())
        .unwrap();
    assert_eq!(
        value,
        DataType::String("tmux attach -t work".to_string()).to_shared()
    );
}

#[test]
fn attach_cmd_errors_with_extra_arg() {
    let mock = empty_mock().with(&["new-session", "-d", "-s", "work"], "");
    let result = try_run(r#"tmux.new_session("work").attach_cmd("extra")"#, mock);
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// Full chaining scenario
// ---------------------------------------------------------------------------

#[test]
fn chained_session_setup_sends_all_commands() {
    let mock = empty_mock()
        .with(&["new-session", "-d", "-s", "watchers"], "")
        .with(
            &[
                "new-window",
                "-t",
                "watchers",
                "-n",
                "watches",
                "npm run build:watch",
            ],
            "",
        )
        .with(
            &[
                "split-window",
                "-t",
                "watchers:watches",
                "-d",
                "npm run test:watch",
            ],
            "",
        )
        .with(&["select-layout", "-t", "watchers:watches", "tiled"], "");
    run(
        r#"
var session = tmux.new_session("watchers")
    .new_window("watches", "npm run build:watch")
    .split_pane("watches", "npm run test:watch")
    .set_layout("watches", "tiled")
        "#,
        mock,
    );
}
