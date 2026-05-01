use crate::{
    RuntimeContext,
    interpreter::{coerce::ArgumentError, datatype::SharedDataType},
    modules::GitError,
};
use std::{fmt, sync::Arc};

pub(crate) mod dictionary;
pub(crate) mod global;
pub(crate) mod list;

#[derive(Debug)]
pub struct CallInfo {
    name: String,
}

impl CallInfo {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct ExecutionError {
    call_info: CallInfo,
    reason: String,
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Exeucting '{}' failed: {}",
            self.call_info.name, self.reason
        )
    }
}

impl From<GitError> for ExecutionError {
    fn from(value: GitError) -> Self {
        Self {
            call_info: CallInfo::new(&value.command),
            reason: value.reason.to_string(),
        }
    }
}

impl From<ArgumentError> for ExecutionError {
    fn from(value: ArgumentError) -> Self {
        Self {
            call_info: CallInfo::new(match &value {
                ArgumentError::InvalidCount { fn_name, .. } => fn_name.as_str(),
                ArgumentError::InvalidRange { fn_name, .. } => fn_name.as_str(),
                ArgumentError::InvalidType { fn_name, .. } => fn_name.as_str(),
            }),
            reason: value.to_string(),
        }
    }
}

impl ExecutionError {
    pub fn new(call_info: CallInfo, reason: &str) -> Self {
        Self {
            reason: reason.to_string(),
            call_info,
        }
    }
}

pub type BuiltinFn = fn(
    Option<SharedDataType>,
    Vec<SharedDataType>,
    &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError>;
pub type Executable = Arc<
    dyn Fn(
        Option<SharedDataType>,
        Vec<SharedDataType>,
        &mut RuntimeContext,
    ) -> Result<SharedDataType, ExecutionError>,
>;
