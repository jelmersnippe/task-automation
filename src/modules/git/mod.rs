use regex::Regex;
use std::{
    collections::HashMap,
    fmt,
    fs::canonicalize,
    path::PathBuf,
    process::{Command, Stdio},
    rc::Rc,
};

use crate::{
    RuntimeContext,
    interpreter::{
        builtin::{CallInfo, ExecutionError},
        coerce::Args,
        datatype::DataType,
        dictionary::DictionaryDeclaration,
        list::ListDeclaration,
    },
    modules::Module,
};

pub fn create_git_module() -> Module {
    Module::new("git")
        .function("in_directory", in_directory)
        .function("current_branch", current_branch)
        .function("local_branches", local_branches)
        .function("remote_branches", remote_branches)
        .function("worktrees", worktrees)
        .function("delete_branch", delete_branch)
        .function("rebase", rebase)
        .function("fetch", fetch)
        .function("prune", prune)
        .function("pull", pull)
        .function("push", push)
}

#[derive(Debug)]
pub struct GitError {
    command: String,
    reason: String,
}

impl fmt::Display for GitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Git command '{}' failed: {}", self.command, self.reason)
    }
}

fn in_directory(
    _: Option<Rc<DataType>>,
    args: Vec<Rc<DataType>>,
    context: &mut RuntimeContext,
) -> Result<Rc<DataType>, ExecutionError> {
    let args = Args::new("in_directory", &args);
    args.exact(1)?;

    let directory = args.string(0)?;
    let absolute_path = canonicalize(PathBuf::from(directory))
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    context.cwd = absolute_path;

    Ok(Rc::new(DataType::Module(create_git_module())))
}

fn current_branch(
    _: Option<Rc<DataType>>,
    args: Vec<Rc<DataType>>,
    context: &mut RuntimeContext,
) -> Result<Rc<DataType>, ExecutionError> {
    let args = Args::new("current_branch", &args);
    args.exact(0)?;

    let branch =
        run_git_command(&["rev-parse", "--abbrev-ref", "HEAD"], context.cwd.clone()).unwrap();

    Ok(Rc::new(DataType::String(String::from(branch.trim()))))
}

fn rebase(
    _: Option<Rc<DataType>>,
    args: Vec<Rc<DataType>>,
    context: &mut RuntimeContext,
) -> Result<Rc<DataType>, ExecutionError> {
    let args = Args::new("rebase", &args);
    args.exact(0)?;

    run_git_command(&["rebase", "origin/master"], context.cwd.clone()).unwrap();

    Ok(Rc::new(DataType::Undefined))
}

fn local_branches(
    _: Option<Rc<DataType>>,
    args: Vec<Rc<DataType>>,
    context: &mut RuntimeContext,
) -> Result<Rc<DataType>, ExecutionError> {
    let args = Args::new("local_branches", &args);
    args.exact(0)?;

    let branches: Vec<Rc<DataType>> = run_git_command(
        &["for-each-ref", "--format=%(refname:short)", "refs/heads/"],
        context.cwd.clone(),
    )
    .unwrap()
    .split("\n")
    .filter(|x| !x.is_empty())
    .map(|x| Rc::new(DataType::String(x.trim().to_string())))
    .collect();

    Ok(Rc::new(DataType::List(ListDeclaration::new(branches))))
}
fn remote_branches(
    _: Option<Rc<DataType>>,
    args: Vec<Rc<DataType>>,
    context: &mut RuntimeContext,
) -> Result<Rc<DataType>, ExecutionError> {
    let args = Args::new("remote_branches", &args);
    args.exact(0)?;

    let branches: Vec<Rc<DataType>> = run_git_command(&["branch", "--remote"], context.cwd.clone())
        .unwrap()
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| Rc::new(DataType::String(x.trim().to_string())))
        .collect();

    Ok(Rc::new(DataType::List(ListDeclaration::new(branches))))
}
fn worktrees(
    _: Option<Rc<DataType>>,
    args: Vec<Rc<DataType>>,
    context: &mut RuntimeContext,
) -> Result<Rc<DataType>, ExecutionError> {
    let args = Args::new("worktrees", &args);
    args.exact(0)?;

    let worktrees: Vec<Rc<DataType>> = run_git_command(&["worktree", "list"], context.cwd.clone())
        .unwrap()
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            let worktree_info = parse_worktree_line(x);

            Rc::new(DataType::Dictionary(DictionaryDeclaration::new(
                HashMap::from([
                    (
                        String::from("directory"),
                        Rc::new(DataType::String(worktree_info.directory)),
                    ),
                    (
                        String::from("branch"),
                        Rc::new(DataType::String(worktree_info.branch)),
                    ),
                ]),
            )))
        })
        .collect();

    Ok(Rc::new(DataType::List(ListDeclaration::new(worktrees))))
}
fn delete_branch(
    _: Option<Rc<DataType>>,
    args: Vec<Rc<DataType>>,
    context: &mut RuntimeContext,
) -> Result<Rc<DataType>, ExecutionError> {
    let args = Args::new("worktrees", &args);
    args.exact(1)?;
    let branch = args.string(0)?;

    run_git_command(&["branch", "-D", &branch], context.cwd.clone()).unwrap();

    Ok(Rc::new(DataType::Undefined))
}
fn fetch(
    _: Option<Rc<DataType>>,
    args: Vec<Rc<DataType>>,
    context: &mut RuntimeContext,
) -> Result<Rc<DataType>, ExecutionError> {
    let args = Args::new("fetch", &args);
    args.exact(0)?;

    run_git_command(&["fetch"], context.cwd.clone()).unwrap();

    Ok(Rc::new(DataType::Undefined))
}
fn prune(
    _: Option<Rc<DataType>>,
    args: Vec<Rc<DataType>>,
    context: &mut RuntimeContext,
) -> Result<Rc<DataType>, ExecutionError> {
    let args = Args::new("prune", &args);
    args.exact(0)?;

    run_git_command(&["gc"], context.cwd.clone()).unwrap();

    Ok(Rc::new(DataType::Undefined))
}
fn push(
    _: Option<Rc<DataType>>,
    args: Vec<Rc<DataType>>,
    context: &mut RuntimeContext,
) -> Result<Rc<DataType>, ExecutionError> {
    let args = Args::new("push", &args);
    args.range(0, 1)?;

    let mut git_args = vec!["push"];

    match args.optional_string(0)? {
        // TODO: String literal helper
        Some(arg) => {
            if arg == "--force" {
                git_args.push("--force-with-lease");
            } else {
                return Err(ExecutionError::new(
                    CallInfo::new(""),
                    format!(
                        "Invalid arg supplied to git push. Expected --force, found: {}",
                        arg
                    )
                    .as_str(),
                ));
            }
        }
        None => {}
    };

    // TODO: Hacky solution - fix with cleaner internal helper?
    let current_branch = current_branch(None, vec![], context)?;
    let args = Args::new("push", &vec![current_branch]);
    let branch = args.string(0)?;

    git_args.push("origin");
    git_args.push(&branch);

    run_git_command(&git_args, context.cwd.clone()).unwrap();

    Ok(Rc::new(DataType::Undefined))
}
fn pull(
    _: Option<Rc<DataType>>,
    args: Vec<Rc<DataType>>,
    context: &mut RuntimeContext,
) -> Result<Rc<DataType>, ExecutionError> {
    let args = Args::new("pull", &args);
    args.exact(0)?;

    run_git_command(&["pull"], context.cwd.clone()).unwrap();

    Ok(Rc::new(DataType::Undefined))
}

fn run_git_command(args: &[&str], cwd: String) -> Result<String, GitError> {
    let output = Command::new("git")
        .current_dir(cwd)
        .args(args)
        .stdout(Stdio::piped())
        .output()
        .map_err(|error| GitError {
            reason: error.to_string(),
            command: format!("git {}", args.join(" ")),
        })?;

    if !output.status.success() {
        return Err(GitError {
            reason: String::from_utf8_lossy(&output.stderr).to_string(),
            command: format!("git {}", args.join(" ")),
        });
    }

    let stdout = String::from_utf8(output.stdout).map_err(|e| GitError {
        reason: e.to_string(),
        command: format!("git {}", args.join(" ")),
    })?;

    Ok(stdout)
}

struct WorktreeInfo {
    pub branch: String,
    pub directory: String,
}

fn parse_worktree_line(line: &str) -> WorktreeInfo {
    let re = Regex::new(r"^(\S+)\s+\S+\s+\[([^\]]+)\]").unwrap();

    re.captures(line)
        .map(|caps| WorktreeInfo {
            directory: caps[1].to_string(),
            branch: caps[2].to_string(),
        })
        .expect("Worktree parse failed.")
}
