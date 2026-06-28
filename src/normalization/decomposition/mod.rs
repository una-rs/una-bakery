use std::collections::HashMap;
use std::ops::{Deref, Index};

use crate::codepoint::Codepoint;
use crate::data::ucd::UnicodeData;
use crate::normalization::NormType;

pub struct Decompositions(HashMap<u32, Decomposition>);

#[derive(Clone)]
pub struct Decomposition(pub Vec<u32>);

impl Decompositions {
    /// Generates a hashmap of full decompositions for code points.
    /// It's important to note that this is relevant even for NFD, as a decomposition from `UnicodeData.txt` may itself be further decomposable.
    pub fn generate(unicode: &UnicodeData, norm_type: NormType) -> Self {
        let mut map: HashMap<u32, Decomposition> = HashMap::new();

        for codepoint in unicode.values() {
            if codepoint.decomposition.is_empty()
                || (norm_type.is_canonical() && codepoint.decomposition_tag.is_some())
            {
                continue;
            }

            let decomposition: Vec<u32> = decompose(codepoint.code, norm_type, unicode);
            map.insert(codepoint.code, Decomposition(decomposition));
        }

        Self(map)
    }

    pub fn get(&self, code: u32) -> Decomposition {
        match self.0.get(&code) {
            Some(decomp) => decomp.clone(),
            None => Decomposition(vec![]),
        }
    }
}

impl Decomposition {
    pub fn as_codepoints<'a>(&self, unicode: &UnicodeData) -> Vec<Codepoint> {
        let mut result: Vec<Codepoint> = vec![];

        for entry in self.iter() {
            result.push(unicode[*entry].clone());
        }

        result
    }
}

impl Deref for Decomposition {
    type Target = [u32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Index<usize> for Decomposition {
    type Output = u32;

    fn index(&self, key: usize) -> &Self::Output {
        &self.0[key]
    }
}

/// Full decomposition of a code point.
pub fn decompose(code: u32, norm_type: NormType, unicode: &UnicodeData) -> Vec<u32> {
    let mut result: Vec<u32> = vec![];

    // Keep in mind that if a code point is not listed in `UnicodeData.txt`, it will never appear in the decomposition of other code points.

    let codepoint = match unicode.get(&code) {
        Some(codepoint) => codepoint,
        None => return result,
    };

    if codepoint.decomposition.is_empty()
        || (norm_type.is_canonical() && codepoint.decomposition_tag.is_some())
    {
        return result;
    }

    for &dec_code in codepoint.decomposition.iter() {
        let decomposition = decompose(dec_code, norm_type, unicode);

        match decomposition.is_empty() {
            true => result.push(dec_code),
            false => result.extend(decomposition),
        }
    }

    result
}
