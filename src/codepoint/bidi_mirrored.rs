use crate::errors::UnknownPropertyValue;

/// A "mirrored" character in bidirectional text (Bidi Mirrored).
/// Taken from the UCD: ninth field of UnicodeData.txt.
///
/// For example, parentheses.
///
/// https://www.unicode.org/reports/tr44/#Bidi_Mirrored
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BidiMirrored(bool);

impl BidiMirrored {
    #[inline]
    pub fn is_mirrored(&self) -> bool {
        self.0
    }
}

impl TryFrom<&str> for BidiMirrored {
    type Error = UnknownPropertyValue;

    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "Y" => Self(true),
            "N" => Self(false),
            _ => return Err(value.to_string()),
        })
    }
}
