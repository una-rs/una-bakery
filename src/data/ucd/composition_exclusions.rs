use std::ops::Deref;

use crate::errors::*;

const NAME: &str = "CompositionExclusions.txt";

/// CompositionExclusions.txt from the UCD.
///
/// Composition exclusions cannot be computed algorithmically; this list is manually maintained by the Unicode Consortium.
///
/// https://www.unicode.org/reports/tr15/#Primary_Exclusion_List_Table
pub struct CompositionExclusions(Vec<u32>);

impl CompositionExclusions {
    pub fn parse(source: &str) -> Result<Self, ParserError> {
        let mut exclusions = vec![];

        for (i, line) in source.lines().enumerate() {
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let (code, _) = line
                .split_once('#')
                .ok_or_else(|| ParserError::InvalidSourceLine {
                    source: NAME,
                    line: i + 1,
                })?;

            let code = u32::from_str_radix(code.trim(), 16).map_err(|_| {
                ParserError::InvalidPropertyValue {
                    source: NAME,
                    line: i + 1,
                    property: "code",
                    value: code.trim().to_string(),
                }
            })?;

            exclusions.push(code);
        }

        Ok(Self(exclusions))
    }
}

impl Deref for CompositionExclusions {
    type Target = Vec<u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
