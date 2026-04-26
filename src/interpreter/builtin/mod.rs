use crate::interpreter::DataType;
use std::{fmt, rc::Rc};

pub(crate) mod dictionary;
pub(crate) mod global;
pub(crate) mod list;

#[derive(Debug, Clone)]
pub struct Builtin {
    pub name: &'static str,
    receiver: Option<Rc<DataType>>,
    function: BuiltinFn,
}

impl fmt::Display for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "builtin {}", self.name)
    }
}

pub type BuiltinFn = fn(Option<Rc<DataType>>, Vec<Rc<DataType>>) -> Rc<DataType>;

impl Builtin {
    pub fn new(name: &'static str, function: BuiltinFn) -> Self {
        Self {
            name,
            function,
            receiver: None,
        }
    }

    pub fn bind(&self, receiver: Rc<DataType>) -> Self {
        Self {
            name: self.name,
            function: self.function,
            receiver: Some(receiver),
        }
    }

    pub fn execute(&self, parameters: Vec<Rc<DataType>>) -> Rc<DataType> {
        (self.function)(self.receiver.clone(), parameters)
    }
}
