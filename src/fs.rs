use std::{fs::canonicalize, path::PathBuf};

use crate::interpreter::builtin::{CallInfo, ExecutionError};

pub fn get_absolute_path(directory: &str) -> Result<String, ExecutionError> {
    let expanded = if directory.starts_with("~/") {
        let home = std::env::var("HOME").map_err(|err| {
            ExecutionError::new(
                CallInfo::new(""),
                &format!("Could not resolve home directory: {}", err),
            )
        })?;
        format!("{}/{}", home, &directory[2..])
    } else {
        directory.to_string()
    };

    canonicalize(PathBuf::from(&expanded))
        .map_err(|err| {
            ExecutionError::new(
                CallInfo::new(""),
                &format!("Directory '{}' could not be resolved ({})", directory, err),
            )
        })?
        .into_os_string()
        .into_string()
        .map_err(|err| {
            ExecutionError::new(
                CallInfo::new(""),
                &format!(
                    "Directory '{}' could not be resolved ({:?})",
                    directory, err
                ),
            )
        })
}
