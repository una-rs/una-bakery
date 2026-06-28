use std::collections::HashMap;

use crate::codepoint::CodepointsBlock;
use crate::errors::*;

const NAME: &str = "Blocks.txt";

/// Blocks.txt from the UCD.
///
/// The list of block names for ranges of code points.
///
/// https://www.unicode.org/reports/tr44/#Blocks.txt
pub struct Blocks(HashMap<String, CodepointsBlock>);

impl Blocks {
    pub fn parse(source: &str) -> Result<Self, ParserError> {
        let mut map = HashMap::new();

        for (i, line) in source.lines().enumerate() {
            macro_rules! err {
                (InvalidSourceLine) => {
                    ParserError::InvalidSourceLine {
                        source: NAME,
                        line: i + 1,
                    }
                };
                (InvalidPropertyValue, $prop:expr, $val:expr) => {
                    ParserError::InvalidPropertyValue {
                        source: NAME,
                        line: i + 1,
                        property: $prop,
                        value: $val.to_string(),
                    }
                };
                (KeyAlreadyExists, $key:expr) => {
                    ParserError::KeyAlreadyExists {
                        source: NAME,
                        line: i + 1,
                        key: $key,
                    }
                };
            }

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let (range, name) = line
                .split_once(';')
                .ok_or_else(|| err!(InvalidSourceLine))?;

            let (from, to) = range
                .split_once("..")
                .ok_or_else(|| err!(InvalidSourceLine))?;

            // When comparing block names, casing, whitespace, hyphens, and underbars are ignored.
            let key: String = name
                .chars()
                .filter(|&c| !c.is_whitespace() && c != '_' && c != '-')
                .flat_map(|c| c.to_lowercase())
                .collect();

            let name = name.trim();

            let from = u32::from_str_radix(from, 16)
                .map_err(|_| err!(InvalidPropertyValue, "from", from))?;
            let to =
                u32::from_str_radix(to, 16).map_err(|_| err!(InvalidPropertyValue, "to", to))?;

            if map.contains_key(&key) {
                return Err(err!(KeyAlreadyExists, key));
            }

            map.insert(
                key,
                CodepointsBlock {
                    name: name.to_owned(),
                    from,
                    to,
                },
            );
        }

        Ok(Self(map))
    }

    pub fn get_by_codepoint(&self, code: u32) -> Option<CodepointsBlock> {
        self.0
            .values()
            .into_iter()
            .find(|block| block.range().contains(&code))
            .map(|v| v.clone())
    }
}
