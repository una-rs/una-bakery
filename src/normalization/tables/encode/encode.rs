use super::*;
use crate::data::ucd::QCMap;
use crate::normalization::DecompositionCase;

/// Encodes decomposition information for a code point into a single u32.
/// If u32 is insufficient, an additional expansion block is used.
pub fn encode(
    code: u32,
    cases_map: &HashMap<u32, DecompositionCase>,
    ccc_map: &CompressedCCCMap,
    _qc_map: &QCMap,
    compositions: &BakedCompositions,
    table_type: TableType,
    ucd: &UCD,
) -> Encoded {
    let methods = [
        handle_starters,
        handle_nonstarters,
        handle_singletons,
        handle_dec_to_nonstarters,
        handle_pairs,
        handle_hangul,
        handle_dec_trailing_starters,
        handle_dec_trailing_nonstarters,
    ];

    let case = match cases_map.get(&code) {
        Some(case) => case,
        None => {
            return Encoded {
                value: 0,
                expansion: None,
            };
        }
    };

    // Unwrap is safe here — this is essentially a match statement split across functions.
    let value = methods
        .iter()
        .find_map(|method| {
            method(
                code,
                case,
                cases_map,
                ccc_map,
                _qc_map,
                compositions,
                table_type,
                ucd,
            )
        })
        .unwrap();

    // P.S. Why: I might add post-processing for this value later.

    value
}

/// Starters.
fn handle_starters(
    code: u32,
    case: &DecompositionCase,
    _cases_map: &HashMap<u32, DecompositionCase>,
    _ccc_map: &CompressedCCCMap,
    _qc_map: &QCMap,
    compositions: &BakedCompositions,
    table_type: TableType,
    _ucd: &UCD,
) -> Option<Encoded> {
    if !case.is_starter_case() {
        return None;
    }

    // NFD/NFKD tables — encoded as ignorable.
    if table_type.is_decomposition_only() {
        return Some(encode_ignorable());
    }

    // Starters. No additional data needed.
    if matches!(case, DecompositionCase::StarterIgnored) {
        return Some(encode_ignorable());
    }

    // Starters. Combines backwards.
    if matches!(case, DecompositionCase::StarterCombinesBackwards { .. }) {
        return Some(encode_starter_combines_backwards(code));
    }

    let cp_compositions = compositions.index[&code];

    // Starters. Combines forwards.
    if matches!(case, DecompositionCase::StarterCombinesForwards { .. }) {
        return Some(encode_starter_combines_forwards(cp_compositions));
    }

    // Starters. Combines forwards & backwards.
    if matches!(
        case,
        DecompositionCase::StarterCombinesForwardsBackwards { .. }
    ) {
        return Some(encode_starter_combines_forwards_backwards(
            code,
            cp_compositions,
        ));
    }

    unreachable!()
}

/// Nonstarters — stores its CCC for reordering. No difference in encoding.
fn handle_nonstarters(
    _code: u32,
    case: &DecompositionCase,
    _cases_map: &HashMap<u32, DecompositionCase>,
    ccc_map: &CompressedCCCMap,
    _qc_map: &QCMap,
    _compositions: &BakedCompositions,
    _table_type: TableType,
    _ucd: &UCD,
) -> Option<Encoded> {
    // Non-combining nonstarters.
    if let DecompositionCase::Nonstarter { ccc } = case {
        return Some(encode_nonstarter(ccc_map.get(ccc)));
    }

    // May be combined with the preceding code point.
    if let DecompositionCase::NonstarterCombinesBackwards { ccc, .. } = case {
        return Some(encode_nonstarter(ccc_map.get(ccc)));
    }

    None
}

/// Singletons — decomposes to another starter.
fn handle_singletons(
    _code: u32,
    case: &DecompositionCase,
    cases_map: &HashMap<u32, DecompositionCase>,
    _ccc_map: &CompressedCCCMap,
    _qc_map: &QCMap,
    compositions: &BakedCompositions,
    table_type: TableType,
    _ucd: &UCD,
) -> Option<Encoded> {
    // Singleton, cannot be combined.
    if let DecompositionCase::Singleton { decomposition } = case {
        return Some(encode_singleton(decomposition));
    }

    // Singleton — the decomposition starter may combine with the subsequent code point.
    if let DecompositionCase::SingletonStarterCombinesForwards { decomposition, .. } = case {
        // ... but this is a NFD/NFKD table case.
        if table_type.is_decomposition_only() {
            return Some(encode_singleton(decomposition));
        }

        return Some(encode_singleton_in_the_composition_table(
            decomposition,
            compositions,
            cases_map,
        ));
    }

    None
}

/// Pairs.
fn handle_pairs(
    code: u32,
    case: &DecompositionCase,
    cases_map: &HashMap<u32, DecompositionCase>,
    ccc_map: &CompressedCCCMap,
    qc_map: &QCMap,
    compositions: &BakedCompositions,
    table_type: TableType,
    ucd: &UCD,
) -> Option<Encoded> {
    let (is_pair, decomposition) = case.is_pairs_case();

    if !is_pair {
        return None;
    }

    // -- Composition only.

    // Common starters.
    if table_type.is_composition_only() {
        // Pairs as an ignorable starters in composition-only table.
        if let DecompositionCase::PairRecombinesIntoOriginal { .. } = case {
            return Some(encode_ignorable());
        }

        // Pair: starter ←→ starter + nonstarter. Always combines into the original code point, combines with the subsequent code points.
        if let DecompositionCase::PairRecombinesIntoOriginalCombinesForwards { .. } = case {
            let cp_compositions = compositions.index[&code];
            return Some(encode_starter_combines_forwards(cp_compositions));
        }
    }

    // Pairs as a singletons.
    if let DecompositionCase::PairAsSingleton { singleton, .. }
    | DecompositionCase::PairAsSingletonCombinesForwards { singleton, .. } = case
        && table_type.is_composition_only()
    {
        return Some(encode_singleton_in_the_composition_table(
            singleton,
            compositions,
            cases_map,
        ));
    }

    // -- Baked as an expansions.

    // Cannot be baked as u32 value.
    if !is_bakeable_pair(decomposition) {
        return handle_pairs_as_expansions(
            code,
            case,
            cases_map,
            ccc_map,
            qc_map,
            compositions,
            table_type,
            ucd,
        );
    }

    // -- Common cases.

    let encoded = match case {
        // Pair, the decomposition may be combined into the original code point.
        DecompositionCase::Pair { decomposition, .. }
        | DecompositionCase::PairRecombinesIntoOriginal { decomposition }
        | DecompositionCase::PairRecombinesIntoOriginalCombinesForwards { decomposition, .. } => {
            encode_pair_maybe_recomposed(decomposition)
        }

        // Pair, the decomposition cannot be recombined into the original code point.
        DecompositionCase::PairNoRecomposition { decomposition }
        | DecompositionCase::PairNoRecompositionCombinesForwards { decomposition, .. }
        | DecompositionCase::PairAsSingleton { decomposition, .. }
        | DecompositionCase::PairAsSingletonCombinesForwards { decomposition, .. } => {
            encode_pair_no_recomposition(decomposition)
        }

        _ => unreachable!(),
    };

    Some(encoded)
}

/// Pairs. Cannot be baked directly as u32 value.
fn handle_pairs_as_expansions(
    code: u32,
    case: &DecompositionCase,
    cases_map: &HashMap<u32, DecompositionCase>,
    ccc_map: &CompressedCCCMap,
    _qc_map: &QCMap,
    compositions: &BakedCompositions,
    table_type: TableType,
    _ucd: &UCD,
) -> Option<Encoded> {
    let (is_pair, decomposition) = case.is_pairs_case();

    if !is_pair || is_bakeable_pair(decomposition) {
        return None;
    }

    // Decomposition-only table.

    if table_type.is_decomposition_only() {
        return Some(encode_expansion_decomposition_only(decomposition, ccc_map));
    }

    // Composition-only table.

    if let DecompositionCase::PairAsSingleton { singleton, .. } = case
        && table_type.is_composition_only()
    {
        return Some(encode_singleton_in_the_composition_table(
            singleton,
            compositions,
            cases_map,
        ));
    }

    // -- Common case.

    // Recombines into the original code point.
    if let DecompositionCase::PairRecombinesIntoOriginal { .. }
    | DecompositionCase::PairRecombinesIntoOriginalCombinesForwards { .. } = case
    {
        return Some(encode_expansion_recombines(
            decomposition,
            code,
            ccc_map,
            compositions,
        ));
    }

    // Precomposition = singleton.
    if let DecompositionCase::PairAsSingleton { singleton, .. }
    | DecompositionCase::PairAsSingletonCombinesForwards { singleton, .. } = case
    {
        return Some(encode_expansion_common_case(
            decomposition,
            &[singleton.clone()],
            ccc_map,
            compositions,
            false,
        ));
    }

    // May (or not) be recombined into the original code point.
    if let DecompositionCase::Pair { .. } = case {
        return Some(encode_expansion_common_case(
            decomposition,
            decomposition,
            ccc_map,
            compositions,
            true,
        ));
    }

    // Typical expansion.
    if let DecompositionCase::PairNoRecomposition { .. }
    | DecompositionCase::PairNoRecompositionCombinesForwards { .. } = case
    {
        return Some(encode_expansion_common_case(
            decomposition,
            decomposition,
            ccc_map,
            compositions,
            false,
        ));
    }

    unreachable!()
}

// Hangul syllables & jamo.
fn handle_hangul(
    _code: u32,
    case: &DecompositionCase,
    _cases_map: &HashMap<u32, DecompositionCase>,
    ccc_map: &CompressedCCCMap,
    _qc_map: &QCMap,
    compositions: &BakedCompositions,
    table_type: TableType,
    _ucd: &UCD,
) -> Option<Encoded> {
    // Hangul syllables: in NFC/NFKC-only tables, these code points are always ignored.
    // Combining occurs only when a standalone jamo is encountered.
    // Hangul syllable membership is determined via simple arithmetic rather than lookups for optimal performance.
    if let DecompositionCase::HangulSyllable = case {
        let value = match table_type.is_composition_only() {
            true => encode_ignorable(),
            false => encode_hangul_syllable(),
        };

        return Some(value);
    }

    // Hangul jamo. Ignored in NFD/NFKD-only tables.
    if let DecompositionCase::HangulLeading | DecompositionCase::HangulVowelOrTrailing = case {
        if table_type.is_decomposition_only() {
            return Some(encode_ignorable());
        }

        let value = match case {
            DecompositionCase::HangulLeading => encode_ignorable(),
            DecompositionCase::HangulVowelOrTrailing => encode_vt_hangul_jamo(),
            _ => unreachable!(),
        };

        return Some(value);
    }

    // Compatibility decompositions: L-jamo.
    if let DecompositionCase::HangulCompatibilityIntoLeading { decomposition } = case {
        return Some(encode_singleton(decomposition));
    }

    // Decomposition into V/T jamo.
    if let DecompositionCase::HangulCompatibilityIntoVowelOrTrailing { decomposition } = case {
        let value = match table_type.is_decomposition_only() {
            true => encode_singleton(decomposition),
            false => {
                encode_expansion_combines_backwards(&[decomposition.clone()], ccc_map, compositions)
            }
        };

        return Some(value);
    }

    None
}

/// Decomposition: trailing starters.
fn handle_dec_trailing_starters(
    code: u32,
    case: &DecompositionCase,
    cases_map: &HashMap<u32, DecompositionCase>,
    ccc_map: &CompressedCCCMap,
    qc_map: &QCMap,
    compositions: &BakedCompositions,
    table_type: TableType,
    _ucd: &UCD,
) -> Option<Encoded> {
    let (is_case, decomposition) = case.is_ends_with_starter_case();

    if !is_case {
        return None;
    }

    // -- Decomposition-only tables.

    if table_type.is_decomposition_only() {
        return Some(encode_expansion_decomposition_only(decomposition, ccc_map));
    }

    // -- Composition-only tables.

    if table_type.is_composition_only() {
        // Recombines into the original code point.
        if let DecompositionCase::EndsWithStarterRecombinesIntoOriginal { .. } = case {
            return Some(encode_ignorable());
        }

        // Recombines into the original code point, combines forwards.
        if let DecompositionCase::EndsWithStarterRecombinesIntoOriginalCombinesForwards { .. } =
            case
        {
            let cp_compositions = compositions.index[&code];
            return Some(encode_starter_combines_forwards(cp_compositions));
        }

        // Singletons.
        if let DecompositionCase::EndsWithStarter { precomposition, .. }
        | DecompositionCase::EndsWithStarterCombinesForwards { precomposition, .. } = case
            && precomposition.len() == 1
        {
            return Some(encode_singleton_in_the_composition_table(
                &precomposition[0],
                compositions,
                cases_map,
            ));
        }
    }

    // -- Common case.

    // Recombines?
    if let DecompositionCase::EndsWithStarterRecombinesIntoOriginal { .. }
    | DecompositionCase::EndsWithStarterRecombinesIntoOriginalCombinesForwards { .. } = case
    {
        return Some(encode_expansion_recombines(
            decomposition,
            code,
            ccc_map,
            compositions,
        ));
    }

    // Decomposition ends with starters. May have precomposition.
    if let DecompositionCase::EndsWithStarter { precomposition, .. }
    | DecompositionCase::EndsWithStarterCombinesForwards { precomposition, .. } = case
    {
        assert!(qc_map.get(code) == 'N');

        return Some(encode_expansion_common_case(
            decomposition,
            precomposition,
            ccc_map,
            compositions,
            false,
        ));
    }

    // First starter of the decomposition may be combined with previous code point.
    if let DecompositionCase::EndsWithStarterCombinesBackwards { .. }
    | DecompositionCase::EndsWithStarterCombinesBoth { .. } = case
    {
        return Some(encode_expansion_combines_backwards(
            decomposition,
            ccc_map,
            compositions,
        ));
    }

    unreachable!()
}

fn handle_dec_trailing_nonstarters(
    code: u32,
    case: &DecompositionCase,
    cases_map: &HashMap<u32, DecompositionCase>,
    ccc_map: &CompressedCCCMap,
    qc_map: &QCMap,
    compositions: &BakedCompositions,
    table_type: TableType,
    _ucd: &UCD,
) -> Option<Encoded> {
    let (is_case, decomposition) = case.is_ends_with_nonstarter_case();

    if !is_case {
        return None;
    }

    // -- Decomposition-only tables.

    if table_type.is_decomposition_only() {
        return Some(encode_expansion_decomposition_only(decomposition, ccc_map));
    }

    // -- Composition-only tables.

    if table_type.is_composition_only() {
        // Recombines into the original code point.
        if let DecompositionCase::EndsWithNonstarterRecombinesIntoOriginal { .. } = case {
            return Some(encode_ignorable());
        }

        // Recombines into the original code point, combines forwards.
        if let DecompositionCase::EndsWithNonstarterRecombinesIntoOriginalCombinesForwards {
            ..
        } = case
        {
            let cp_compositions = compositions.index[&code];
            return Some(encode_starter_combines_forwards(cp_compositions));
        }

        // Singleton?
        if let DecompositionCase::EndsWithNonstarter { precomposition, .. }
        | DecompositionCase::EndsWithNonstarterCombinesForwards { precomposition, .. } = case
            && precomposition.len() == 1
        {
            return Some(encode_singleton_in_the_composition_table(
                &precomposition[0],
                compositions,
                cases_map,
            ));
        }
    }

    // -- Common case.

    // Recombines?
    if let DecompositionCase::EndsWithNonstarterRecombinesIntoOriginal { .. }
    | DecompositionCase::EndsWithNonstarterRecombinesIntoOriginalCombinesForwards { .. } = case
    {
        return Some(encode_expansion_recombines(
            decomposition,
            code,
            ccc_map,
            compositions,
        ));
    }

    // Decomposition ends with nonstarters. Common case.
    if let DecompositionCase::EndsWithNonstarter { precomposition, .. }
    | DecompositionCase::EndsWithNonstarterCombinesForwards { precomposition, .. } = case
    {
        return Some(encode_expansion_common_case(
            decomposition,
            precomposition,
            ccc_map,
            compositions,
            qc_map.get(code) == 'Y',
        ));
    }

    unreachable!()
}

/// Starters/nonstarters → nonstarters.
fn handle_dec_to_nonstarters(
    _code: u32,
    case: &DecompositionCase,
    _cases_map: &HashMap<u32, DecompositionCase>,
    ccc_map: &CompressedCCCMap,
    _qc_map: &QCMap,
    _compositions: &BakedCompositions,
    table_type: TableType,
    _ucd: &UCD,
) -> Option<Encoded> {
    if let DecompositionCase::StarterToNonstarters { decomposition }
    | DecompositionCase::NonstarterToNonstarters { decomposition } = case
    {
        if table_type.is_decomposition_only() {
            return Some(encode_expansion_decomposition_only(decomposition, ccc_map))
        }

        return Some(encode_expansion_nonstarters(decomposition, ccc_map));
    }

    None
}
