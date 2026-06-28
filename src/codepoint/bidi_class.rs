use crate::errors::UnknownPropertyValue;

/// Bidirectional text class (bidi class).
/// Taken from the UCD: fourth field of UnicodeData.txt.
///
/// 23 variants, fits in 5 bits.
///
/// https://www.unicode.org/reports/tr44/#Bidi_Class_Values
///
/// Class groups:
///
///   - Strong (L, R, AL): characters with an explicit, inherent directionality.
///   - Weak (EN, ES, ET, AN, CS, NSM, BN): characters whose direction is determined by context.
///   - Neutral (B, S, WS, ON): characters with no inherent direction that do not affect text direction.
///   - Explicit formatting (LRE, LRO, RLE, RLO, PDF, LRI, RLI, FSI, PDI): control characters used to explicitly manage text direction.
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum BidiClass {
    /// L – any strong left-to-right character.
    LeftToRight = 1,
    /// R – any strong right-to-left (non-Arabic-type) character.
    RightToLeft = 2,
    /// AL – any strong right-to-left (Arabic-type) character.
    ArabicLetter = 3,

    /// EN – any ASCII digit or Eastern Arabic-Indic digit.
    EuropeanNumber = 4,
    /// ES – plus and minus signs.
    EuropeanSeparator = 5,
    /// ET – a terminator in a numeric format context, includes currency signs.
    EuropeanTerminator = 6,
    /// AN – any Arabic-Indic digit.
    ArabicNumber = 7,
    /// CS – commas, colons, and slashes.
    CommonSeparator = 8,
    /// NSM – any nonspacing mark.
    NonspacingMark = 9,
    /// BN – most format characters, control codes, or noncharacters.
    BoundaryNeutral = 10,

    /// B – various newline characters that separate paragraphs.
    ParagraphSeparator = 12,
    /// S – various segment-related control codes.
    SegmentSeparator = 13,
    /// WS – whitespace characters such as spaces and tabs.
    Whitespace = 14,
    /// ON – most other symbols and punctuation marks with no specific directional influence.
    OtherNeutral = 15,

    /// LRE – U+202A: left-to-right embedding control.
    LeftToRightEmbedding = 16,
    /// LRO – U+202D: left-to-right override control.
    LeftToRightOverride = 17,
    /// RLE – U+202B: right-to-left embedding control.
    RightToLeftEmbedding = 18,
    /// RLO – U+202E: right-to-left override control.
    RightToLeftOverride = 19,
    /// PDF – U+202C: pop directional format.
    PopDirectionalFormat = 20,
    /// LRI – U+2066: left-to-right isolate control.
    LeftToRightIsolate = 21,
    /// RLI – U+2067: right-to-left isolate control.
    RightToLeftIsolate = 22,
    /// FSI – U+2068: first strong isolate control.
    FirstStrongIsolate = 23,
    /// PDI – U+2069: pop directional isolate.
    PopDirectionalIsolate = 24,
}

impl BidiClass {
    #[inline]
    pub fn is_strong(&self) -> bool {
        u8::from(*self) < 4
    }

    #[inline]
    pub fn is_weak(&self) -> bool {
        let value = u8::from(*self);

        value & 0b_1111_1100 == 0b_0000_0100 || value & 0b_1111_1100 == 0b_0000_1000
    }

    #[inline]
    pub fn is_neutral(&self) -> bool {
        u8::from(*self) & 0b_1111_1100 == 0b_0000_1100
    }

    #[inline]
    pub fn is_explicit(&self) -> bool {
        u8::from(*self) & 0b_1111_0000 == 0b_0001_0000
    }
}

impl TryFrom<&str> for BidiClass {
    type Error = UnknownPropertyValue;

    #[inline]
    fn try_from(abbr: &str) -> Result<Self, Self::Error> {
        Ok(match abbr {
            "L" => Self::LeftToRight,
            "R" => Self::RightToLeft,
            "AL" => Self::ArabicLetter,
            "EN" => Self::EuropeanNumber,
            "ES" => Self::EuropeanSeparator,
            "ET" => Self::EuropeanTerminator,
            "AN" => Self::ArabicNumber,
            "CS" => Self::CommonSeparator,
            "NSM" => Self::NonspacingMark,
            "BN" => Self::BoundaryNeutral,
            "B" => Self::ParagraphSeparator,
            "S" => Self::SegmentSeparator,
            "WS" => Self::Whitespace,
            "ON" => Self::OtherNeutral,
            "LRE" => Self::LeftToRightEmbedding,
            "LRO" => Self::LeftToRightOverride,
            "RLE" => Self::RightToLeftEmbedding,
            "RLO" => Self::RightToLeftOverride,
            "PDF" => Self::PopDirectionalFormat,
            "LRI" => Self::LeftToRightIsolate,
            "RLI" => Self::RightToLeftIsolate,
            "FSI" => Self::FirstStrongIsolate,
            "PDI" => Self::PopDirectionalIsolate,
            _ => return Err(abbr.to_string()),
        })
    }
}

impl From<BidiClass> for u8 {
    #[inline]
    fn from(value: BidiClass) -> Self {
        unsafe { core::mem::transmute::<BidiClass, u8>(value) }
    }
}
