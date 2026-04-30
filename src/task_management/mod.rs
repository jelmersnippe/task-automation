use std::{cell::RefCell, collections::HashMap, fmt};

use crate::interpreter::datatype::Callable;

pub struct TaskRegistry {
    tasks: RefCell<HashMap<String, Callable>>,
}

impl TaskRegistry {
    pub fn new() -> Self {
        Self {
            tasks: Default::default(),
        }
    }

    pub fn register(&self, name: String, task: Callable) {
        if self.tasks.borrow().contains_key(name.as_str()) {
            println!(
                "Hey buddy! Task '{}' was already registered, but I'll override it for you. I hope you know what you're doing :)",
                name
            );
        }

        self.tasks.borrow_mut().insert(name, task);
    }

    pub fn get(&self, name: &str) -> Result<Callable, TaskRunError> {
        match self.tasks.borrow().get(name) {
            Some(task) => Ok(task.clone()),
            None => Err(TaskRunError {
                task: name.to_string(),
                reason: "Not registered",
            }),
        }
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
