use std::{collections::HashMap, sync::Arc};

use crate::{
    interpreter::{builtin::ExecutionError, datatype::DataType, Interpreter},
    modules::{git_module, GitError, GitRunner},
    runner::interpret,
    RuntimeContext,
};

use super::parse_worktree_line;

// ---------------------------------------------------------------------------
// MockGitRunner
// ---------------------------------------------------------------------------

struct MockGitRunner {
    /// Maps "arg0 arg1 arg2 ..." to the response string.
    responses: HashMap<String, String>,
}

impl MockGitRunner {
    fn new() -> Self {
        Self {
            responses: HashMap::new(),
        }
    }

    fn with(mut self, args: &[&str], response: &str) -> Self {
        self.responses.insert(args.join(" "), response.to_string());
        self
    }
}

impl GitRunner for MockGitRunner {
    fn run(&self, args: &[&str], _cwd: &str) -> Result<String, GitError> {
        let key = args.join(" ");
        match self.responses.get(&key) {
            Some(response) => Ok(response.clone()),
            None => Err(GitError {
                command: format!("git {}", key),
                reason: format!("MockGitRunner: no response registered for '{}'", key),
            }),
        }
    }
}

// ---------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------

fn try_run_with_mock(dsl: &str, mock: MockGitRunner) -> Result<Interpreter, ExecutionError> {
    let mut context = RuntimeContext::new();
    context.git_runner = Arc::new(mock);
    context.module_registry.register(git_module());
    interpret(dsl.to_string(), &mut context)
}

fn run_with_mock(dsl: &str, mock: MockGitRunner) -> Interpreter {
    try_run_with_mock(dsl, mock).unwrap()
}

fn empty_mock() -> MockGitRunner {
    MockGitRunner::new()
}

// ---------------------------------------------------------------------------
// Unit tests: parse_worktree_line
// ---------------------------------------------------------------------------

#[test]
fn parse_worktree_line_valid() {
    let line = "/home/user/project abc1234 [main]";
    let info = parse_worktree_line(line).unwrap();
    assert_eq!(info.directory, "/home/user/project");
    assert_eq!(info.branch, "main");
}

#[test]
fn parse_worktree_line_feature_branch() {
    let line = "/repos/myrepo deadbeef [feature/my-feature]";
    let info = parse_worktree_line(line).unwrap();
    assert_eq!(info.directory, "/repos/myrepo");
    assert_eq!(info.branch, "feature/my-feature");
}

#[test]
fn parse_worktree_line_no_branch_brackets() {
    let result = parse_worktree_line("/some/path abc1234 main");
    assert!(result.is_err());
}

#[test]
fn parse_worktree_line_empty() {
    let result = parse_worktree_line("");
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// Happy-path tests (mocked git responses)
// ---------------------------------------------------------------------------

#[test]
fn current_branch_returns_branch_name() {
    let mock = empty_mock().with(&["rev-parse", "--abbrev-ref", "HEAD"], "main\n");
    let interpreter = run_with_mock("var b = git.current_branch()", mock);
    assert_eq!(
        interpreter
            .scope
            .lock()
            .unwrap()
            .get_variable(&"b".to_string())
            .unwrap(),
        (DataType::String("main".to_string())).to_shared()
    );
}

#[test]
fn local_branches_returns_list() {
    let mock = empty_mock().with(
        &["for-each-ref", "--format=%(refname:short)", "refs/heads/"],
        "main\nfeat/foo\n",
    );
    let interpreter = run_with_mock("var b = git.local_branches()", mock);
    let value = interpreter
        .scope
        .lock()
        .unwrap()
        .get_variable(&"b".to_string())
        .unwrap();
    if let DataType::List(list) = value.as_ref() {
        assert_eq!(list.length(), (DataType::Number(2.0)).to_shared());
        assert_eq!(
            list.get((DataType::Number(0.0)).to_shared()).unwrap(),
            (DataType::String("main".to_string())).to_shared()
        );
        assert_eq!(
            list.get((DataType::Number(1.0)).to_shared()).unwrap(),
            (DataType::String("feat/foo".to_string())).to_shared()
        );
    } else {
        panic!("Expected a List");
    }
}

#[test]
fn remote_branches_returns_list() {
    let mock = empty_mock().with(&["branch", "--remote"], "  origin/main\n  origin/dev\n");
    let interpreter = run_with_mock("var b = git.remote_branches()", mock);
    let value = interpreter
        .scope
        .lock()
        .unwrap()
        .get_variable(&"b".to_string())
        .unwrap();
    if let DataType::List(list) = value.as_ref() {
        assert_eq!(list.length(), (DataType::Number(2.0)).to_shared());
        assert_eq!(
            list.get((DataType::Number(0.0)).to_shared()).unwrap(),
            (DataType::String("origin/main".to_string())).to_shared()
        );
        assert_eq!(
            list.get((DataType::Number(1.0)).to_shared()).unwrap(),
            (DataType::String("origin/dev".to_string())).to_shared()
        );
    } else {
        panic!("Expected a List");
    }
}

#[test]
fn worktrees_returns_list_of_dicts() {
    let mock = empty_mock().with(
        &["worktree", "list"],
        "/home/user/repo abc1234 [main]\n/home/user/repo-feat deadbeef [feat/bar]\n",
    );
    let interpreter = run_with_mock("var w = git.worktrees()", mock);
    let value = interpreter
        .scope
        .lock()
        .unwrap()
        .get_variable(&"w".to_string())
        .unwrap();
    if let DataType::List(list) = value.as_ref() {
        assert_eq!(list.length(), (DataType::Number(2.0)).to_shared());

        let first = list.get((DataType::Number(0.0)).to_shared()).unwrap();
        if let DataType::Dictionary(dict) = first.as_ref() {
            assert_eq!(
                dict.get(&"directory".to_string()),
                (DataType::String("/home/user/repo".to_string())).to_shared()
            );
            assert_eq!(
                dict.get(&"branch".to_string()),
                (DataType::String("main".to_string())).to_shared()
            );
        } else {
            panic!("Expected first item to be a Dictionary");
        }

        let second = list.get((DataType::Number(1.0)).to_shared()).unwrap();
        if let DataType::Dictionary(dict) = second.as_ref() {
            assert_eq!(
                dict.get(&"directory".to_string()),
                (DataType::String("/home/user/repo-feat".to_string())).to_shared()
            );
            assert_eq!(
                dict.get(&"branch".to_string()),
                (DataType::String("feat/bar".to_string())).to_shared()
            );
        } else {
            panic!("Expected second item to be a Dictionary");
        }
    } else {
        panic!("Expected a List");
    }
}

#[test]
fn fetch_succeeds() {
    let mock = empty_mock().with(&["fetch"], "");
    run_with_mock("git.fetch()", mock);
}

#[test]
fn pull_succeeds() {
    let mock = empty_mock().with(&["pull"], "");
    run_with_mock("git.pull()", mock);
}

#[test]
fn rebase_succeeds() {
    let mock = empty_mock().with(&["rebase", "origin/master"], "");
    run_with_mock("git.rebase()", mock);
}

#[test]
fn prune_succeeds() {
    let mock = empty_mock().with(&["gc"], "");
    run_with_mock("git.prune()", mock);
}

#[test]
fn delete_branch_succeeds() {
    let mock = empty_mock().with(&["branch", "-D", "feat/foo"], "");
    run_with_mock(r#"git.delete_branch("feat/foo")"#, mock);
}

#[test]
fn push_succeeds() {
    let mock = empty_mock()
        .with(&["rev-parse", "--abbrev-ref", "HEAD"], "main\n")
        .with(&["push", "origin", "main"], "");
    run_with_mock("git.push()", mock);
}

#[test]
fn push_with_force_flag_uses_force_with_lease() {
    let mock = empty_mock()
        .with(&["rev-parse", "--abbrev-ref", "HEAD"], "main\n")
        .with(&["push", "--force-with-lease", "origin", "main"], "");
    run_with_mock(r#"git.push("--force")"#, mock);
}

#[test]
fn in_directory_valid_path_returns_module() {
    let mock = empty_mock();
    let interpreter = run_with_mock(r#"var g = git.in_directory(".")"#, mock);
    let value = interpreter
        .scope
        .lock()
        .unwrap()
        .get_variable(&"g".to_string())
        .unwrap();
    assert!(matches!(value.as_ref(), DataType::Module(_)));
}

// ---------------------------------------------------------------------------
// Argument validation — expect Err(ExecutionError)
// These pass: arg coercion returns ExecutionError before the git runner is called
// ---------------------------------------------------------------------------

#[test]
fn current_branch_errors_with_extra_arg() {
    let result = try_run_with_mock(r#"git.current_branch("extra")"#, empty_mock());
    assert!(result.is_err());
}

#[test]
fn local_branches_errors_with_extra_arg() {
    let result = try_run_with_mock(r#"git.local_branches("extra")"#, empty_mock());
    assert!(result.is_err());
}

#[test]
fn remote_branches_errors_with_extra_arg() {
    let result = try_run_with_mock(r#"git.remote_branches("extra")"#, empty_mock());
    assert!(result.is_err());
}

#[test]
fn worktrees_errors_with_extra_arg() {
    let result = try_run_with_mock(r#"git.worktrees("extra")"#, empty_mock());
    assert!(result.is_err());
}

#[test]
fn rebase_errors_with_extra_arg() {
    let result = try_run_with_mock(r#"git.rebase("extra")"#, empty_mock());
    assert!(result.is_err());
}

#[test]
fn fetch_errors_with_extra_arg() {
    let result = try_run_with_mock(r#"git.fetch("extra")"#, empty_mock());
    assert!(result.is_err());
}

#[test]
fn prune_errors_with_extra_arg() {
    let result = try_run_with_mock(r#"git.prune("extra")"#, empty_mock());
    assert!(result.is_err());
}

#[test]
fn pull_errors_with_extra_arg() {
    let result = try_run_with_mock(r#"git.pull("extra")"#, empty_mock());
    assert!(result.is_err());
}

#[test]
fn delete_branch_errors_with_no_args() {
    let result = try_run_with_mock("git.delete_branch()", empty_mock());
    assert!(result.is_err());
}

#[test]
fn delete_branch_errors_with_too_many_args() {
    let result = try_run_with_mock(r#"git.delete_branch("a", "b")"#, empty_mock());
    assert!(result.is_err());
}

#[test]
fn push_errors_with_too_many_args() {
    let result = try_run_with_mock(r#"git.push("a", "b")"#, empty_mock());
    assert!(result.is_err());
}

#[test]
fn push_errors_with_invalid_flag() {
    // push() explicitly returns Err before calling the runner for unrecognised flags
    let result = try_run_with_mock(r#"git.push("--bad-flag")"#, empty_mock());
    assert!(result.is_err());
}

#[test]
fn in_directory_errors_with_no_args() {
    let result = try_run_with_mock("git.in_directory()", empty_mock());
    assert!(result.is_err());
}

#[test]
fn in_directory_errors_with_too_many_args() {
    let result = try_run_with_mock(r#"git.in_directory(".", "extra")"#, empty_mock());
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// Git command failures — expect Err(ExecutionError)
// These currently FAIL: the code calls .unwrap() on the runner result and
// panics instead of propagating the error. Fix by replacing .unwrap() with
// .map_err(|e| ExecutionError::new(...))? in each git function.
// ---------------------------------------------------------------------------

#[test]
fn in_directory_errors_with_nonexistent_path() {
    let result = try_run_with_mock(
        r#"git.in_directory("/this/path/does/not/exist")"#,
        empty_mock(),
    );
    assert!(result.is_err());
}

#[test]
fn current_branch_errors_when_git_fails() {
    let result = try_run_with_mock("git.current_branch()", empty_mock());
    assert!(result.is_err());
}

#[test]
fn local_branches_errors_when_git_fails() {
    let result = try_run_with_mock("git.local_branches()", empty_mock());
    assert!(result.is_err());
}

#[test]
fn remote_branches_errors_when_git_fails() {
    let result = try_run_with_mock("git.remote_branches()", empty_mock());
    assert!(result.is_err());
}

#[test]
fn worktrees_errors_when_git_fails() {
    let result = try_run_with_mock("git.worktrees()", empty_mock());
    assert!(result.is_err());
}

#[test]
fn fetch_errors_when_git_fails() {
    let result = try_run_with_mock("git.fetch()", empty_mock());
    assert!(result.is_err());
}

#[test]
fn pull_errors_when_git_fails() {
    let result = try_run_with_mock("git.pull()", empty_mock());
    assert!(result.is_err());
}

#[test]
fn rebase_errors_when_git_fails() {
    let result = try_run_with_mock("git.rebase()", empty_mock());
    assert!(result.is_err());
}

#[test]
fn prune_errors_when_git_fails() {
    let result = try_run_with_mock("git.prune()", empty_mock());
    assert!(result.is_err());
}

#[test]
fn delete_branch_errors_when_git_fails() {
    let result = try_run_with_mock(r#"git.delete_branch("feat/foo")"#, empty_mock());
    assert!(result.is_err());
}

#[test]
fn push_errors_when_git_fails() {
    // mock has no responses, so both the rev-parse and push calls will fail
    let result = try_run_with_mock("git.push()", empty_mock());
    assert!(result.is_err());
}
