use std::{collections::HashMap, sync::Arc};

use crate::{
    interpreter::{builtin::ExecutionError, datatype::DataType, Interpreter},
    modules::{tmux_module, TmuxError, TmuxRunner},
    runner::interpret,
    RuntimeContext,
};

// The project root always exists and is a known absolute path in tests.
const TEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

// ---------------------------------------------------------------------------
// MockTmuxRunner
// ---------------------------------------------------------------------------

struct MockTmuxRunner {
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
    // Fix cwd to the project root so fallback-cwd tests are predictable.
    context.cwd = TEST_DIR.to_string();
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
// new_window — explicit cwd + cmd
// ---------------------------------------------------------------------------

#[test]
fn new_window_with_cwd_and_cmd() {
    let mock = empty_mock()
        .with(&["new-session", "-d", "-s", "work"], "")
        .with(
            &[
                "new-window",
                "-t",
                "work",
                "-n",
                "editor",
                "-c",
                TEST_DIR,
                "nvim .",
            ],
            "",
        );
    let dsl = format!(
        r#"tmux.new_session("work").new_window("editor", {{cwd: "{}", cmd: "nvim ."}})"#,
        TEST_DIR
    );
    run(&dsl, mock);
}

#[test]
fn new_window_with_cwd_only_falls_back_to_shell() {
    let mock = empty_mock()
        .with(&["new-session", "-d", "-s", "work"], "")
        .with(
            &["new-window", "-t", "work", "-n", "shell", "-c", TEST_DIR],
            "",
        );
    let dsl = format!(
        r#"tmux.new_session("work").new_window("shell", {{cwd: "{}"}})"#,
        TEST_DIR
    );
    run(&dsl, mock);
}

#[test]
fn new_window_without_cwd_falls_back_to_context_cwd() {
    // No cwd key — should fall back to context.cwd = TEST_DIR
    let mock = empty_mock()
        .with(&["new-session", "-d", "-s", "work"], "")
        .with(
            &[
                "new-window",
                "-t",
                "work",
                "-n",
                "editor",
                "-c",
                TEST_DIR,
                "nvim .",
            ],
            "",
        );
    run(
        r#"tmux.new_session("work").new_window("editor", {cmd: "nvim ."})"#,
        mock,
    );
}

#[test]
fn new_window_returns_self_for_chaining() {
    let mock = empty_mock()
        .with(&["new-session", "-d", "-s", "work"], "")
        .with(
            &[
                "new-window",
                "-t",
                "work",
                "-n",
                "editor",
                "-c",
                TEST_DIR,
                "nvim .",
            ],
            "",
        )
        .with(
            &["new-window", "-t", "work", "-n", "shell", "-c", TEST_DIR],
            "",
        );
    let dsl = format!(
        r#"tmux.new_session("work")
            .new_window("editor", {{cwd: "{dir}", cmd: "nvim ."}})
            .new_window("shell",  {{cwd: "{dir}"}})"#,
        dir = TEST_DIR
    );
    run(&dsl, mock);
}

#[test]
fn new_window_errors_when_tmux_fails() {
    let mock = empty_mock()
        .with(&["new-session", "-d", "-s", "work"], "")
        .with_failure(&[
            "new-window",
            "-t",
            "work",
            "-n",
            "editor",
            "-c",
            TEST_DIR,
            "nvim .",
        ]);
    let dsl = format!(
        r#"tmux.new_session("work").new_window("editor", {{cwd: "{}", cmd: "nvim ."}})"#,
        TEST_DIR
    );
    assert!(try_run(&dsl, mock).is_err());
}

#[test]
fn new_window_errors_with_wrong_arg_count() {
    let mock = empty_mock().with(&["new-session", "-d", "-s", "work"], "");
    let result = try_run(r#"tmux.new_session("work").new_window("editor")"#, mock);
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// kill_window / select_window
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
fn split_pane_with_cwd_and_cmd() {
    let mock = empty_mock()
        .with(&["new-session", "-d", "-s", "work"], "")
        .with(
            &[
                "split-window",
                "-t",
                "work:editor",
                "-d",
                "-c",
                TEST_DIR,
                "npm test",
            ],
            "",
        );
    let dsl = format!(
        r#"tmux.new_session("work").split_pane("editor", {{cwd: "{}", cmd: "npm test"}})"#,
        TEST_DIR
    );
    run(&dsl, mock);
}

#[test]
fn split_pane_without_cwd_falls_back_to_context_cwd() {
    let mock = empty_mock()
        .with(&["new-session", "-d", "-s", "work"], "")
        .with(
            &["split-window", "-t", "work:editor", "-d", "-c", TEST_DIR],
            "",
        );
    run(r#"tmux.new_session("work").split_pane("editor", {})"#, mock);
}

#[test]
fn split_pane_h_with_cwd_and_cmd() {
    let mock = empty_mock()
        .with(&["new-session", "-d", "-s", "work"], "")
        .with(
            &[
                "split-window",
                "-h",
                "-t",
                "work:editor",
                "-d",
                "-c",
                TEST_DIR,
                "npm test",
            ],
            "",
        );
    let dsl = format!(
        r#"tmux.new_session("work").split_pane_h("editor", {{cwd: "{}", cmd: "npm test"}})"#,
        TEST_DIR
    );
    run(&dsl, mock);
}

#[test]
fn split_pane_errors_when_tmux_fails() {
    let mock = empty_mock()
        .with(&["new-session", "-d", "-s", "work"], "")
        .with_failure(&[
            "split-window",
            "-t",
            "work:editor",
            "-d",
            "-c",
            TEST_DIR,
            "npm test",
        ]);
    let dsl = format!(
        r#"tmux.new_session("work").split_pane("editor", {{cwd: "{}", cmd: "npm test"}})"#,
        TEST_DIR
    );
    assert!(try_run(&dsl, mock).is_err());
}

// ---------------------------------------------------------------------------
// kill_pane / set_layout
// ---------------------------------------------------------------------------

#[test]
fn kill_pane_sends_correct_command() {
    let mock = empty_mock()
        .with(&["new-session", "-d", "-s", "work"], "")
        .with(&["kill-pane", "-t", "work:editor.1"], "");
    run(r#"tmux.new_session("work").kill_pane("editor", 1)"#, mock);
}

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
// send_keys / interrupt
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
    let dsl = format!(
        r#"
var session = tmux.new_session("watchers")
    .new_window("watches", {{cwd: "{dir}", cmd: "npm run build:watch"}})
    .split_pane("watches", {{cwd: "{dir}", cmd: "npm run test:watch"}})
    .split_pane("watches", {{cwd: "{dir}"}})
    .set_layout("watches", "tiled")
        "#,
        dir = TEST_DIR
    );
    let mock = empty_mock()
        .with(&["new-session", "-d", "-s", "watchers"], "")
        .with(
            &[
                "new-window",
                "-t",
                "watchers",
                "-n",
                "watches",
                "-c",
                TEST_DIR,
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
                "-c",
                TEST_DIR,
                "npm run test:watch",
            ],
            "",
        )
        .with(
            &[
                "split-window",
                "-t",
                "watchers:watches",
                "-d",
                "-c",
                TEST_DIR,
            ],
            "",
        )
        .with(&["select-layout", "-t", "watchers:watches", "tiled"], "");
    run(&dsl, mock);
}
