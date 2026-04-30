use crate::{RuntimeContext, interpreter::datatype::DataType};
use std::{fmt, rc::Rc, sync::Arc};

pub(crate) mod dictionary;
pub(crate) mod global;
pub(crate) mod list;

pub type BuiltinFn =
    fn(Option<Rc<DataType>>, Vec<Rc<DataType>>, &mut RuntimeContext) -> Rc<DataType>;
pub type Executable =
    Arc<dyn Fn(Option<Rc<DataType>>, Vec<Rc<DataType>>, &mut RuntimeContext) -> Rc<DataType>>;

#[derive(Clone)]
pub struct Builtin {
    pub name: &'static str,
    receiver: Option<Rc<DataType>>,
    function: Executable,
}

impl fmt::Debug for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Builtin({})", self.name)
    }
}

impl fmt::Display for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "builtin {}", self.name)
    }
}

impl Builtin {
    pub fn new(name: &'static str, function: BuiltinFn) -> Self {
        Self {
            name,
            function: Arc::new(function),
            receiver: None,
        }
    }

    pub fn bind(&self, receiver: Rc<DataType>) -> Self {
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
    ) -> Rc<DataType> {
        (self.function)(self.receiver.clone(), parameters, context)
    }
}
