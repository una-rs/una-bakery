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

/// Classifies a singleton decomposition: a starter that decomposes into another starter.
pub fn singleton(cd: &Classifier, cp: &Codepoint) -> Option<DecompositionCase> {
    let dec = cd.decompositions.get(cp.code).as_codepoints(cd.unicode);

    checks!(
        cp.is_starter(),     // Code point is a starter.
        dec.len() == 1,      // Decomposition is a single code point.
        dec[0].is_starter()  // Decomposed code point is a starter.
    );

    let dec = dec[0].clone();

    // Neither the original nor the decomposed code point combines with the preceding code point.

    assert!(cd.compositions.backwards(cp.code).is_none());
    assert!(cd.compositions.backwards(dec.code).is_none());

    // Singleton code points are always decomposed; they never appear in normalized output.

    assert_eq!(cd.qc_map.get(cp.code), 'N');

    // If the original code point can combine with a following code point, this occurs only
    // when the decomposition is a compatibility decomposition.
    // In canonical form, this code point is a regular starter.
    // This means that this property should simply be ignored.

    if cd.compositions.forwards(cp.code).is_some() {
        assert!(cp.decomposition_tag.is_some());
    }

    // No composition exclusions (although this check is not actually required here).
    // I keep it as a safeguard in case something changes in future versions.

    assert!(!cd.exclusions.contains(&cp.code));
    assert!(!cd.exclusions.contains(&dec.code));

    // There are two cases that can be encoded differently for optimization purposes:
    //
    // 1. The decomposition code point is a regular starter that cannot combine with following code points.
    // 2. The decomposition code point is a starter that can combine with subsequent code points.
    //
    // Only three cases fall into the second category in NFC, whereas in NFKC it is almost half of them.

    let combines_forwards = cd.compositions.forwards(dec.code);
    let qc = cd.qc_map.get(dec.code);

    Some(match combines_forwards {
        Some(combines_forwards) => {
            assert_eq!(qc, 'Y');

            DecompositionCase::SingletonStarterCombinesForwards {
                decomposition: dec,
                combines_forwards: combines_forwards.clone(),
            }
        }
        None => {
            assert_eq!(qc, 'Y');

            DecompositionCase::Singleton {
                decomposition: dec.clone(),
            }
        }
    })
}
