use crate::{
    codepoint::Codepoint,
    normalization::{
        Classifier, DecompositionCase, composing::compose_starter_with_min_ccc_nonstarters,
    },
};

macro_rules! checks {
    ($($expr: expr),+) => {
        if !($($expr &&)+ true) {
            return None;
        }
    };
}

/// Decomposition: → starter → pair.
pub fn pair_via_singleton(cd: &Classifier, cp: &Codepoint) -> Option<DecompositionCase> {
    let dec = cd.decompositions.get(cp.code).as_codepoints(cd.unicode);

    checks!(
        cp.is_starter(),             // Code point is a starter.
        cp.decomposition.len() == 1, // Original decomposition is a single code point.
        dec.len() == 2               // Full decomposition = 2 code points.
    );

    let c0 = &dec[0];
    let c1 = &dec[1];

    checks!(
        c0.is_starter(),    // First decomposed code point is a starter.
        c1.is_nonstarter()  // Second is a nonstarter.
    );

    let singleton = cd.unicode[cp.decomposition[0]].clone();

    // Decomposition verification.
    assert_eq!(c0.code, singleton.decomposition[0]);
    assert_eq!(c1.code, singleton.decomposition[1]);

    // Neither the original nor the first decomposed code point (starter) combines with the preceding code point.
    assert!(cd.compositions.backwards(cp.code).is_none());
    assert!(cd.compositions.backwards(c0.code).is_none());

    // Cannot be recombined into the original code point.
    assert_eq!(cd.qc_map.get(cp.code), 'N');

    // Singleton recomposition check.
    if let Some(result) = case_singleton_recomposition(cd, cp, &dec) {
        return Some(result);
    };

    // Common case.

    let forwards = cd.compositions.forwards(c0.code);

    Some(match forwards {
        Some(forwards) => DecompositionCase::PairNoRecompositionCombinesForwards {
            decomposition: dec,
            starter_combines_forwards: forwards.clone(),
        },
        None => DecompositionCase::PairNoRecomposition { decomposition: dec },
    })
}

/// Decomposition: → singleton + nonstarter → starter + nonstarter.
pub fn pair_singleton_starter(cd: &Classifier, cp: &Codepoint) -> Option<DecompositionCase> {
    let dec = cd.decompositions.get(cp.code).as_codepoints(cd.unicode);

    checks!(
        cp.is_starter(),             // Code point is a starter.
        cp.decomposition.len() == 2, // Decomposes into 2 code points.
        dec.len() == 2               // Full decomposition = 2 code points.
    );

    let oc0 = cd.unicode[cp.decomposition[0]].clone();
    let oc1 = cd.unicode[cp.decomposition[1]].clone();

    let c0 = &dec[0];
    let c1 = &dec[1];

    checks!(
        oc0.is_starter(),    // First decomposition code point is a starter.
        oc1.is_nonstarter(), // Second is a nonstarter.
        oc0.code != c0.code, // Singleton decomposition.
        oc1.code == c1.code  // Nonstarter stays the same.
    );

    // Neither original nor the first decomposed code point (starter) does not combine with the preceding code point.
    assert!(cd.compositions.backwards(cp.code).is_none());
    assert!(cd.compositions.backwards(c0.code).is_none());

    // Cannot be recombined into the original code point.
    assert_eq!(cd.qc_map.get(cp.code), 'N');

    // Singleton recomposition check.
    if let Some(result) = case_singleton_recomposition(cd, cp, &dec) {
        return Some(result);
    };

    // Starter may combine with the subsequent code points.
    let forwards = cd.compositions.forwards(c0.code);
    assert!(forwards.is_some());

    // Common case.
    Some(DecompositionCase::PairNoRecompositionCombinesForwards {
        decomposition: dec,
        starter_combines_forwards: forwards.unwrap().clone(),
    })
}

/// Decomposition into a starter + nonstarter pair.
/// No size-based filtering for baking is applied at this stage.
pub fn pair(cd: &Classifier, cp: &Codepoint) -> Option<DecompositionCase> {
    let dec = cd.decompositions.get(cp.code).as_codepoints(cd.unicode);

    checks!(
        cp.is_starter(),             // Code point is a starter.
        dec.len() == 2,              // Decomposes into a pair.
        cp.decomposition.len() == 2  // Originally decomposes into a pair.
    );

    let c0 = &dec[0];
    let c1 = &dec[1];

    checks!(
        c0.is_starter(),    // First decomposed code point is a starter.
        c1.is_nonstarter()  // Second is a nonstarter.
    );

    assert_eq!(c0.code, cp.decomposition[0]);
    assert_eq!(c1.code, cp.decomposition[1]);

    // Neither original nor the first decomposed code point (starter) does not combine with the preceding code point.
    assert!(cd.compositions.backwards(cp.code).is_none());
    assert!(cd.compositions.backwards(c0.code).is_none());

    let starter_combines_forwards = match cd.compositions.forwards(c0.code) {
        Some(forwards) => Some(forwards.clone()),
        None => None,
    };

    // Composition exclusions.
    if cd.exclusions.contains(&cp.code) {
        assert_eq!(cd.qc_map.get(cp.code), 'N');
        assert!(starter_combines_forwards.is_none());

        return Some(DecompositionCase::PairNoRecomposition { decomposition: dec });
    }

    // Compatibility form.
    if starter_combines_forwards.is_none() {
        assert!(cd.norm_type.is_compatibility());
        assert_eq!(cd.qc_map.get(cp.code), 'N');

        return Some(DecompositionCase::PairNoRecomposition { decomposition: dec });
    }

    let starter_combines_forwards = starter_combines_forwards.unwrap();

    // No recomposition found: compatibility decomposition case.
    if !starter_combines_forwards.iter().any(|(_, &v)| v == cp.code) {
        assert!(cd.norm_type.is_compatibility());
        assert_eq!(cd.qc_map.get(cp.code), 'N');

        return Some(DecompositionCase::PairNoRecompositionCombinesForwards {
            decomposition: dec,
            starter_combines_forwards,
        });
    };

    assert_eq!(cd.qc_map.get(cp.code), 'Y');

    // Always recombined vs sometimes recombined?

    let pre = compose_starter_with_min_ccc_nonstarters(
        dec[0].clone(),
        &dec[1..],
        cd.unicode,
        &cd.compositions,
    );

    // Always recombines into the original code point.
    if pre.len() == 1 {
        assert_eq!(pre[0].code, cp.code);

        return Some(match cd.compositions.forwards(cp.code) {
            Some(forwards) => DecompositionCase::PairRecombinesIntoOriginalCombinesForwards {
                decomposition: dec,
                combines_forwards: forwards.clone(),
            },
            None => DecompositionCase::PairRecombinesIntoOriginal { decomposition: dec },
        });
    }

    // Common case.

    Some(DecompositionCase::Pair {
        decomposition: dec,
        starter_combines_forwards: starter_combines_forwards,
    })
}

/// Acts like a singleton in the composing form normalization.
fn case_singleton_recomposition(
    cd: &Classifier,
    cp: &Codepoint,
    dec: &[Codepoint],
) -> Option<DecompositionCase> {
    // Recombines as a singleton?
    let recombines = compose_starter_with_min_ccc_nonstarters(
        dec[0].clone(),
        &dec[1..],
        cd.unicode,
        &cd.compositions,
    );

    if recombines.len() != 1 {
        return None;
    }

    assert_ne!(recombines[0].code, cp.code);

    let forwards = cd.compositions.forwards(recombines[0].code);

    Some(match forwards {
        Some(forwards) => DecompositionCase::PairAsSingletonCombinesForwards {
            decomposition: dec.to_owned(),
            singleton: recombines[0].clone(),
            singleton_combines_forwards: forwards.clone(),
        },
        None => DecompositionCase::PairAsSingleton {
            decomposition: dec.to_owned(),
            singleton: recombines[0].clone(),
        },
    })
}
