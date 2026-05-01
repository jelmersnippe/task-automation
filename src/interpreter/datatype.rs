use std::{fmt, rc::Rc, sync::Arc};

use crate::{
    RuntimeContext,
    interpreter::{
        builtin::{self, BuiltinFn, CallInfo, Executable, ExecutionError},
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

const DICTIONARY_METHODS: (&str, &[(&str, BuiltinFn)]) = (
    "Dictionary",
    &[
        ("has", builtin::dictionary::has),
        ("delete", builtin::dictionary::delete),
        ("clear", builtin::dictionary::clear),
    ],
);

const LIST_METHODS: (&str, &[(&str, BuiltinFn)]) = (
    "List",
    &[
        ("pop", builtin::list::pop),
        ("push", builtin::list::push),
        ("clear", builtin::list::clear),
    ],
);

impl DataType {
    pub(crate) fn get_method(
        self: &Rc<DataType>,
        name: &str,
    ) -> Result<Rc<DataType>, ExecutionError> {
        let call_info = CallInfo::new(name);

        let (type_name, methods) = match &self.as_ref() {
            DataType::List(_) => LIST_METHODS,
            DataType::Dictionary(_) => DICTIONARY_METHODS,
            DataType::Module(module) => {
                let module_fn = module.functions.get(name).ok_or_else(|| {
                    return ExecutionError::new(
                        call_info,
                        &format!("Method '{}' not found for module '{}'", &name, &module.name),
                    );
                })?;

                return Ok(Rc::new(DataType::Function(module_fn.clone())));
            }
            _ => {
                return Err(ExecutionError::new(
                    call_info,
                    &format!("No methods found for '{}'", &self),
                ));
            }
        };

        let function = methods
            .iter()
            .find(|(method_name, _)| *method_name == name)
            .map(|(_, function)| {
                Callable::new(Some(name.to_string()), *function).bind(self.clone())
            })
            .ok_or_else(|| {
                return ExecutionError::new(
                    call_info,
                    &format!("Method '{}' not found on '{}'", name, type_name),
                );
            })?;

        Ok(Rc::new(DataType::Function(function)))
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
