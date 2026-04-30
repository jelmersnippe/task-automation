use std::{collections::HashMap, fmt, rc::Rc, sync::Arc};

use crate::{
    RuntimeContext,
    interpreter::{
        builtin::{BuiltinFn, Executable},
        datatype::DataType,
    },
    modules::git::create_git_module,
};

mod git;

#[derive(Clone)]
pub struct ModuleFunction {
    pub name: String,
    pub function: Executable,
}

impl fmt::Debug for ModuleFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ModuleFunction")
            .field("name", &self.name)
            .finish()
    }
}

impl fmt::Display for ModuleFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "module fn '{}'", self.name)
    }
}

impl ModuleFunction {
    pub fn new(name: String, function: Executable) -> Self {
        Self { name, function }
    }
    pub fn execute(&self, args: Vec<Rc<DataType>>, context: &mut RuntimeContext) -> Rc<DataType> {
        (self.function)(None, args, context)
    }
}

#[derive(Clone)]
pub struct Module {
    pub name: String,
    pub functions: HashMap<String, Executable>,
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
        self.functions.insert(name.to_string(), Arc::new(function));
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
