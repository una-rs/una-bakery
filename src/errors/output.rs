use std::fmt::{Display, Error, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum OutputError {
    IoError { reason: String, path: String },
    Error { reason: String },
}

impl Display for OutputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            OutputError::IoError { reason, path } => {
                write!(f, "output error: {}: {}", reason, path)
            }
            OutputError::Error { reason } => write!(f, "output error: {}", reason),
        }
    }
}

impl From<Error> for OutputError {
    fn from(value: Error) -> Self {
        Self::Error {
            reason: value.to_string(),
        }
    }
}
