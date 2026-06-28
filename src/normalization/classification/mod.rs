use std::collections::HashMap;

use crate::data::ucd::*;
use crate::errors::ClassifierError;
use crate::normalization::composition::Compositions;
use crate::normalization::{Decompositions, NormType};

mod case;
mod cases;
mod meta;

pub use case::*;
use cases::*;

pub use meta::*;

/// Classifier for Unicode code points used for baking normalization data.
pub struct Classifier<'a> {
    unicode: &'a UnicodeData,
    decompositions: Decompositions,
    compositions: Compositions,
    exclusions: &'a CompositionExclusions,
    qc_map: &'a QCMap,
    norm_type: NormType,
}

impl<'a> Classifier<'a> {
    pub fn new(
        unicode: &'a UnicodeData,
        exclusions: &'a CompositionExclusions,
        qc_map: &'a QCMap,
        norm_type: NormType,
    ) -> Self {
        Self {
            unicode,
            decompositions: Decompositions::generate(unicode, norm_type),
            compositions: Compositions::generate(unicode, exclusions),
            exclusions,
            qc_map,
            norm_type,
        }
    }

    /// Creates a map of Unicode code point normalization classifications.
    pub fn create_map(&self) -> Result<HashMap<u32, DecompositionCase>, ClassifierError> {
        let mut map: HashMap<u32, DecompositionCase> = HashMap::new();

        let mut keys: Vec<&u32> = self.unicode.keys().collect();
        keys.sort();

        for &code in keys {
            match self.classify(code) {
                Some(case) => map.insert(code, case),
                None => {
                    println!("not classified: {:04X}", code);
                    return Err(ClassifierError::CodepointNotClassified { codepoint: code });
                }
            };
        }

        Ok(map)
    }

    /// Классификация кодпоинта из таблицы UCD для алгоритма нормализации.
    fn classify(&self, code: u32) -> Option<DecompositionCase> {
        let codepoint = match self.unicode.get(&code) {
            Some(codepoint) => codepoint,
            None => return Some(DecompositionCase::StarterIgnored),
        };

        [
            // Hangul code points.
            hangul,
            hangul_compatibility_jamo,
            // Starters.
            starter,
            // Nonstarters.
            nonstarter,
            // Singletons.
            singleton,
            // Pairs.
            pair_via_singleton,
            pair_singleton_starter,
            pair,
            // Decopositions into nonstarters.
            starters_to_nonstarters,
            nonstarters_to_nonstarters,
            // Starter → Starter + ... + starter.
            edge_starters,
            // Starter → Starter + ... + nonstarter(s).
            ends_with_nonstarter,
        ]
        .iter()
        .find_map(|f| f(self, codepoint))
    }
}
