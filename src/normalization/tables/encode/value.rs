pub const MARKER_AND_FLAG_MAX_VALUE: u8 = ((MAX_MARKER as u8) << 1) | StopFlag::Enabled as u8;

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Marker {
    /// Starters. Ignored and combines-backwards variants.
    Starter = 0b_000,
    /// Nonstarters. No decomposition.
    Nonstarter = 0b_001,
    /// Singletons.
    Singleton = 0b_010,
    /// Expansions — non-standard cases.
    Expansion = 0b_011,
    /// Hangul syllable.
    HangulSyllable = 0b_100,
    /// Expansion — first starter may be combines with the preceding code point / nonstarters decomposition.
    CombinesBackwardsOrNonstarters = 0b_101,
}

const MAX_MARKER: Marker = Marker::CombinesBackwardsOrNonstarters;

impl Marker {
    #[allow(non_upper_case_globals)]
    pub const None: Marker = Marker::Starter;

    pub fn from_baked(baked: u32) -> Option<Self> {
        if baked as u8 > MARKER_AND_FLAG_MAX_VALUE {
            return None;
        }

        Some(match (baked as u8) >> 1 {
            0b_000 => Marker::Starter,
            0b_001 => Marker::Nonstarter,
            0b_010 => Marker::Singleton,
            0b_011 => Marker::Expansion,
            0b_100 => Marker::HangulSyllable,
            0b_101 => Marker::CombinesBackwardsOrNonstarters,

            _ => unreachable!(),
        })
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub enum StopFlag {
    None = 0,
    Enabled = 1,
}

impl From<u8> for StopFlag {
    fn from(value: u8) -> Self {
        match value {
            0 => StopFlag::None,
            1 => StopFlag::Enabled,

            _ => unreachable!(),
        }
    }
}

/// Encoded code point value, and — if present — corresponding expansion data.
/// If expansion exists, its index is not stored here — it is mapped during table baking.
pub struct Encoded {
    pub value: u32,
    pub expansion: Option<Vec<u32>>,
}

impl Encoded {
    /// Prepares a value from marker bits and flag.
    pub fn bits(marker: Marker, flag: StopFlag) -> u32 {
        ((marker as u32) << 1) | (flag as u32)
    }

    /// Marker value.
    pub fn marker(&self) -> Option<Marker> {
        Marker::from_baked(self.value)
    }

    /// Flag value.
    pub fn flag(&self) -> StopFlag {
        StopFlag::from((self.value as u8) & 1)
    }

    /// Replace the flag.
    pub fn set_flag(&mut self, flag: StopFlag) {
        match flag {
            StopFlag::Enabled => self.value |= 1,
            StopFlag::None => self.value &= !1,
        }
    }
}
