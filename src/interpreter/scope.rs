use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use crate::interpreter::{
    builtin::{CallInfo, ExecutionError},
    datatype::DataType,
};

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

    pub fn get_variable(&self, identifier: &String) -> Result<Rc<DataType>, ExecutionError> {
        if let Some(var) = self.variables.get(identifier) {
            return Ok(var.clone());
        }

        match &self.parent {
            Some(parent) => parent.borrow().get_variable(identifier),
            None => Err(ExecutionError::new(
                CallInfo::new(""),
                format!("Variable '{}' is not declared", identifier).as_str(),
            )),
        }
    }

    pub fn set_variable(
        &mut self,
        identifier: String,
        data: Rc<DataType>,
    ) -> Result<(), ExecutionError> {
        if self.variables.contains_key(&identifier) {
            return Err(ExecutionError::new(
                CallInfo::new(""),
                format!("Duplicate identifier '{}' already declared", &identifier).as_str(),
            ));
        }

        self.variables.insert(identifier, data);
        Ok(())
    }

    pub fn update_variable(
        &mut self,
        identifier: &String,
        data: Rc<DataType>,
    ) -> Result<(), ExecutionError> {
        if !self.variables.contains_key(identifier) {
            match &self.parent {
                Some(parent) => parent.borrow_mut().update_variable(identifier, data)?,
                None => {
                    return Err(ExecutionError::new(
                        CallInfo::new(""),
                        format!("Identifier '{}' has not declared", &identifier).as_str(),
                    ));
                }
            };
        } else {
            self.variables.insert(identifier.clone(), data);
        }

        Ok(())
    }
}
