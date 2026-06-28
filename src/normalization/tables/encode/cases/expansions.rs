use crate::codepoint::{Codepoint, CompressedCCCMap};
use crate::normalization::*;

/// Decomposition-only tables expansions.
/// No need in the stop flag.
pub fn encode_expansion_decomposition_only(
    dec: &[Codepoint],
    ccc_map: &CompressedCCCMap,
) -> Encoded {
    let (value, expansion) =
        expansion_decomposition(Marker::Expansion, StopFlag::None, dec, ccc_map);

    Encoded {
        value,
        expansion: Some(expansion),
    }
}

/// Common case: decomposition, precomposition, combining.
pub fn encode_expansion_common_case(
    dec: &[Codepoint],
    pre: &[Codepoint],
    ccc_map: &CompressedCCCMap,
    compositions: &BakedCompositions,
    may_recombine: bool,
) -> Encoded {
    let flag = match may_recombine {
        true => StopFlag::None,
        false => StopFlag::Enabled,
    };

    let (value, mut expansion) = expansion_decomposition(Marker::Expansion, flag, dec, ccc_map);

    match dec == pre {
        true => {
            expansion.push(
                ExpansionCompositionInfo::from(
                    CompositionCaseMarker::Decomposition,
                    dec,
                    compositions,
                )
                .baked(),
            );
        }
        false => {
            expansion.push(
                ExpansionCompositionInfo::from(
                    CompositionCaseMarker::Precomposition,
                    pre,
                    compositions,
                )
                .baked(),
            );

            expansion.append(&mut expansion_vec(pre, ccc_map));
        }
    }

    Encoded {
        value,
        expansion: Some(expansion),
    }
}

/// Recombines into the original code point (always).
pub fn encode_expansion_recombines(
    dec: &[Codepoint],
    code: u32,
    ccc_map: &CompressedCCCMap,
    compositions: &BakedCompositions,
) -> Encoded {
    let (value, mut expansion) =
        expansion_decomposition(Marker::Expansion, StopFlag::None, dec, ccc_map);

    expansion.push(
        ExpansionCompositionInfo::from_code(
            CompositionCaseMarker::AlwaysRecombines,
            code,
            compositions,
        )
        .baked(),
    );

    Encoded {
        value,
        expansion: Some(expansion),
    }
}

/// May be combined with the previous code point.
pub fn encode_expansion_combines_backwards(
    dec: &[Codepoint],
    ccc_map: &CompressedCCCMap,
    compositions: &BakedCompositions,
) -> Encoded {
    let (value, mut expansion) = expansion_decomposition(
        Marker::CombinesBackwardsOrNonstarters,
        StopFlag::Enabled,
        dec,
        ccc_map,
    );

    debug_assert!(dec.iter().all(|c| c.is_starter()));
    debug_assert!(dec.len() <= 3);

    expansion.push(
        ExpansionCompositionInfo::from(CompositionCaseMarker::Decomposition, dec, compositions)
            .baked(),
    );

    Encoded {
        value,
        expansion: Some(expansion),
    }
}

/// Code point → nonstarters.
pub fn encode_expansion_nonstarters(dec: &[Codepoint], ccc_map: &CompressedCCCMap) -> Encoded {
    let (value, mut expansion) = expansion_decomposition(
        Marker::CombinesBackwardsOrNonstarters,
        StopFlag::Enabled,
        dec,
        ccc_map,
    );

    expansion.push(ExpansionCompositionInfo::nonstarters().baked());

    Encoded {
        value,
        expansion: Some(expansion),
    }
}

/// Starters which may be combined with previous code points. Encoded as expansion.
pub fn encode_starter_combines_backwards(code: u32) -> Encoded {
    expansion_starter_combines_backwards(code, None)
}

/// Starters which may be combined either with both previous and upcoming code points. Encoded as expansion.
pub fn encode_starter_combines_forwards_backwards(
    code: u32,
    compositions: CompositionInfo,
) -> Encoded {
    expansion_starter_combines_backwards(code, Some(compositions))
}

fn expansion_decomposition(
    marker: Marker,
    flag: StopFlag,
    dec: &[Codepoint],
    ccc_map: &CompressedCCCMap,
) -> (u32, Vec<u32>) {
    let mut value = Encoded::bits(marker, flag);
    let expansion = expansion_vec(dec, ccc_map);

    let dec_exp_info = ExpansionInfo::from(0, dec);

    value |= (dec_exp_info.baked() as u32) << 8;

    (value, expansion)
}

fn expansion_starter_combines_backwards(
    code: u32,
    composition_info: Option<CompositionInfo>,
) -> Encoded {
    let mut value = Encoded::bits(Marker::CombinesBackwardsOrNonstarters, StopFlag::Enabled);

    let exp_info = ExpansionInfo {
        length: 1,
        last_starter: 0,
        offset: 0,
    };

    value |= (exp_info.baked() as u32) << 8;

    let mut expansion = starter_expansion_vec(code);

    expansion.push(
        ExpansionCompositionInfo {
            marker: CompositionCaseMarker::AlwaysRecombines,
            length: 1,
            last_starter: 0,
            composition_info,
        }
        .baked(),
    );

    Encoded {
        value,
        expansion: Some(expansion),
    }
}
