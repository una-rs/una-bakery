use std::collections::HashMap;

use crate::data::ucd::CompositionExclusions;
use crate::data::ucd::UnicodeData;

/// Maps used for NFC/NFKC composition tables: whether a code point can combine with a following and/or preceding code point.
/// Data is derived from canonical decomposition pairs and the Composition Exclusion Table.
pub struct Compositions {
    /// Key: the *second* code point in a composable pair.  
    /// Value: a map where the key is the *first* code point, and the value is the composed result.
    forwards: CompositionsMap,
    /// Key: the *first* code point in a composable pair.  
    /// Value: a map where the key is the *second* code point, and the value is the composed result.
    backwards: CompositionsMap,
}

impl Compositions {
    /// Generates hash maps for composition lookups.
    pub fn generate(unicode: &UnicodeData, exclusions: &CompositionExclusions) -> Self {
        let mut forwards: HashMap<u32, HashMap<u32, u32>> = HashMap::new();
        let mut backwards: HashMap<u32, HashMap<u32, u32>> = HashMap::new();

        for codepoint in unicode.values() {
            // Only consider decompositions of length 2, as composition proceeds pairwise.
            if codepoint.decomposition.len() != 2 {
                continue;
            }

            // Skip compatibility decompositions: NFC/NFKC use *canonical* composition after decomposition.
            // See https://unicode.org/reports/tr15/#Norm_Forms
            if codepoint.decomposition_tag.is_some() {
                continue;
            }

            // Exclude characters listed in the Composition Exclusion Table — they must not be composed.
            if exclusions.contains(&codepoint.code) {
                continue;
            }

            let c0 = codepoint.decomposition[0];
            let c1 = codepoint.decomposition[1];

            // First codepoint must me a starter.
            if unicode[c0].is_nonstarter() {
                // One of these: U+0F71 TIBETAN VOWEL SIGN AA, U+0308 COMBINING DIAERESIS.
                continue;
            }

            forwards
                .entry(c0)
                .or_insert(HashMap::new())
                .insert(c1, codepoint.code);

            backwards
                .entry(c1)
                .or_insert(HashMap::new())
                .insert(c0, codepoint.code);
        }

        Self {
            forwards: CompositionsMap(forwards),
            backwards: CompositionsMap(backwards),
        }
    }

    pub fn forwards(&self, code: u32) -> Option<&HashMap<u32, u32>> {
        self.forwards.get(code)
    }

    pub fn backwards(&self, code: u32) -> Option<&HashMap<u32, u32>> {
        self.backwards.get(code)
    }

    pub fn forwards_list(&self) -> Vec<u32> {
        self.forwards.0.keys().copied().collect()
    }

    pub fn backwards_list(&self) -> Vec<u32> {
        self.backwards.0.keys().copied().collect()
    }
}

pub struct CompositionsMap(HashMap<u32, HashMap<u32, u32>>);

impl CompositionsMap {
    pub fn get(&self, code: u32) -> Option<&HashMap<u32, u32>> {
        self.0.get(&code)
    }
}
