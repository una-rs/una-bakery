use crate::normalization::*;

/// Non-composable starters. No useful data is stored — only a marker.
pub fn encode_ignorable() -> Encoded {
    Encoded {
        value: Encoded::bits(Marker::Starter, StopFlag::None),
        expansion: None,
    }
}

/// Starters which may be combined with subsequent code points.
pub fn encode_starter_combines_forwards(compositions: CompositionInfo) -> Encoded {
    let mut value = Encoded::bits(Marker::Starter, StopFlag::None);

    value |= (compositions.baked() as u32) << 16;

    Encoded {
        value,
        expansion: None,
    }
}
