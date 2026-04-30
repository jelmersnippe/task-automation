use std::{cell::RefCell, fmt, rc::Rc};

use crate::{
    RuntimeContext,
    interpreter::{
        Parameters,
        builtin::{self, Builtin},
        dictionary::DictionaryDeclaration,
        function::FunctionDeclaration,
        list::ListDeclaration,
        scope::Scope,
    },
    modules::{Module, ModuleFunction},
};

#[derive(Debug, Clone)]
pub enum DataType {
    Number(f32),
    String(String),
    Boolean(bool),
    Function(Callable),
    List(ListDeclaration),
    Dictionary(DictionaryDeclaration),
    Module(Module),
    Undefined,
}

#[derive(Debug, Clone)]
pub enum Callable {
    BuiltIn(Builtin),
    Module(ModuleFunction),
    User(FunctionDeclaration),
}

impl fmt::Display for Callable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Callable::BuiltIn(builtin) => write!(f, "{}", builtin),
            Callable::User(function_declaration) => write!(f, "{}", function_declaration),
            Callable::Module(module) => write!(f, "{}", module),
        }
    }
}

impl Callable {
    pub fn execute(
        &self,
        parameters: &Parameters,
        scope: Rc<RefCell<Scope>>,
        context: &mut RuntimeContext,
    ) -> Rc<DataType> {
        match self {
            Callable::BuiltIn(builtin) => {
                builtin.execute(parameters.resolve(scope.clone(), context), context)
            }
            Callable::User(function_declaration) => {
                function_declaration.execute(parameters.resolve(scope.clone(), context), context)
            }
            Callable::Module(module_fn) => {
                module_fn.execute(parameters.resolve(scope.clone(), context), context)
            }
        }
    }
}

impl PartialEq for DataType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::Function(l0), Self::Function(r0)) => match (l0, r0) {
                (Callable::User(f1), Callable::User(f2)) => f1 == f2,
                _ => false,
            },
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Dictionary(l0), Self::Dictionary(r0)) => l0 == r0,
            (Self::Undefined, Self::Undefined) => true,
            _ => false,
        }
    }
}

impl DataType {
    pub(crate) fn get_method(self: &Rc<DataType>, name: &str) -> Rc<DataType> {
        Rc::new(DataType::Function(match self.as_ref() {
            DataType::Dictionary(_) => Callable::BuiltIn(match name {
                "has" => Builtin::new("has", builtin::dictionary::has).bind(self.clone()),
                "delete" => Builtin::new("delete", builtin::dictionary::delete).bind(self.clone()),
                "clear" => Builtin::new("clear", builtin::dictionary::clear).bind(self.clone()),
                _ => panic!("Function with name '{}' not found on dict", name),
            }),
            DataType::List(_) => Callable::BuiltIn(match name {
                "pop" => Builtin::new("pop", builtin::list::pop).bind(self.clone()),
                "push" => Builtin::new("push", builtin::list::push).bind(self.clone()),
                "clear" => Builtin::new("clear", builtin::list::clear).bind(self.clone()),
                _ => panic!("Function with name '{}' not found on list", name),
            }),
            DataType::Module(module) => {
                let module_fn = module.functions.get(name);

                match module_fn {
                    Some(function) => {
                        Callable::Module(ModuleFunction::new(name.to_string(), function.clone()))
                    }
                    _ => panic!(
                        "Function with name '{}' not found on module {}",
                        name, module.name
                    ),
                }
            }
            _ => panic!("No methods available on {}", self),
        }))
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            DataType::Number(x) => format!("{}", x),
            DataType::String(x) => format!("\"{}\"", x),
            DataType::Boolean(x) => format!("{}", x),
            DataType::Function(function_declaration) => format!("{}", function_declaration),
            DataType::List(values) => format!("{}", values),
            DataType::Dictionary(entries) => format!("{}", entries),
            DataType::Module(module) => format!("{}", module),
            DataType::Undefined => "undefined".to_string(),
        };
        write!(f, "{}", string)
    }
}
