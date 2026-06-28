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

/// Classifies a starter code point that decomposes into one or more nonstarters.
pub fn starters_to_nonstarters(cd: &Classifier, cp: &Codepoint) -> Option<DecompositionCase> {
    let dec = cd.decompositions.get(cp.code).as_codepoints(cd.unicode);

    checks!(
        cp.is_starter(),                       // Starter.
        !dec.is_empty(),                       // Has decomposition.
        dec.iter().all(|c| c.is_nonstarter())  // All decomposed code points are nonstarters.
    );

    // For canonical decomposition (into two nonstarters):
    //
    //   - U+0F73 — TIBETAN VOWEL SIGN II
    //   - U+0F75 — TIBETAN VOWEL SIGN UU
    //   - U+0F81 — TIBETAN VOWEL SIGN REVERSED II
    //
    // In compatibility decomposition, the following are added (decomposing into a single nonstarter):
    //
    //   - U+FF9E — HALFWIDTH KATAKANA VOICED SOUND MARK
    //   - U+FF9F — HALFWIDTH KATAKANA SEMI-VOICED SOUND MARK

    Some(DecompositionCase::StarterToNonstarters { decomposition: dec })
}
/// Nonstarter decomposing into nonstarter(s).
pub fn nonstarters_to_nonstarters(cd: &Classifier, cp: &Codepoint) -> Option<DecompositionCase> {
    let dec = cd.decompositions.get(cp.code).as_codepoints(cd.unicode);

    checks!(
        cp.is_nonstarter(),                    // Nonstarter.
        !dec.is_empty(),                       // Has decomposition.
        dec.iter().all(|c| c.is_nonstarter())  // All decomposed code points are nonstarters.
    );

    Some(DecompositionCase::NonstarterToNonstarters { decomposition: dec })
}
