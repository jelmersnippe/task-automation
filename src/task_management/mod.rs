use std::{cell::RefCell, collections::HashMap, fmt};

use crate::{RuntimeContext, interpreter::function::FunctionDeclaration};

pub struct TaskRegistry {
    tasks: RefCell<HashMap<String, FunctionDeclaration>>,
}

impl TaskRegistry {
    pub fn new() -> Self {
        Self {
            tasks: Default::default(),
        }
    }

    pub fn register(&self, name: String, task: FunctionDeclaration) {
        if self.tasks.borrow().contains_key(name.as_str()) {
            println!(
                "Hey buddy! Task '{}' was already registered, but I'll override it for you. I hope you know what you're doing :)",
                name
            );
        }

        self.tasks.borrow_mut().insert(name, task);
    }

    pub fn run(&self, name: String, context: &RuntimeContext) -> Result<(), TaskRunError> {
        let tasks = self.tasks.borrow();
        let task = tasks.get(&name);

        match task {
            Some(task) => task.execute(vec![], context),
            None => {
                return Err(TaskRunError {
                    task: name,
                    reason: "Not registered",
                });
            }
        };

        Ok(())
    }
}

#[derive(Debug)]
pub struct TaskRunError {
    task: String,
    reason: &'static str,
}

impl fmt::Display for TaskRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Running task '{}' failed: {}", self.task, self.reason)
    }
}
