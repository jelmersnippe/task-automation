use std::{collections::HashMap, fmt};

use crate::{
    interpreter::{builtin::BuiltinFn, datatype::Callable},
    modules::git::create_git_module,
};

mod git;

#[derive(Clone)]
pub struct Module {
    pub name: String,
    pub functions: HashMap<String, Callable>,
}

impl fmt::Debug for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Module").field("name", &self.name).finish()
    }
}

impl Module {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            functions: Default::default(),
        }
    }

    pub fn function(mut self, name: &str, function: BuiltinFn) -> Self {
        self.functions.insert(
            name.to_string(),
            Callable::new(Some(name.to_string()), function),
        );
        self
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Module '{}'", self.name)
    }
}

pub struct ModuleRegistry {
    pub modules: Vec<Module>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            modules: Default::default(),
        }
    }

    pub fn register(&mut self, module: Module) {
        self.modules.push(module);
    }
}

pub fn git_module() -> Module {
    create_git_module()
}
