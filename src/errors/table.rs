use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum TableError {
    OutOfRange { codepoint: u32 },
    PagesOutOfRange { count: u32 },
    UnknownError,
    StatsError { reason: String },
}

impl Display for TableError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            TableError::OutOfRange { codepoint } => {
                write!(
                    f,
                    "failed to fit the data with u16 indexes on U+{:0X}",
                    *codepoint
                )
            }
            TableError::PagesOutOfRange { count } => {
                write!(f, "index page count exceeds the limit: {:0X}", *count)
            }
            TableError::UnknownError => {
                write!(f, "unknown table baking error")
            }
            TableError::StatsError { reason } => {
                write!(f, "stats error: {}", reason)
            }
        }
    }
}
