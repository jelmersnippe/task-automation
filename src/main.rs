use std::env;
use std::sync::Arc;

use crate::modules::{GitRunner, ModuleRegistry, ProcessGitRunner, git_module};
use crate::runner::{RuntimeError, repl, run};
use crate::task_management::TaskRegistry;

mod interpreter;
mod lexer;
mod modules;
mod parser;
mod runner;
mod task_management;

#[derive(Clone)]
pub struct RuntimeContext {
    pub task_registry: TaskRegistry,
    pub module_registry: ModuleRegistry,
    pub cwd: String,
    pub git_runner: Arc<dyn GitRunner>,
}

impl RuntimeContext {
    pub fn new() -> Self {
        Self {
            task_registry: TaskRegistry::new(),
            module_registry: ModuleRegistry::new(),
            cwd: env::current_dir()
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap(),
            git_runner: Arc::new(ProcessGitRunner),
        }
    }
}

fn main() -> Result<(), RuntimeError> {
    let mut runtime_context = RuntimeContext::new();
    runtime_context.module_registry.register(git_module());

    let arg = std::env::args()
        .nth(1)
        .expect("Expected 'repl' or 'run' with a task name");

    match arg.as_str() {
        "repl" => repl(&mut runtime_context),
        "run" => run(
            &std::env::args().collect::<Vec<String>>()[2..],
            &mut runtime_context,
        ),
        _ => Err(RuntimeError::new(&format!(
            "Invalid argument supplied: '{}'. Expect repl or run",
            arg
        ))),
    }
}
