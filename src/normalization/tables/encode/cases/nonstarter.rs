use crate::codepoint::CompressedCCC;
use crate::normalization::*;

/// Nonstarter. Stores CCC.
pub fn encode_nonstarter(ccc: CompressedCCC) -> Encoded {
    let mut value = Encoded::bits(Marker::Nonstarter, StopFlag::Enabled);

    value |= (ccc.u8() as u32) << 8;

    Encoded {
        value,
        expansion: None,
    }
}
