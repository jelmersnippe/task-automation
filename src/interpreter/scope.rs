use std::{collections::HashMap, fmt, rc::Rc};

use crate::interpreter::builtin::{Builtin, dict_clear, dict_delete, dict_has};

#[derive(Debug, Clone)]
pub enum DataType {
    Number(f32),
    String(String),
    Boolean(bool),
    Function(super::function::FunctionDeclaration),
    List(super::list::ListDeclaration),
    Dictionary(super::list::DictionaryDeclaration),
    Builtin(Builtin),
    Undefined(),
}

impl PartialEq for DataType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::Function(l0), Self::Function(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Dictionary(l0), Self::Dictionary(r0)) => l0 == r0,
            (Self::Undefined(), Self::Undefined()) => true,
            _ => false,
        }
    }
}

impl DataType {
    pub(crate) fn get_builtins(&self) -> Vec<Builtin> {
        match self {
            DataType::Dictionary(_) => {
                vec![
                    Builtin::new(String::from("has"), dict_has).bind(Rc::new(self.clone())),
                    Builtin::new(String::from("delete"), dict_delete).bind(Rc::new(self.clone())),
                    Builtin::new(String::from("clear"), dict_clear).bind(Rc::new(self.clone())),
                ]
            }
            _ => vec![],
        }
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
            DataType::Undefined() => "undefined".to_string(),
            DataType::Builtin(builtin) => format!("{:?}", builtin),
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

    pub fn get_variable(&self, identifier: &String) -> Rc<DataType> {
        if let Some(var) = self.variables.get(identifier) {
            return Rc::clone(var);
        }

        match self.parent {
            Some(parent) => parent.get_variable(identifier),
            None => Rc::new(DataType::Undefined()),
        }
    }

    pub fn set_variable(&mut self, identifier: String, data: Rc<DataType>) {
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
