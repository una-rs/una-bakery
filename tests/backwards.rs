use std::collections::HashMap;

use una_bakery::data::ucd::UnicodeData;
use una_bakery::data::{UCD, ucd};
use una_bakery::normalization::{Classifier, LAST_DECOMPOSITION_CODEPOINT};
use una_bakery::normalization::{DecompositionCase, NormType};

#[test]
fn check_starters_backwards_combining() {
    let ucd = ucd().unwrap();
    let uni = &ucd.unicode;

    let (canonical_map, compat_map) = maps(&ucd);

    let (with_starters, _) = combining_backwards_lists(uni, &canonical_map);
    let (with_starters_compat, _) = combining_backwards_lists(uni, &compat_map);

    assert_eq!(with_starters, with_starters_compat);

    for code in with_starters {
        let case = canonical_map.get(&code).unwrap();

        match case {
            // Common starter cases.
            DecompositionCase::StarterCombinesForwards { .. }
            | DecompositionCase::StarterCombinesForwardsBackwards { .. } => continue,

            // Will be written as an expansion with recomposition into the original code point in the unified data tables
            // or as common starter in the composition-only tables.
            DecompositionCase::EndsWithStarterRecombinesIntoOriginalCombinesForwards { .. } => {
                continue;
            }

            DecompositionCase::EndsWithStarterCombinesBoth { .. } => {
                // QC = Maybe.
                assert_eq!(ucd.quick_checks.nfc.get(code), 'M');

                // U+16121 — M — GURUNG KHEMA VOWEL SIGN U
                // U+16122 — M — GURUNG KHEMA VOWEL SIGN UU
                assert!(code == 0x16121 || code == 0x16122);
            }

            _ => unreachable!(),
        }
    }
}

#[test]
fn check_nonstarters_backwards_combining() {
    let ucd = ucd().unwrap();
    let uni = &ucd.unicode;

    let (canonical_map, compat_map) = maps(&ucd);

    let (_, with_nonstarters) = combining_backwards_lists(uni, &canonical_map);
    let (_, with_nonstarters_compat) = combining_backwards_lists(uni, &compat_map);

    assert_eq!(with_nonstarters, with_nonstarters_compat);

    for code in with_nonstarters {
        let case = canonical_map.get(&code).unwrap();

        match case {
            // Common starter cases.
            DecompositionCase::StarterCombinesForwards { .. }
            | DecompositionCase::StarterCombinesForwardsBackwards { .. } => continue,

            // May be recomposed. Encoded as a pair with no stop flag.
            DecompositionCase::Pair { .. } => continue,

            // Recombines into the original code point. Encoded as expansion in the unified tables. Stop flag = 0.
            DecompositionCase::PairRecombinesIntoOriginalCombinesForwards { .. }
            | DecompositionCase::EndsWithStarterRecombinesIntoOriginalCombinesForwards { .. }
            | DecompositionCase::EndsWithNonstarterRecombinesIntoOriginalCombinesForwards {
                ..
            } => continue,

            _ => unreachable!(),
        }
    }
}

fn maps(
    ucd: &UCD,
) -> (
    HashMap<u32, DecompositionCase>,
    HashMap<u32, DecompositionCase>,
) {
    let canonical_map = Classifier::new(
        &ucd.unicode,
        &ucd.composition_exclusions,
        &ucd.quick_checks.nfc,
        NormType::Canonical,
    )
    .create_map()
    .unwrap();

    let compatibility_map = Classifier::new(
        &ucd.unicode,
        &ucd.composition_exclusions,
        &ucd.quick_checks.nfkc,
        NormType::Compatibility,
    )
    .create_map()
    .unwrap();

    (canonical_map, compatibility_map)
}

fn combining_backwards_lists(
    uni: &UnicodeData,
    canonical_map: &HashMap<u32, DecompositionCase>,
) -> (Vec<u32>, Vec<u32>) {
    let mut with_starters = vec![];
    let mut with_nonstarters = vec![];

    for code in 0..LAST_DECOMPOSITION_CODEPOINT {
        let cp = match canonical_map.get(&code) {
            Some(cp) => cp,
            None => continue,
        };

        macro_rules! push_starters {
            ($from: ident, $target: ident) => {
                $from
                    .keys()
                    .copied()
                    .collect::<Vec<u32>>()
                    .iter()
                    .for_each(|&c| match uni.get(&c) {
                        Some(cp) => {
                            if cp.is_starter() {
                                $target.push(cp.code)
                            } else {
                                panic!("not a starter")
                            }
                        }
                        None => $target.push(c),
                    })
            };
        }

        match cp {
            una_bakery::normalization::DecompositionCase::StarterCombinesBackwards {
                combines_backwards,
            }
            | una_bakery::normalization::DecompositionCase::StarterCombinesForwardsBackwards {
                combines_backwards,
                ..
            }
            | una_bakery::normalization::DecompositionCase::EndsWithStarterCombinesBackwards {
                combines_backwards,
                ..
            }
            | una_bakery::normalization::DecompositionCase::EndsWithStarterCombinesBoth {
                combines_backwards,
                ..
            } => push_starters!(combines_backwards, with_starters),
            una_bakery::normalization::DecompositionCase::NonstarterCombinesBackwards {
                combines_backwards,
                ..
            } => push_starters!(combines_backwards, with_nonstarters),
            _ => continue,
        };
    }

    with_starters.sort_unstable();
    with_starters.dedup();

    with_nonstarters.sort_unstable();
    with_nonstarters.dedup();

    (with_starters, with_nonstarters)
}
