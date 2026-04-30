use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use crate::interpreter::{
    builtin::{CallInfo, ExecutionError},
    coerce::Args,
    datatype::DataType,
};

#[derive(Debug, PartialEq, Clone)]
pub struct DictionaryDeclaration {
    entries: Rc<RefCell<HashMap<String, Rc<DataType>>>>,
}

impl DictionaryDeclaration {
    pub fn new(entries: HashMap<String, Rc<DataType>>) -> Self {
        Self {
            entries: Rc::new(RefCell::new(entries)),
        }
    }

    pub fn has(&self, key: &String) -> bool {
        self.entries.borrow().contains_key(key)
    }

    pub fn get(&self, key: &String) -> Result<Rc<DataType>, ExecutionError> {
        let binding = self.entries.borrow();
        let value = binding.get(key);

        match value {
            Some(data) => Ok(Rc::clone(data)),
            None => Err(ExecutionError::new(
                CallInfo::new(""),
                format!("Dict does not have key '{}'", key).as_str(),
            )),
        }
    }

    pub fn set(&self, key: Rc<DataType>, value: Rc<DataType>) -> Result<(), ExecutionError> {
        let args = Args::new("set", &vec![key]);
        let key_string = args.string(0)?;
        self.entries.borrow_mut().insert(key_string, value);
        Ok(())
    }

    pub fn delete(&self, key: &String) {
        self.entries.borrow_mut().remove(key);
    }

    pub fn clear(&self) {
        self.entries.borrow_mut().clear();
    }

    pub fn length(&self) -> Rc<DataType> {
        Rc::new(DataType::Number(self.entries.borrow().len() as f32))
    }
}

impl fmt::Display for DictionaryDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;

        for (i, (key, value)) in self.entries.borrow().iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", key, value)?;
        }

        write!(f, "}}")
    }
}
