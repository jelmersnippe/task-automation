use std::{
    fmt,
    sync::{Arc, Mutex},
};

use crate::interpreter::{
    builtin::{CallInfo, ExecutionError},
    coerce::{Args, DataKind},
    datatype::{DataType, SharedDataType},
};

#[derive(Debug, Clone)]
pub struct ListDeclaration {
    pub values: Arc<Mutex<Vec<SharedDataType>>>,
}

impl PartialEq for ListDeclaration {
    fn eq(&self, other: &Self) -> bool {
        if Arc::ptr_eq(&self.values, &other.values) {
            return true;
        }
        *self.values.lock().unwrap() == *other.values.lock().unwrap()
    }
}

impl ListDeclaration {
    pub fn new(values: Vec<SharedDataType>) -> Self {
        Self {
            values: Arc::new(Mutex::new(values)),
        }
    }

    pub fn get(&self, index: SharedDataType) -> Result<SharedDataType, ExecutionError> {
        let args = Args::new("get", &vec![index]);
        let i = args.int(0)?;

        let values = self.values.lock().unwrap();
        match i < values.len() {
            true => Ok(values[i].clone()),
            false => Err(ExecutionError::new(
                CallInfo::new("get"),
                "Index out of bounds",
            )),
        }
    }

    pub fn set(&self, index: SharedDataType, value: SharedDataType) -> Result<(), ExecutionError> {
        let args = Args::new("get", &vec![index]);
        let i = args.int(0)?;

        let mut values = self.values.lock().unwrap();
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
            .lock()
            .unwrap()
            .iter()
            .all(|value| DataKind::from(&**value) == kind)
    }

    pub fn length(&self) -> SharedDataType {
        (DataType::Number(self.values.lock().unwrap().len() as f32)).to_shared()
    }

    pub fn clear(&self) {
        self.values.lock().unwrap().clear();
    }

    pub fn pop(&self) -> Option<SharedDataType> {
        self.values.lock().unwrap().pop()
    }

    pub fn push(&self, value: SharedDataType) {
        self.values.lock().unwrap().push(value);
    }
}

impl fmt::Display for ListDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;

        for (i, value) in self.values.lock().unwrap().iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", value)?;
        }

        write!(f, "]")
    }
}
