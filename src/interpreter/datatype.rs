use std::{fmt, rc::Rc, sync::Arc};

use crate::{
    RuntimeContext,
    interpreter::{
        builtin::{self, BuiltinFn, Executable, ExecutionError},
        dictionary::DictionaryDeclaration,
        list::ListDeclaration,
    },
    modules::Module,
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

#[derive(Clone)]
pub struct Callable {
    name: Option<String>,
    function: Executable,
    receiver: Option<Rc<DataType>>,
}

impl fmt::Debug for Callable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Callable")
            .field("name", &self.name)
            .finish()
    }
}

impl fmt::Display for Callable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fn ")?;

        if let Some(name) = &self.name {
            write!(f, "{}", name)?;
        }

        write!(f, "() {{ }}")
    }
}

impl Callable {
    pub fn new(name: Option<String>, function: BuiltinFn) -> Self {
        Self {
            name,
            function: Arc::new(function),
            receiver: None,
        }
    }
    pub fn from_executable(name: Option<String>, executable: Executable) -> Self {
        Self {
            name,
            function: executable,
            receiver: None,
        }
    }
    pub fn bind(self, receiver: Rc<DataType>) -> Self {
        Self {
            name: self.name,
            function: self.function.clone(),
            receiver: Some(receiver),
        }
    }
    pub fn execute(
        &self,
        parameters: Vec<Rc<DataType>>,
        context: &mut RuntimeContext,
    ) -> Result<Rc<DataType>, ExecutionError> {
        (self.function)(self.receiver.clone(), parameters, context)
    }
}

impl PartialEq for DataType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::Function(l0), Self::Function(r0)) => l0.name == r0.name,
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
            DataType::Dictionary(_) => match name {
                "has" => Callable::new(Some(String::from("has")), builtin::dictionary::has)
                    .bind(self.clone()),
                "delete" => {
                    Callable::new(Some(String::from("delete")), builtin::dictionary::delete)
                        .bind(self.clone())
                }
                "clear" => Callable::new(Some(String::from("clear")), builtin::dictionary::clear)
                    .bind(self.clone()),
                _ => panic!("Function with name '{}' not found on dict", name),
            },
            DataType::List(_) => match name {
                "pop" => {
                    Callable::new(Some(String::from("pop")), builtin::list::pop).bind(self.clone())
                }
                "push" => Callable::new(Some(String::from("push")), builtin::list::push)
                    .bind(self.clone()),
                "clear" => Callable::new(Some(String::from("clear")), builtin::list::clear)
                    .bind(self.clone()),
                _ => panic!("Function with name '{}' not found on list", name),
            },
            DataType::Module(module) => {
                let module_fn = module.functions.get(name);

                match module_fn {
                    Some(function) => function.clone(),
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
