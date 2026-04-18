use std::{collections::HashMap, fmt, rc::Rc};

use crate::interpreter::builtin::get_builtins;

#[derive(PartialEq, Debug, Clone)]
pub enum DataType {
    Number(f32),
    String(String),
    Boolean(bool),
    Function(super::function::FunctionDeclaration),
    List(super::list::ListDeclaration),
    Void(),
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            DataType::Number(x) => format!("{}", x),
            DataType::String(x) => format!("\"{}\"", x),
            DataType::Boolean(x) => format!("{}", x),
            DataType::Function(function_declaration) => format!("{}", function_declaration),
            DataType::List(data_types) => format!("{}", data_types),
            DataType::Void() => "void".to_string(),
        };
        write!(f, "{}", string)
    }
}

pub struct Scope<'a> {
    parent: Option<&'a Scope<'a>>,
    variables: HashMap<String, Rc<DataType>>,
}

impl<'a> Scope<'a> {
    pub fn new(parent: Option<&'a Scope<'a>>) -> Self {
        Self {
            parent: parent,
            variables: Default::default(),
        }
    }

    pub fn get_variable(&self, identifier: &String) -> Option<Rc<DataType>> {
        if let Some(var) = self.variables.get(identifier) {
            return Some(Rc::clone(var));
        }

        return match self.parent {
            Some(parent) => parent.get_variable(identifier),
            None => None,
        };
    }

    pub fn set_variable(&mut self, identifier: String, data: Rc<DataType>) {
        if get_builtins().contains_key(&identifier.as_str()) {
            panic!("Can't override builtin '{}'", &identifier)
        }

        if self.variables.contains_key(&identifier) {
            panic!("Duplicate identifier '{}' already declared", &identifier);
        }

        self.variables.insert(identifier, data);
    }

    pub fn update_variable(&mut self, identifier: String, data: Rc<DataType>) {
        if !self.variables.contains_key(&identifier) {
            panic!("Identifier '{}' has not declared", &identifier);
        }

        self.variables.insert(identifier, data);
    }
}
