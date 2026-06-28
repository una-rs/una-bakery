use std::collections::HashMap;

use crate::{
    codepoint::Codepoint,
    normalization::{Classifier, DecompositionCase, composing::normalize_decomposed},
};

macro_rules! checks {
    ($($expr: expr),+) => {
        if !($($expr &&)+ true) {
            return None;
        }
    };
}

/// Classifies a code point with a decomposition that starts and ends with a starter.
pub fn edge_starters(cd: &Classifier, cp: &Codepoint) -> Option<DecompositionCase> {
    let dec = cd.decompositions.get(cp.code).as_codepoints(cd.unicode);

    checks!(
        cp.is_starter(), // Starter.
        dec.len() >= 2   // Decomposition >= 2 code points.
    );

    let first = dec.first().unwrap();
    let last = dec.last().unwrap();

    checks!(
        first.is_starter(), // Starting with a starter.
        last.is_starter()   // Ending with a starter.
    );

    // Quick checks.
    let qc = cd.qc_map.get(cp.code);

    if let Some(backwards) = cd.compositions.backwards(first.code) {
        assert_eq!(qc, 'M');
        return edge_starters_backwards(cd, cp, dec, backwards.clone());
    }

    let pre = normalize_decomposed(&dec, &cd.unicode, &cd.compositions);

    assert!(cd.compositions.backwards(pre[0].code).is_none());

    if pre.len() == 1 && pre[0].code == cp.code {
        assert_eq!(qc, 'Y');

        return Some(match cd.compositions.forwards(cp.code) {
            Some(forwards) => {
                DecompositionCase::EndsWithStarterRecombinesIntoOriginalCombinesForwards {
                    decomposition: dec,
                    combines_forwards: forwards.clone(),
                }
            }
            None => DecompositionCase::EndsWithStarterRecombinesIntoOriginal { decomposition: dec },
        });
    }

    assert_eq!(qc, 'N');

    Some(match cd.compositions.forwards(pre.last().unwrap().code) {
        Some(forwards) => DecompositionCase::EndsWithStarterCombinesForwards {
            decomposition: dec,
            precomposition: pre,
            combines_forwards: forwards.clone(),
        },
        None => DecompositionCase::EndsWithStarter {
            decomposition: dec,
            precomposition: pre,
        },
    })
}

fn edge_starters_backwards(
    cd: &Classifier,
    _: &Codepoint,
    dec: Vec<Codepoint>,
    backwards: HashMap<u32, u32>,
) -> Option<DecompositionCase> {
    match cd.compositions.forwards(dec.last().unwrap().code) {
        Some(forwards) => Some(DecompositionCase::EndsWithStarterCombinesBoth {
            decomposition: dec,
            combines_backwards: backwards,
            combines_forwards: forwards.clone(),
        }),
        None => Some(DecompositionCase::EndsWithStarterCombinesBackwards {
            decomposition: dec,
            combines_backwards: backwards,
        }),
    }
}
