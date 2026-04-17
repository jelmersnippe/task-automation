use std::{collections::HashMap, rc::Rc};

use crate::interpreter::builtin::get_builtins;

#[derive(PartialEq, Debug, Clone)]
pub enum DataType {
    Number(f32),
    String(String),
    Boolean(bool),
    Function(super::function::FunctionDeclaration),
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
