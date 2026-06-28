use crate::{
    codepoint::Codepoint,
    normalization::{Classifier, DecompositionCase},
};

macro_rules! checks {
    ($($expr: expr),+) => {
        if !($($expr &&)+ true) {
            return None;
        }
    };
}

/// Classifies a nonstarter code point that has no decomposition.
pub fn nonstarter(cd: &Classifier, cp: &Codepoint) -> Option<DecompositionCase> {
    checks!(
        cp.is_nonstarter(),                        // Not a starter.
        cd.decompositions.get(cp.code).is_empty()  // No decomposition.
    );

    // Explicit check for empty decomposition: these 4 code points have no decomposition in either canonical or compatibility form:
    //
    //   - U+0340 COMBINING GRAVE TONE MARK
    //   - U+0341 COMBINING ACUTE TONE MARK
    //   - U+0343 COMBINING GREEK KORONIS
    //   - U+0344 COMBINING GREEK DIALYTIKA TONOS

    Some(match cd.compositions.backwards(cp.code) {
        None => DecompositionCase::Nonstarter { ccc: cp.ccc },
        Some(combines_backwards) => DecompositionCase::NonstarterCombinesBackwards {
            ccc: cp.ccc,
            combines_backwards: combines_backwards.clone(),
        },
    })
}
