use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum ClassifierError {
    CodepointNotClassified { codepoint: u32 },
}

impl Display for ClassifierError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ClassifierError::CodepointNotClassified { codepoint } => {
                write!(f, "codepoint not classifed: U+{:X}", *codepoint)
            }
        }
    }
}
