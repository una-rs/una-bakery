use std::collections::HashMap;
use std::ops::{Deref, Index};

use crate::codepoint::*;
use crate::data::ucd::blocks::Blocks;
use crate::errors::*;

const PRIVATE_USE_START: u32 = 0xF0000;
const CODEPOINT_COLUMNS_COUNT: usize = 15;
const NAME: &str = "UnicodeData.txt";

pub struct UnicodeData(HashMap<u32, Codepoint>);

impl UnicodeData {
    pub fn parse(source: &str, blocks: &Blocks) -> Result<Self, ParserError> {
        let mut map: HashMap<u32, Codepoint> = HashMap::new();
        let mut range_start: Option<Codepoint> = None;

        for (i, line) in source.lines().enumerate() {
            macro_rules! err {
                ($i:expr) => {
                    ParserError::InvalidSourceLine {
                        source: NAME,
                        line: $i + 1,
                    }
                };
            }

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let props: Vec<&str> = line.split(';').collect();
            if props.len() != CODEPOINT_COLUMNS_COUNT {
                return Err(err!(i));
            }

            let codepoint = parse_codepoint(props, i, blocks)?;

            if !codepoint.name.starts_with('<') || codepoint.name == "<control>" {
                map.insert(codepoint.code, codepoint);
                continue;
            }

            if codepoint.code >= PRIVATE_USE_START
                || codepoint.name.contains("Private Use")
                || codepoint.name.contains("Surrogate")
            {
                continue;
            }

            // CJK, Hangul, Tangut ranges.

            if !codepoint.name.starts_with("<CJK")
                && !codepoint.name.starts_with("<Tangut")
                && !codepoint.name.starts_with("<Hangul")
            {
                return Err(err!(i));
            }

            if codepoint.name.ends_with("First>") {
                range_start = Some(codepoint);
                continue;
            }

            if !codepoint.name.ends_with("Last>") {
                return Err(err!(i));
            }

            let group_codepoint = range_start.ok_or_else(|| err!(i))?;

            // In our case, code point names are not important.
            // If needed, they can be obtained from UCD: extracted/DerivedName.txt

            let trimmed_group_name = &group_codepoint.name[1..group_codepoint.name.len() - 8];

            for code in group_codepoint.code..=codepoint.code {
                let mut codepoint = group_codepoint.clone();

                codepoint.code = code;
                codepoint.name = format!("{} - {:X}", trimmed_group_name, code);

                map.insert(code, codepoint);
            }

            range_start = None;
        }

        Ok(Self(map))
    }
}

impl Deref for UnicodeData {
    type Target = HashMap<u32, Codepoint>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Index<u32> for UnicodeData {
    type Output = Codepoint;

    fn index(&self, key: u32) -> &Self::Output {
        &self.0[&key]
    }
}

fn parse_codepoint(props: Vec<&str>, i: usize, blocks: &Blocks) -> Result<Codepoint, ParserError> {
    macro_rules! err {
        ($prop:expr, $val:expr) => {
            ParserError::InvalidPropertyValue {
                source: NAME,
                line: i + 1,
                property: $prop,
                value: $val.to_string(),
            }
        };
    }

    // Code and name.
    let code = u32::from_str_radix(props[0], 16).map_err(|e| err!("code", e))?;

    let name = props[1].to_owned();

    // General category and canonical combining class (CCC).
    let gc = GeneralCategory::try_from(props[2]).map_err(|e| err!("gc", e))?;

    let ccc = CanonicalCombiningClass::try_from(props[3]).map_err(|e| err!("ccc", e))?;

    // Bidi class and Bidi Mirrored.
    let bc = BidiClass::try_from(props[4]).map_err(|e| err!("bidi_class", e))?;

    let bidi_mirrored = BidiMirrored::try_from(props[9]).map_err(|e| err!("bidi_mirrored", e))?;

    // Decomposition mapping and decomposition tag.
    let decomposition = Decomposition::try_from(props[5]).map_err(|e| err!("decomposition", e))?;

    // Various numeric values.
    let numeric =
        NumericType::try_from((props[6], props[7], props[8])).map_err(|e| err!("numeric", e))?;

    // Simple case mappings (if present).
    let simple_uppercase_mapping =
        SimpleCaseMapping::try_from(props[12]).map_err(|e| err!("simple_uppercase_mapping", e))?;

    let simple_lowercase_mapping =
        SimpleCaseMapping::try_from(props[13]).map_err(|e| err!("simple_lowercase_mapping", e))?;

    let simple_titlecase_mapping =
        SimpleCaseMapping::try_from(props[14]).map_err(|e| err!("simple_titlecase_mapping", e))?;

    // Skip columns 10 and 11:
    //
    // * Unicode_1_Name (Obsolete as of Unicode 6.2.0)
    // * ISO_Comment (Obsolete as of Unicode 5.2.0; Deprecated and Stabilized as of 6.0.0)

    Ok(Codepoint {
        code,
        name: name.clone(),
        gc,
        ccc,
        bc,
        numeric,
        bidi_mirrored,
        simple_uppercase_mapping,
        simple_lowercase_mapping,
        simple_titlecase_mapping,
        decomposition_tag: decomposition.tag,
        decomposition: decomposition.codes,
        block: blocks.get_by_codepoint(code),
    })
}
