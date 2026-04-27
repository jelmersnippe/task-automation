use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use crate::interpreter::datatype::DataType;

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
