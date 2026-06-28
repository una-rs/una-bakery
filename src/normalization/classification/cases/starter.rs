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

/// Starter without decomposition.
pub fn starter(cd: &Classifier, cp: &Codepoint) -> Option<DecompositionCase> {
    checks!(
        cp.is_starter(),                           // Code point is a starter.
        cd.decompositions.get(cp.code).is_empty()  // No decomposition.
    );

    let combines_backwards = cd.compositions.backwards(cp.code);
    let combines_forwards = cd.compositions.forwards(cp.code);

    if combines_forwards.is_none() && combines_backwards.is_none() {
        return Some(DecompositionCase::StarterIgnored);
    }

    assert!(!cd.exclusions.contains(&cp.code));

    if let Some(fmap) = combines_forwards
        && let Some(bmap) = combines_backwards
    {
        return Some(DecompositionCase::StarterCombinesForwardsBackwards {
            combines_forwards: fmap.clone(),
            combines_backwards: bmap.clone(),
        });
    }

    if let Some(fmap) = combines_forwards {
        return Some(DecompositionCase::StarterCombinesForwards {
            combines_forwards: fmap.clone(),
        });
    }

    if let Some(bmap) = combines_backwards {
        return Some(DecompositionCase::StarterCombinesBackwards {
            combines_backwards: bmap.clone(),
        });
    }

    None
}
