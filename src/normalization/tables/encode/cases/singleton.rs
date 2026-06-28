use std::collections::HashMap;

use crate::codepoint::Codepoint;
use crate::normalization::*;

/// Singleton: starter → starter.
pub fn encode_singleton(decomposes_into: &Codepoint) -> Encoded {
    let mut value = Encoded::bits(Marker::Singleton, StopFlag::Enabled);

    value |= decomposes_into.code << 8;

    Encoded {
        value,
        expansion: None,
    }
}

/// Singleton: starter → starter.
/// Decomposition code point may be (or may be not) combined with the subsequent code point.
/// Compositions info is not stored here, it is assumed that we will need perform an additional table lookup.
pub fn encode_singleton_in_the_composition_table(
    decomposes_into: &Codepoint,
    compositions: &BakedCompositions,
    cases: &HashMap<u32, DecompositionCase>,
) -> Encoded {
    assert!(composition_data_stored(
        decomposes_into.code,
        compositions,
        cases
    ));

    encode_singleton(decomposes_into)
}

/// Check if code point in its encoded form will contain a baked composition info (if needed).
fn composition_data_stored(
    cp: u32,
    compositions: &BakedCompositions,
    cases: &HashMap<u32, DecompositionCase>,
) -> bool {
    let case = cases.get(&cp).unwrap();

    match compositions.index.get(&cp) {
        Some(_) => match case {
            DecompositionCase::StarterCombinesForwards { .. }
            | DecompositionCase::StarterCombinesForwardsBackwards { .. }
            | DecompositionCase::PairRecombinesIntoOriginalCombinesForwards { .. }
            | DecompositionCase::EndsWithStarterRecombinesIntoOriginalCombinesForwards { .. }
            | DecompositionCase::EndsWithNonstarterRecombinesIntoOriginalCombinesForwards {
                ..
            } => true,
            _ => false,
        },
        None => match case {
            DecompositionCase::PairRecombinesIntoOriginal { .. }
            | DecompositionCase::EndsWithNonstarterRecombinesIntoOriginal { .. }
            | DecompositionCase::HangulSyllable => true,
            _ => false,
        },
    }
}
