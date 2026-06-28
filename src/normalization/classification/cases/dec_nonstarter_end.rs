use crate::{
    codepoint::Codepoint,
    normalization::{Classifier, DecompositionCase, composing::*},
};

macro_rules! checks {
    ($($expr: expr),+) => {
        if !($($expr &&)+ true) {
            return None;
        }
    };
}

/// Classifies a code point with a decomposition that starts with a starter and ends with a nonstarter.
pub fn ends_with_nonstarter(cd: &Classifier, cp: &Codepoint) -> Option<DecompositionCase> {
    let dec = cd.decompositions.get(cp.code).as_codepoints(cd.unicode);

    checks!(
        cp.is_starter(),                     // Starter.
        dec.len() >= 2,                      // Decomposition >= 2 code points.
        dec.first().unwrap().is_starter(),   // First code point is a starter.
        dec.last().unwrap().is_nonstarter()  // Last code point is a nonstarter.
    );

    // Precomposed till the last starter: `composed` + `starter` + `nonstarters`.

    let ls_idx = get_last_starter_idx(&dec);
    let mut pre = normalize_decomposed(&dec[..=ls_idx], cd.unicode, &cd.compositions);

    let mut trailing = compose_starter_with_min_ccc_nonstarters(
        pre.pop().unwrap(),
        &dec[ls_idx + 1..],
        cd.unicode,
        &cd.compositions,
    );

    pre.append(&mut trailing);

    // Neither the first decomposed code point nor the first precomposed code point combines with the preceding code point.

    assert!(cd.compositions.backwards(dec[0].code).is_none());
    assert!(cd.compositions.backwards(pre[0].code).is_none());

    // Quick checks.
    let qc = cd.qc_map.get(cp.code);

    // Recombines?
    if pre[0].code == cp.code && pre.len() == 1 {
        assert_eq!(qc, 'Y');
        return case_recombines(cd, cp, dec, pre);
    }

    Some(
        match cd
            .compositions
            .forwards(pre[get_last_starter_idx(&pre)].code)
        {
            Some(forwards) => {
                let normalized = normalize_decomposed(&dec, cd.unicode, &cd.compositions);

                assert_eq!(
                    qc,
                    match normalized[0].code == cp.code {
                        true => 'Y',
                        false => 'N',
                    }
                );

                DecompositionCase::EndsWithNonstarterCombinesForwards {
                    decomposition: dec,
                    precomposition: pre,
                    combines_forwards: forwards.clone(),
                }
            }
            None => {
                assert_eq!(qc, 'N');

                DecompositionCase::EndsWithNonstarter {
                    decomposition: dec,
                    precomposition: pre,
                }
            }
        },
    )
}

fn case_recombines(
    cd: &Classifier,
    cp: &Codepoint,
    dec: Vec<Codepoint>,
    pre: Vec<Codepoint>,
) -> Option<DecompositionCase> {
    let normalized = normalize_decomposed(&dec, cd.unicode, &cd.compositions);
    assert_eq!(normalized, pre);

    Some(match cd.compositions.forwards(cp.code) {
        Some(forwards) => {
            DecompositionCase::EndsWithNonstarterRecombinesIntoOriginalCombinesForwards {
                decomposition: dec,
                combines_forwards: forwards.clone(),
            }
        }
        None => DecompositionCase::EndsWithNonstarterRecombinesIntoOriginal { decomposition: dec },
    })
}

fn get_last_starter_idx(dec: &[Codepoint]) -> usize {
    let mut ls = dec.len() - 1;

    for i in (0..dec.len()).rev() {
        match dec[i].is_nonstarter() {
            true => ls -= 1,
            false => break,
        }
    }

    ls
}
