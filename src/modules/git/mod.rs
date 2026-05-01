use regex::Regex;
use std::{collections::HashMap, sync::LazyLock};

use crate::{
    fs::get_absolute_path,
    interpreter::{
        builtin::{CallInfo, ExecutionError},
        coerce::{Args, OptionalValue},
        datatype::{DataType, SharedDataType},
        dictionary::DictionaryDeclaration,
        list::ListDeclaration,
    },
    modules::Module,
    RuntimeContext,
};

#[cfg(test)]
mod tests;

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

fn in_directory(
    _: Option<SharedDataType>,
    args: Vec<SharedDataType>,
    context: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let args = Args::new("in_directory", &args);
    args.exact(1)?;

    let directory = args.string(0)?;
    context.cwd = get_absolute_path(&directory)?;

    Ok((DataType::Module(create_git_module())).to_shared())
}

fn current_branch(
    _: Option<SharedDataType>,
    args: Vec<SharedDataType>,
    context: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let args = Args::new("current_branch", &args);
    args.exact(0)?;

    let branch = context
        .git_runner
        .run(&["rev-parse", "--abbrev-ref", "HEAD"], &context.cwd)?;

    Ok((DataType::String(String::from(branch.trim()))).to_shared())
}

fn rebase(
    _: Option<SharedDataType>,
    args: Vec<SharedDataType>,
    context: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let args = Args::new("rebase", &args);
    args.exact(0)?;

    context
        .git_runner
        .run(&["rebase", "origin/master"], &context.cwd)?;

    Ok((DataType::Undefined).to_shared())
}

fn local_branches(
    _: Option<SharedDataType>,
    args: Vec<SharedDataType>,
    context: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let args = Args::new("local_branches", &args);
    args.exact(0)?;

    let branches: Vec<SharedDataType> = context
        .git_runner
        .run(
            &["for-each-ref", "--format=%(refname:short)", "refs/heads/"],
            &context.cwd,
        )?
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| (DataType::String(x.trim().to_string())).to_shared())
        .collect();

    Ok((DataType::List(ListDeclaration::new(branches))).to_shared())
}

fn remote_branches(
    _: Option<SharedDataType>,
    args: Vec<SharedDataType>,
    context: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let args = Args::new("remote_branches", &args);
    args.exact(0)?;

    let branches: Vec<SharedDataType> = context
        .git_runner
        .run(&["branch", "--remote"], &context.cwd)?
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| (DataType::String(x.trim().to_string())).to_shared())
        .collect();

    Ok((DataType::List(ListDeclaration::new(branches))).to_shared())
}

fn worktrees(
    _: Option<SharedDataType>,
    args: Vec<SharedDataType>,
    context: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let args = Args::new("worktrees", &args);
    args.exact(0)?;

    let worktree_info: Vec<WorktreeInfo> = context
        .git_runner
        .run(&["worktree", "list"], &context.cwd)?
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| parse_worktree_line(x))
        .collect::<Result<Vec<_>, _>>()?;

    let result: Vec<SharedDataType> = worktree_info
        .iter()
        .map(|x| {
            DataType::Dictionary(DictionaryDeclaration::new(HashMap::from([
                (
                    String::from("directory"),
                    (DataType::String(x.directory.clone())).to_shared(),
                ),
                (
                    String::from("branch"),
                    (DataType::String(x.branch.clone())).to_shared(),
                ),
            ])))
            .to_shared()
        })
        .collect();

    Ok((DataType::List(ListDeclaration::new(result))).to_shared())
}

fn delete_branch(
    _: Option<SharedDataType>,
    args: Vec<SharedDataType>,
    context: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let args = Args::new("delete_branch", &args);
    args.exact(1)?;
    let branch = args.string(0)?;

    context
        .git_runner
        .run(&["branch", "-D", &branch], &context.cwd)?;

    Ok((DataType::Undefined).to_shared())
}

fn fetch(
    _: Option<SharedDataType>,
    args: Vec<SharedDataType>,
    context: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let args = Args::new("fetch", &args);
    args.exact(0)?;

    context.git_runner.run(&["fetch"], &context.cwd)?;

    Ok((DataType::Undefined).to_shared())
}

fn prune(
    _: Option<SharedDataType>,
    args: Vec<SharedDataType>,
    context: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let args = Args::new("prune", &args);
    args.exact(0)?;

    context.git_runner.run(&["gc"], &context.cwd)?;

    Ok((DataType::Undefined).to_shared())
}

fn push(
    _: Option<SharedDataType>,
    args: Vec<SharedDataType>,
    context: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let args = Args::new("push", &args);
    args.range(0, 1)?;

    let mut git_args = vec!["push"];

    match args.string(0).optional()? {
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
    let branch_args = Args::new("push", &vec![current_branch]);
    let branch = branch_args.string(0)?;

    git_args.push("origin");
    git_args.push(&branch);

    context.git_runner.run(&git_args, &context.cwd)?;

    Ok((DataType::Undefined).to_shared())
}

fn pull(
    _: Option<SharedDataType>,
    args: Vec<SharedDataType>,
    context: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let args = Args::new("pull", &args);
    args.exact(0)?;

    context.git_runner.run(&["pull"], &context.cwd)?;

    Ok((DataType::Undefined).to_shared())
}

pub(crate) struct WorktreeInfo {
    pub branch: String,
    pub directory: String,
}

const WORKTREE_PATTERN: &str = r"^(\S+)\s+\S+\s+\[([^\]]+)\]";
static WORKTREE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(WORKTREE_PATTERN).unwrap());

pub(crate) fn parse_worktree_line(line: &str) -> Result<WorktreeInfo, ExecutionError> {
    WORKTREE_REGEX
        .captures(line)
        .map(|caps| WorktreeInfo {
            directory: caps[1].to_string(),
            branch: caps[2].to_string(),
        })
        .ok_or_else(|| {
            ExecutionError::new(
                CallInfo::new("parse_worktree_line"),
                format!("Worktree line was not valid: {}", line).as_str(),
            )
        })
}
