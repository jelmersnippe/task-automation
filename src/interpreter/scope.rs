use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use crate::{
    RuntimeContext,
    interpreter::{
        Parameters,
        builtin::{self, Builtin},
        dictionary::DictionaryDeclaration,
        function::FunctionDeclaration,
        list::ListDeclaration,
    },
};

#[derive(Debug, Clone)]
pub enum DataType {
    Number(f32),
    String(String),
    Boolean(bool),
    Function(Callable),
    List(ListDeclaration),
    Dictionary(DictionaryDeclaration),
    Undefined,
}

#[derive(Debug, Clone)]
pub enum Callable {
    BuiltIn(Builtin),
    User(FunctionDeclaration),
}

impl fmt::Display for Callable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Callable::BuiltIn(builtin) => write!(f, "{}", builtin),
            Callable::User(function_declaration) => write!(f, "{}", function_declaration),
        }
    }
}

impl Callable {
    pub fn execute(
        &self,
        parameters: &Parameters,
        scope: Rc<RefCell<Scope>>,
        context: &RuntimeContext,
    ) -> Rc<DataType> {
        match self {
            Callable::BuiltIn(builtin) => {
                builtin.execute(parameters.resolve(scope.clone(), context))
            }
            Callable::User(function_declaration) => {
                function_declaration.execute(parameters.resolve(scope.clone(), context), context)
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
        Rc::new(DataType::Function(Callable::BuiltIn(match self.as_ref() {
            DataType::Dictionary(_) => match name {
                "has" => Builtin::new("has", builtin::dictionary::has).bind(self.clone()),
                "delete" => Builtin::new("delete", builtin::dictionary::delete).bind(self.clone()),
                "clear" => Builtin::new("clear", builtin::dictionary::clear).bind(self.clone()),
                _ => panic!("Method with name '{}' not found on dict", name),
            },
            DataType::List(_) => match name {
                "pop" => Builtin::new("pop", builtin::list::pop).bind(self.clone()),
                "push" => Builtin::new("push", builtin::list::push).bind(self.clone()),
                "clear" => Builtin::new("clear", builtin::list::clear).bind(self.clone()),
                _ => panic!("Method with name '{}' not found on list", name),
            },
            _ => panic!("No methods available on {}", self),
        })))
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
            DataType::Undefined => "undefined".to_string(),
        };
        write!(f, "{}", string)
    }
}

#[derive(Debug, PartialEq)]
pub struct Scope {
    parent: Option<Rc<RefCell<Scope>>>,
    variables: HashMap<String, Rc<DataType>>,
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\n")?;

        for (k, v) in self.variables.iter() {
            write!(f, "{}: {},\n", k, v)?;
        }
        write!(f, "}}")
    }
}

impl Scope {
    pub fn new(parent: Option<Rc<RefCell<Scope>>>) -> Self {
        Self {
            parent: parent,
            variables: Default::default(),
        }
    }

    pub fn get_variable(&self, identifier: &String) -> Rc<DataType> {
        if let Some(var) = self.variables.get(identifier) {
            return var.clone();
        }

        match &self.parent {
            Some(parent) => parent.borrow().get_variable(identifier),
            None => panic!("Variable '{}' is not declared", identifier),
        }
    }

    pub fn set_variable(&mut self, identifier: String, data: Rc<DataType>) {
        if self.variables.contains_key(&identifier) {
            panic!("Duplicate identifier '{}' already declared", &identifier);
        }

        self.variables.insert(identifier, data);
    }

    pub fn update_variable(&mut self, identifier: &String, data: Rc<DataType>) {
        if !self.variables.contains_key(identifier) {
            match &self.parent {
                Some(parent) => parent.borrow_mut().update_variable(identifier, data),
                None => panic!("Identifier '{}' has not declared", &identifier),
            }
        } else {
            self.variables.insert(identifier.clone(), data);
        }
    }
}
