use std::{collections::HashMap, fmt, rc::Rc};

use crate::{RuntimeContext, interpreter::function::FunctionDeclaration};

pub struct TaskRegistry {
    tasks: HashMap<&'static str, Rc<FunctionDeclaration>>,
}

impl TaskRegistry {
    pub fn new() -> Self {
        Self {
            tasks: Default::default(),
        }
    }

    pub fn register(&mut self, name: &'static str, task: Rc<FunctionDeclaration>) {
        if self.tasks.contains_key(name) {
            println!(
                "Hey buddy! Task '{}' was already registered, but I'll override it for you. I hope you know what you're doing :)",
                name
            );
        }

        self.tasks.insert(name, task);
    }

    pub fn run(
        &mut self,
        name: &'static str,
        context: &RuntimeContext,
    ) -> Result<(), TaskRunError> {
        let task = self.tasks.get(name);

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
    task: &'static str,
    reason: &'static str,
}

impl fmt::Display for TaskRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Running task '{}' failed: {}", self.task, self.reason)
    }
}
