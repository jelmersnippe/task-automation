use std::fmt;

use crate::interpreter::{
    datatype::{Callable, DataType, SharedDataType},
    dictionary::DictionaryDeclaration,
    list::ListDeclaration,
};

pub fn expect_string(data: &DataType) -> Result<String, DataKind> {
    match data {
        DataType::Number(x) => Ok(x.to_string()),
        DataType::String(x) => Ok(x.clone()),
        DataType::Boolean(x) => Ok(x.to_string()),
        _ => Err(DataKind::from(data)),
    }
}

pub fn expect_callable(data: &DataType) -> Result<&Callable, DataKind> {
    if let DataType::Function(callable) = data {
        return Ok(callable);
    }

    Err(DataKind::from(data))
}

pub fn expect_dict(data: &DataType) -> Result<&DictionaryDeclaration, DataKind> {
    match data {
        DataType::Dictionary(x) => Ok(x),
        _ => Err(DataKind::from(data)),
    }
}

pub fn expect_bool(data: &DataType) -> Result<bool, DataKind> {
    match data {
        DataType::Boolean(x) => Ok(*x),
        _ => Err(DataKind::from(data)),
    }
}

pub fn expect_list(data: &DataType) -> Result<&ListDeclaration, DataKind> {
    match data {
        DataType::List(x) => Ok(x),
        _ => Err(DataKind::from(data)),
    }
}

pub fn expect_int(data: &DataType) -> Result<usize, DataKind> {
    if let DataType::Number(number) = data {
        let i = number.round() as usize;
        if *number as usize != i {
            return Err(DataKind::Float);
        }

        return Ok(*number as usize);
    }

    Err(DataKind::from(data))
}

#[derive(Debug, PartialEq)]
pub enum DataKind {
    String,
    Boolean,
    Int,
    Float,
    List,
    Dictionary,
    Callable,
    Module,
    Undefined,
}

impl From<&DataType> for DataKind {
    fn from(value: &DataType) -> Self {
        match value {
            DataType::Number(_) => DataKind::Int,
            DataType::String(_) => DataKind::String,
            DataType::Boolean(_) => DataKind::Boolean,
            DataType::Function(_) => DataKind::Callable,
            DataType::List(_) => DataKind::List,
            DataType::Dictionary(_) => DataKind::Dictionary,
            DataType::Module(_) => DataKind::Module,
            DataType::Undefined => DataKind::Undefined,
        }
    }
}

#[derive(Debug)]
pub enum ArgumentError {
    InvalidCount {
        fn_name: String,
        expected: usize,
        found: usize,
    },
    InvalidRange {
        fn_name: String,
        expected_min: usize,
        expected_max: usize,
        found: usize,
    },
    InvalidType {
        fn_name: String,
        index: usize,
        expected_type: DataKind,
        found_type: DataKind,
    },
    MissingKey {
        fn_name: String,
        key: String,
        expected_type: DataKind,
    },
    InvalidKeyType {
        fn_name: String,
        key: String,
        expected_type: DataKind,
        found_type: DataKind,
    },
}

impl fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArgumentError::InvalidCount {
                fn_name,
                expected,
                found,
            } => write!(
                f,
                "Argument count error. {} expected {} arguments, found: {}",
                fn_name, expected, found
            ),
            ArgumentError::InvalidRange {
                fn_name,
                expected_min,
                expected_max,
                found,
            } => write!(
                f,
                "Argument range error. {} expected {}-{} arguments, found: {}",
                fn_name, expected_min, expected_max, found
            ),
            ArgumentError::InvalidType {
                fn_name,
                index,
                expected_type,
                found_type,
            } => write!(
                f,
                "Argument type error. {} expected {:?} as argument {}, found: {:?}",
                fn_name, expected_type, index, found_type
            ),
            ArgumentError::MissingKey {
                fn_name,
                key,
                expected_type,
            } => write!(
                f,
                "Dictionary error. {} expected key \"{}\" of type {:?}, but it was not found",
                fn_name, key, expected_type
            ),
            ArgumentError::InvalidKeyType {
                fn_name,
                key,
                expected_type,
                found_type,
            } => write!(
                f,
                "Dictionary type error. {} expected key \"{}\" to be {:?}, found: {:?}",
                fn_name, key, expected_type, found_type
            ),
        }
    }
}

pub trait OptionalValue<T> {
    fn optional(self) -> Result<Option<T>, ArgumentError>;
}

impl<T> OptionalValue<T> for Result<T, ArgumentError> {
    fn optional(self) -> Result<Option<T>, ArgumentError> {
        match self {
            Ok(v) => Ok(Some(v)),
            Err(ArgumentError::MissingKey { .. }) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

pub struct Args {
    fn_name: String,
    pub arguments: Vec<SharedDataType>,
}

impl Args {
    pub fn new(fn_name: &str, arguments: &Vec<SharedDataType>) -> Self {
        Self {
            fn_name: fn_name.to_string(),
            arguments: arguments.iter().cloned().collect(),
        }
    }

    pub fn exact(&self, length: usize) -> Result<(), ArgumentError> {
        let len = self.arguments.len();
        if len != length {
            return Err(ArgumentError::InvalidCount {
                fn_name: self.fn_name.clone(),
                found: len,
                expected: length,
            });
        }

        Ok(())
    }
    pub fn range(&self, min: usize, max: usize) -> Result<(), ArgumentError> {
        let len = self.arguments.len();
        if len < min || len > max {
            return Err(ArgumentError::InvalidRange {
                fn_name: self.fn_name.clone(),
                found: len,
                expected_min: min,
                expected_max: max,
            });
        }

        Ok(())
    }

    pub fn optional_string(&self, index: usize) -> Result<Option<String>, ArgumentError> {
        self.arguments
            .get(index)
            .map(|data| {
                expect_string(data).map_err(|found| ArgumentError::InvalidType {
                    fn_name: self.fn_name.clone(),
                    index,
                    expected_type: DataKind::String,
                    found_type: found,
                })
            })
            .transpose()
    }

    pub fn string(&self, index: usize) -> Result<String, ArgumentError> {
        let value = self
            .arguments
            .get(index)
            .ok_or(ArgumentError::InvalidType {
                fn_name: self.fn_name.clone(),
                index,
                expected_type: DataKind::String,
                found_type: DataKind::Undefined,
            })?;
        expect_string(value).map_err(|found| ArgumentError::InvalidType {
            fn_name: self.fn_name.clone(),
            index,
            expected_type: DataKind::String,
            found_type: found,
        })
    }
    pub fn int(&self, index: usize) -> Result<usize, ArgumentError> {
        let value = self
            .arguments
            .get(index)
            .ok_or(ArgumentError::InvalidType {
                fn_name: self.fn_name.clone(),
                index,
                expected_type: DataKind::Int,
                found_type: DataKind::Undefined,
            })?;
        expect_int(value).map_err(|found| ArgumentError::InvalidType {
            fn_name: self.fn_name.clone(),
            index,
            expected_type: DataKind::Int,
            found_type: found,
        })
    }
    pub fn boolean(&self, index: usize) -> Result<bool, ArgumentError> {
        let value = self
            .arguments
            .get(index)
            .ok_or(ArgumentError::InvalidType {
                fn_name: self.fn_name.clone(),
                index,
                expected_type: DataKind::Boolean,
                found_type: DataKind::Undefined,
            })?;
        expect_bool(value).map_err(|found| ArgumentError::InvalidType {
            fn_name: self.fn_name.clone(),
            index,
            expected_type: DataKind::Boolean,
            found_type: found,
        })
    }
    pub fn list(&self, index: usize) -> Result<&ListDeclaration, ArgumentError> {
        let value = self
            .arguments
            .get(index)
            .ok_or(ArgumentError::InvalidType {
                fn_name: self.fn_name.clone(),
                index,
                expected_type: DataKind::List,
                found_type: DataKind::Undefined,
            })?;
        expect_list(value).map_err(|found| ArgumentError::InvalidType {
            fn_name: self.fn_name.clone(),
            index,
            expected_type: DataKind::List,
            found_type: found,
        })
    }
    pub fn dictionary(&self, index: usize) -> Result<&DictionaryDeclaration, ArgumentError> {
        let value = self
            .arguments
            .get(index)
            .ok_or(ArgumentError::InvalidType {
                fn_name: self.fn_name.clone(),
                index,
                expected_type: DataKind::Dictionary,
                found_type: DataKind::Undefined,
            })?;
        expect_dict(value).map_err(|found| ArgumentError::InvalidType {
            fn_name: self.fn_name.clone(),
            index,
            expected_type: DataKind::Dictionary,
            found_type: found,
        })
    }
    pub fn callable(&self, index: usize) -> Result<&Callable, ArgumentError> {
        let value = self
            .arguments
            .get(index)
            .ok_or(ArgumentError::InvalidType {
                fn_name: self.fn_name.clone(),
                index,
                expected_type: DataKind::Callable,
                found_type: DataKind::Undefined,
            })?;
        expect_callable(value).map_err(|found| ArgumentError::InvalidType {
            fn_name: self.fn_name.clone(),
            index,
            expected_type: DataKind::Callable,
            found_type: found,
        })
    }
    pub fn any(&self, index: usize) -> Result<&SharedDataType, ArgumentError> {
        self.arguments.get(index).ok_or(ArgumentError::InvalidType {
            // TODO: Fix proper error type here. Index accessor gone wrong
            fn_name: self.fn_name.clone(),
            index,
            expected_type: DataKind::Callable,
            found_type: DataKind::Undefined,
        })
    }
}

pub struct DictArgs<'a> {
    fn_name: String,
    dict: &'a DictionaryDeclaration,
}

impl<'a> DictArgs<'a> {
    pub fn new(fn_name: &str, dict: &'a DictionaryDeclaration) -> Self {
        Self {
            fn_name: fn_name.to_string(),
            dict,
        }
    }

    fn get(&self, key: &str) -> SharedDataType {
        self.dict.get(key)
    }

    pub fn string(&self, key: &str) -> Result<String, ArgumentError> {
        let value = self.get(key);
        if matches!(*value, DataType::Undefined) {
            return Err(ArgumentError::MissingKey {
                fn_name: self.fn_name.clone(),
                key: key.to_string(),
                expected_type: DataKind::String,
            });
        }
        expect_string(&value).map_err(|found| ArgumentError::InvalidKeyType {
            fn_name: self.fn_name.clone(),
            key: key.to_string(),
            expected_type: DataKind::String,
            found_type: found,
        })
    }

    pub fn int(&self, key: &str) -> Result<usize, ArgumentError> {
        let value = self.get(key);
        if matches!(*value, DataType::Undefined) {
            return Err(ArgumentError::MissingKey {
                fn_name: self.fn_name.clone(),
                key: key.to_string(),
                expected_type: DataKind::Int,
            });
        }
        expect_int(&value).map_err(|found| ArgumentError::InvalidKeyType {
            fn_name: self.fn_name.clone(),
            key: key.to_string(),
            expected_type: DataKind::Int,
            found_type: found,
        })
    }

    pub fn boolean(&self, key: &str) -> Result<bool, ArgumentError> {
        let value = self.get(key);
        if matches!(*value, DataType::Undefined) {
            return Err(ArgumentError::MissingKey {
                fn_name: self.fn_name.clone(),
                key: key.to_string(),
                expected_type: DataKind::Boolean,
            });
        }
        expect_bool(&value).map_err(|found| ArgumentError::InvalidKeyType {
            fn_name: self.fn_name.clone(),
            key: key.to_string(),
            expected_type: DataKind::Boolean,
            found_type: found,
        })
    }

    pub fn list(&self, key: &str) -> Result<SharedDataType, ArgumentError> {
        let value = self.get(key);
        if matches!(*value, DataType::Undefined) {
            return Err(ArgumentError::MissingKey {
                fn_name: self.fn_name.clone(),
                key: key.to_string(),
                expected_type: DataKind::List,
            });
        }
        expect_list(&value).map_err(|found| ArgumentError::InvalidKeyType {
            fn_name: self.fn_name.clone(),
            key: key.to_string(),
            expected_type: DataKind::List,
            found_type: found,
        })?;
        Ok(value)
    }

    pub fn dictionary(&self, key: &str) -> Result<SharedDataType, ArgumentError> {
        let value = self.get(key);
        if matches!(*value, DataType::Undefined) {
            return Err(ArgumentError::MissingKey {
                fn_name: self.fn_name.clone(),
                key: key.to_string(),
                expected_type: DataKind::Dictionary,
            });
        }
        expect_dict(&value).map_err(|found| ArgumentError::InvalidKeyType {
            fn_name: self.fn_name.clone(),
            key: key.to_string(),
            expected_type: DataKind::Dictionary,
            found_type: found,
        })?;
        Ok(value)
    }

    pub fn callable(&self, key: &str) -> Result<SharedDataType, ArgumentError> {
        let value = self.get(key);
        if matches!(*value, DataType::Undefined) {
            return Err(ArgumentError::MissingKey {
                fn_name: self.fn_name.clone(),
                key: key.to_string(),
                expected_type: DataKind::Callable,
            });
        }
        expect_callable(&value).map_err(|found| ArgumentError::InvalidKeyType {
            fn_name: self.fn_name.clone(),
            key: key.to_string(),
            expected_type: DataKind::Callable,
            found_type: found,
        })?;
        Ok(value)
    }
}
