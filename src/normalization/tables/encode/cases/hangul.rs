use crate::normalization::*;

/// Hangul syllable.
/// Though we can store L/V/T values directly, it will cause data table bloating.
pub fn encode_hangul_syllable() -> Encoded {
    let value = Encoded::bits(Marker::HangulSyllable, StopFlag::None);

    Encoded {
        value,
        expansion: None,
    }
}

/// Hangul V/T jamo.
pub fn encode_vt_hangul_jamo() -> Encoded {
    let value = Encoded::bits(Marker::Starter, StopFlag::Enabled);

    Encoded {
        value,
        expansion: None,
    }
}
