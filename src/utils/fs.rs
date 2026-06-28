use std::fs::{File, create_dir_all};
use std::path::Path;

use crate::errors::OutputError;

pub fn create_file(path_str: &str) -> Result<File, OutputError> {
    let path = Path::new(path_str.trim());

    if path.as_os_str().to_string_lossy().ends_with('/') || path.file_name().is_none() {
        return Err(OutputError::IoError {
            reason: "path is empty or appears to be a directory, not a file".into(),
            path: path_str.into(),
        });
    }

    if let Some(parent) = path.parent() {
        create_dir_all(parent).map_err(|e| OutputError::IoError {
            reason: format!("failed to create the directory ({})", e).into(),
            path: path_str.into(),
        })?;
    }

    File::create(path).map_err(|e| OutputError::IoError {
        reason: format!("failed to open the file ({})", e).into(),
        path: path_str.into(),
    })
}
