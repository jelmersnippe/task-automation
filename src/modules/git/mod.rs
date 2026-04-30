use regex::Regex;
use std::{
    collections::HashMap,
    fmt,
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
        .function("list_local_branches", list_local_branches)
        .function("list_remote_branches", list_remote_branches)
        .function("list_worktrees", list_worktrees)
        .function("delete_branch", delete_branch)
        .function("fetch", fetch)
        .function("prune", prune)
        .function("pull", pull)
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

fn list_local_branches(args: Vec<Rc<DataType>>, _: &RuntimeContext) -> DataType {
    if !args.is_empty() {
        panic!("list_local_branches does not take any arguments");
    }

    let branches: Vec<Rc<DataType>> =
        run_git_command(&["for-each-ref", "--format=%(refname:short)", "refs/heads/"])
            .unwrap()
            .split("\n")
            .filter(|x| !x.is_empty())
            .map(|x| Rc::new(DataType::String(x.trim().to_string())))
            .collect();

    DataType::List(ListDeclaration::new(branches))
}
fn list_remote_branches(args: Vec<Rc<DataType>>, _: &RuntimeContext) -> DataType {
    if !args.is_empty() {
        panic!("list_remote_branches does not take any arguments");
    }

    let branches: Vec<Rc<DataType>> = run_git_command(&["branch", "--remote"])
        .unwrap()
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| Rc::new(DataType::String(x.trim().to_string())))
        .collect();

    DataType::List(ListDeclaration::new(branches))
}
fn list_worktrees(args: Vec<Rc<DataType>>, _: &RuntimeContext) -> DataType {
    if !args.is_empty() {
        panic!("list_remote_branches does not take any arguments");
    }

    let worktrees: Vec<Rc<DataType>> = run_git_command(&["worktree", "list"])
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
fn delete_branch(args: Vec<Rc<DataType>>, _: &RuntimeContext) -> DataType {
    let [arg] = args.as_slice() else {
        panic!("delete_branch expects 1 argument. Received: {:?}", args);
    };

    let branch = expect_string(arg);

    run_git_command(&["branch", "-D", branch.as_str()]).unwrap();

    DataType::Undefined
}
fn fetch(args: Vec<Rc<DataType>>, _: &RuntimeContext) -> DataType {
    if !args.is_empty() {
        panic!("fetch does not take any arguments");
    }

    run_git_command(&["fetch"]).unwrap();

    DataType::Undefined
}
fn prune(args: Vec<Rc<DataType>>, _: &RuntimeContext) -> DataType {
    if !args.is_empty() {
        panic!("prune does not take any arguments");
    }

    run_git_command(&["gc"]).unwrap();

    DataType::Undefined
}
fn pull(args: Vec<Rc<DataType>>, _: &RuntimeContext) -> DataType {
    if !args.is_empty() {
        panic!("pull does not take any arguments");
    }

    run_git_command(&["pull"]).unwrap();

    DataType::Undefined
}

fn run_git_command(args: &[&str]) -> Result<String, GitError> {
    let output = Command::new("git")
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
