use crate::errors::UnknownPropertyValue;

/// Decomposition.
/// Taken from the UCD: fifth field of UnicodeData.txt.
///
/// https://www.unicode.org/reports/tr44/#Decomposition_Type
#[derive(Debug, Clone)]
pub struct Decomposition {
    pub codes: Vec<u32>,
    pub tag: Option<DecompositionTag>,
}

impl TryFrom<&str> for Decomposition {
    type Error = UnknownPropertyValue;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (tag_string, decomposition_string) = match value.starts_with('<') {
            true => value.split_once(' ').ok_or_else(|| value.to_string())?,
            false => ("", value),
        };

        let tag = match !tag_string.is_empty() {
            true => Some(DecompositionTag::try_from(tag_string)?),
            false => None,
        };

        let codes: Vec<u32> = decomposition_string
            .split_whitespace()
            .map(|v| u32::from_str_radix(v, 16).unwrap())
            .collect();

        Ok(Self { codes, tag })
    }
}

/// Decomposition tag.
/// Taken from the UCD: fifth field of UnicodeData.txt.
///
/// Presence flag + 16 variants, fits in 5 bits.
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum DecompositionTag {
    /// Font variant.
    Font = 0,
    /// No-break version of a space or hyphen.
    NoBreak = 1,
    /// Initial presentation form (Arabic).
    Initial = 2,
    /// Medial presentation form (Arabic).
    Medial = 3,
    /// Final presentation form (Arabic).
    Final = 4,
    /// Isolated presentation form (Arabic).
    Isolated = 5,
    /// Encircled form.
    Circle = 6,
    /// Superscript form.
    Super = 7,
    /// Subscript form.
    Sub = 8,
    /// Vertical layout presentation form.
    Vertical = 9,
    /// Wide (or zenkaku) compatibility character.
    Wide = 10,
    /// Narrow (or hankaku) compatibility character.
    Narrow = 11,
    /// Small variant form (CNS compatibility).
    Small = 12,
    /// CJK squared font variant.
    Square = 13,
    /// Vulgar fraction form.
    Fraction = 14,
    /// Otherwise unspecified compatibility character.
    Compat = 15,
}

impl TryFrom<&str> for DecompositionTag {
    type Error = UnknownPropertyValue;

    #[inline]
    fn try_from(abbr: &str) -> Result<Self, Self::Error> {
        Ok(match abbr {
            "<font>" => Self::Font,
            "<noBreak>" => Self::NoBreak,
            "<initial>" => Self::Initial,
            "<medial>" => Self::Medial,
            "<final>" => Self::Final,
            "<isolated>" => Self::Isolated,
            "<circle>" => Self::Circle,
            "<super>" => Self::Super,
            "<sub>" => Self::Sub,
            "<vertical>" => Self::Vertical,
            "<wide>" => Self::Wide,
            "<narrow>" => Self::Narrow,
            "<small>" => Self::Small,
            "<square>" => Self::Square,
            "<fraction>" => Self::Fraction,
            "<compat>" => Self::Compat,
            _ => return Err(abbr.to_string()),
        })
    }
}

impl From<DecompositionTag> for u8 {
    #[inline]
    fn from(value: DecompositionTag) -> Self {
        unsafe { core::mem::transmute::<DecompositionTag, u8>(value) }
    }
}

impl core::fmt::Display for DecompositionTag {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let value = match self {
            DecompositionTag::Font => "<font>",
            DecompositionTag::NoBreak => "<noBreak>",
            DecompositionTag::Initial => "<initial>",
            DecompositionTag::Medial => "<medial>",
            DecompositionTag::Final => "<final>",
            DecompositionTag::Isolated => "<isolated>",
            DecompositionTag::Circle => "<circle>",
            DecompositionTag::Super => "<super>",
            DecompositionTag::Sub => "<sub>",
            DecompositionTag::Vertical => "<vertical>",
            DecompositionTag::Wide => "<wide>",
            DecompositionTag::Narrow => "<narrow>",
            DecompositionTag::Small => "<small>",
            DecompositionTag::Square => "<square>",
            DecompositionTag::Fraction => "<fraction>",
            DecompositionTag::Compat => "<compat>",
        };

        write!(f, "{}", value)
    }
}
