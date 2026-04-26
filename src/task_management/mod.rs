use std::{collections::HashMap, rc::Rc};

use crate::interpreter::function::FunctionDeclaration;

struct TaskRegistry {
    tasks: HashMap<&'static str, Rc<FunctionDeclaration>>,
}

impl TaskRegistry {
    pub fn new() -> Self {
        Self {
            tasks: Default::default(),
        }
    }

    pub fn register(name: &str, task: Rc<FunctionDeclaration>) {
        todo!()
    }
}
