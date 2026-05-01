use std::{cell::RefCell, fmt, rc::Rc};

use crate::interpreter::{
    builtin::{CallInfo, ExecutionError},
    coerce::{Args, DataKind},
    datatype::DataType,
};

#[derive(Debug, PartialEq, Clone)]
pub struct ListDeclaration {
    pub values: Rc<RefCell<Vec<Rc<DataType>>>>,
}

impl ListDeclaration {
    pub fn new(values: Vec<Rc<DataType>>) -> Self {
        Self {
            values: Rc::new(RefCell::new(values)),
        }
    }

    pub fn get(&self, index: Rc<DataType>) -> Result<Rc<DataType>, ExecutionError> {
        let args = Args::new("get", &vec![index]);
        let i = args.int(0)?;

        let values = self.values.borrow();
        match i < values.len() {
            true => Ok(Rc::clone(&values[i])),
            false => Err(ExecutionError::new(
                CallInfo::new("get"),
                "Index out of bounds",
            )),
        }
    }

    pub fn set(&self, index: Rc<DataType>, value: Rc<DataType>) -> Result<(), ExecutionError> {
        let args = Args::new("get", &vec![index]);
        let i = args.int(0)?;

        let mut values = self.values.borrow_mut();
        match i < values.len() {
            true => {
                values[i] = value;
                Ok(())
            }
            false => Err(ExecutionError::new(
                CallInfo::new("get"),
                "Index out of bounds",
            )),
        }
    }

    pub fn all(&self, kind: DataKind) -> bool {
        self.values
            .borrow()
            .iter()
            .all(|value| DataKind::from(&**value) == kind)
    }

    pub fn length(&self) -> Rc<DataType> {
        Rc::new(DataType::Number(self.values.borrow().len() as f32))
    }

    pub fn clear(&self) {
        self.values.borrow_mut().clear();
    }

    pub fn pop(&self) -> Option<Rc<DataType>> {
        self.values.borrow_mut().pop()
    }

    pub fn push(&self, value: Rc<DataType>) {
        self.values.borrow_mut().push(value);
    }
}

impl fmt::Display for ListDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;

        for (i, value) in self.values.borrow().iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", value)?;
        }

        write!(f, "]")
    }
}
