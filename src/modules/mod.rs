use std::{collections::HashMap, fmt, rc::Rc};

use crate::{RuntimeContext, interpreter::datatype::DataType, modules::git::create_git_module};

mod git;

pub type ModuleFn = fn(Vec<Rc<DataType>>, &RuntimeContext) -> DataType;

#[derive(Debug, Clone)]
pub struct ModuleFunction {
    pub name: String,
    pub function: ModuleFn,
}

impl fmt::Display for ModuleFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "module fn '{}'", self.name)
    }
}

impl ModuleFunction {
    pub fn execute(&self, args: Vec<Rc<DataType>>, context: &RuntimeContext) -> Rc<DataType> {
        Rc::new((self.function)(args, context))
    }
}

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub functions: HashMap<String, ModuleFn>,
}

impl Module {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            functions: Default::default(),
        }
    }

    pub fn function(mut self, name: &str, function: ModuleFn) -> Self {
        self.functions.insert(name.to_string(), function);
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
