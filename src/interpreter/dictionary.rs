use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, Mutex},
};

use crate::interpreter::{
    builtin::{CallInfo, ExecutionError},
    coerce::Args,
    datatype::{DataType, SharedDataType},
};

#[derive(Debug, Clone)]
pub struct DictionaryDeclaration {
    entries: Arc<Mutex<HashMap<String, SharedDataType>>>,
}

impl PartialEq for DictionaryDeclaration {
    fn eq(&self, other: &Self) -> bool {
        if Arc::ptr_eq(&self.entries, &other.entries) {
            return true;
        }
        *self.entries.lock().unwrap() == *other.entries.lock().unwrap()
    }
}

impl DictionaryDeclaration {
    pub fn new(entries: HashMap<String, SharedDataType>) -> Self {
        Self {
            entries: Arc::new(Mutex::new(entries)),
        }
    }

    pub fn has(&self, key: &String) -> bool {
        self.entries.lock().unwrap().contains_key(key)
    }

    pub fn get(&self, key: &String) -> Result<SharedDataType, ExecutionError> {
        let binding = self.entries.lock().unwrap();
        let value = binding.get(key);

        match value {
            Some(data) => Ok(data.clone()),
            None => Err(ExecutionError::new(
                CallInfo::new(""),
                format!("Dict does not have key '{}'", key).as_str(),
            )),
        }
    }

    pub fn set(&self, key: SharedDataType, value: SharedDataType) -> Result<(), ExecutionError> {
        let args = Args::new("set", &vec![key]);
        let key_string = args.string(0)?;
        self.entries.lock().unwrap().insert(key_string, value);
        Ok(())
    }

    pub fn delete(&self, key: &String) {
        self.entries.lock().unwrap().remove(key);
    }

    pub fn clear(&self) {
        self.entries.lock().unwrap().clear();
    }

    pub fn length(&self) -> SharedDataType {
        (DataType::Number(self.entries.lock().unwrap().len() as f32)).to_shared()
    }
}

impl fmt::Display for DictionaryDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;

        for (i, (key, value)) in self.entries.lock().unwrap().iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", key, value)?;
        }

        write!(f, "}}")
    }
}
