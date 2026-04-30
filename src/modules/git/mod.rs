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
        coerce::expect_string, datatype::DataType, dictionary::DictionaryDeclaration,
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

fn in_directory(args: Vec<Rc<DataType>>, context: &mut RuntimeContext) -> DataType {
    let [arg] = args.as_slice() else {
        panic!("in_directory expects one argument");
    };

    let directory = expect_string(arg);
    let absolute_path = canonicalize(PathBuf::from(directory))
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    context.cwd = absolute_path;

    DataType::Module(create_git_module())
}

fn current_branch(args: Vec<Rc<DataType>>, context: &mut RuntimeContext) -> DataType {
    if !args.is_empty() {
        panic!("current_branch does not take any arguments");
    }

    let branch =
        run_git_command(&["rev-parse", "--abbrev-ref", "HEAD"], context.cwd.clone()).unwrap();

    DataType::String(String::from(branch.trim()))
}

fn rebase(args: Vec<Rc<DataType>>, context: &mut RuntimeContext) -> DataType {
    if !args.is_empty() {
        panic!("current_branch does not take any arguments");
    }

    run_git_command(&["rebase", "origin/master"], context.cwd.clone()).unwrap();

    DataType::Undefined
}

fn local_branches(args: Vec<Rc<DataType>>, context: &mut RuntimeContext) -> DataType {
    if !args.is_empty() {
        panic!("local_branches does not take any arguments");
    }

    let branches: Vec<Rc<DataType>> = run_git_command(
        &["for-each-ref", "--format=%(refname:short)", "refs/heads/"],
        context.cwd.clone(),
    )
    .unwrap()
    .split("\n")
    .filter(|x| !x.is_empty())
    .map(|x| Rc::new(DataType::String(x.trim().to_string())))
    .collect();

    DataType::List(ListDeclaration::new(branches))
}
fn remote_branches(args: Vec<Rc<DataType>>, context: &mut RuntimeContext) -> DataType {
    if !args.is_empty() {
        panic!("remote_branches does not take any arguments");
    }

    let branches: Vec<Rc<DataType>> = run_git_command(&["branch", "--remote"], context.cwd.clone())
        .unwrap()
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| Rc::new(DataType::String(x.trim().to_string())))
        .collect();

    DataType::List(ListDeclaration::new(branches))
}
fn worktrees(args: Vec<Rc<DataType>>, context: &mut RuntimeContext) -> DataType {
    if !args.is_empty() {
        panic!("remote_branches does not take any arguments");
    }

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

    DataType::List(ListDeclaration::new(worktrees))
}
fn delete_branch(args: Vec<Rc<DataType>>, context: &mut RuntimeContext) -> DataType {
    let [arg] = args.as_slice() else {
        panic!("delete_branch expects 1 argument. Received: {:?}", args);
    };

    let branch = expect_string(arg);

    run_git_command(&["branch", "-D", branch.as_str()], context.cwd.clone()).unwrap();

    DataType::Undefined
}
fn fetch(args: Vec<Rc<DataType>>, context: &mut RuntimeContext) -> DataType {
    if !args.is_empty() {
        panic!("fetch does not take any arguments");
    }

    run_git_command(&["fetch"], context.cwd.clone()).unwrap();

    DataType::Undefined
}
fn prune(args: Vec<Rc<DataType>>, context: &mut RuntimeContext) -> DataType {
    if !args.is_empty() {
        panic!("prune does not take any arguments");
    }

    run_git_command(&["gc"], context.cwd.clone()).unwrap();

    DataType::Undefined
}
fn push(args: Vec<Rc<DataType>>, context: &mut RuntimeContext) -> DataType {
    if args.len() > 1 {
        panic!("push takes 0-1 arguments")
    }
    let arg = args.iter().nth(0);

    let mut git_args = vec!["push"];
    match arg {
        Some(arg) => {
            if expect_string(arg) == "--force" {
                git_args.push("--force-with-lease");
            } else {
                panic!(
                    "Invalid arg supplied to git push. Expected --force, found: {}",
                    arg
                );
            }
        }
        None => {}
    };

    let branch = expect_string(&current_branch(vec![], context));

    git_args.push("origin");
    git_args.push(&branch);

    run_git_command(&git_args, context.cwd.clone()).unwrap();

    DataType::Undefined
}
fn pull(args: Vec<Rc<DataType>>, context: &mut RuntimeContext) -> DataType {
    if !args.is_empty() {
        panic!("pull does not take any arguments");
    }

    run_git_command(&["pull"], context.cwd.clone()).unwrap();

    DataType::Undefined
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
