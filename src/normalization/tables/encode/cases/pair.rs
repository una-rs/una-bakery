use crate::codepoint::Codepoint;
use crate::normalization::*;

pub fn is_bakeable_pair(dec: &[Codepoint]) -> bool {
    if dec[0].code > 0x7FFF || dec[1].code > 0xFFFF {
        return false;
    }

    if (dec[0].code << 1) as u8 <= MARKER_AND_FLAG_MAX_VALUE {
        return false;
    }

    true
}

/// Pair: starter → starter + nonstarter.
///
/// May be recombined into the original code point if there will be no subsequent combining nonstarters
/// with the CCC less than the CCC of the decomposition's second code point.
///
/// Compositions info is not stored here, it is assumed that we will need perform an additional table lookup.
pub fn encode_pair_maybe_recomposed(dec: &[Codepoint]) -> Encoded {
    let mut value = Encoded::bits(Marker::None, StopFlag::None);

    value |= (dec[0].code << 1) | (dec[1].code << 16);

    assert!(is_bakeable_pair(dec));

    Encoded {
        value,
        expansion: None,
    }
}

/// Pair: starter → starter + nonstarter.
///
/// Cannot be recombined into the original code point.
///
/// Compositions info is not stored here, it is assumed that we will need perform an additional table lookup.
pub fn encode_pair_no_recomposition(dec: &[Codepoint]) -> Encoded {
    let mut value = Encoded::bits(Marker::None, StopFlag::Enabled);

    value |= (dec[0].code << 1) | (dec[1].code << 16);

    assert!(is_bakeable_pair(dec));

    Encoded {
        value,
        expansion: None,
    }
}
