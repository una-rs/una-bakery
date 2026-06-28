use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum ParserError {
    InvalidSourceLine {
        source: &'static str,
        line: usize,
    },
    InvalidPropertyValue {
        source: &'static str,
        line: usize,
        property: &'static str,
        value: String,
    },
    KeyAlreadyExists {
        source: &'static str,
        line: usize,
        key: String,
    },
    CodepointBlockNotFound {
        source: &'static str,
        line: usize,
    },
}

pub type UnknownPropertyValue = String;

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ParserError::InvalidSourceLine { source, line } => {
                write!(f, "invalid source line: {}:{}", source, line)
            }
            ParserError::InvalidPropertyValue {
                source,
                line,
                property,
                value,
            } => {
                write!(
                    f,
                    "invalid property value: {}:{}, property: {}, value: {}",
                    source, line, property, value
                )
            }
            ParserError::KeyAlreadyExists { source, line, key } => {
                write!(f, "key already exists: {}:{}, key: {}", source, line, key)
            }
            ParserError::CodepointBlockNotFound { source, line } => {
                write!(f, "code point block not found: {}:{}", source, line)
            }
        }
    }
}

impl std::error::Error for ParserError {}
