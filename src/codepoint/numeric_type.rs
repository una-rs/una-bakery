use crate::errors::UnknownPropertyValue;

/// Numeric Type and Numeric Value.
/// Taken from the UCD: fields 6, 7, and 8 of UnicodeData.txt.
///
/// In addition to the values present in UnicodeData.txt, note that this property also has meaningful values for CJK characters (Ideographic Numeric Values).
///
/// https://www.unicode.org/reports/tr44/#Numeric_Type
#[derive(Debug, Clone, PartialEq)]
pub enum NumericType {
    /// Not a numeric value.
    None,
    /// Decimal digit (0–9).
    Decimal(u8),
    /// Digit (e.g., compatibility superscripts).
    Digit(u8),
    /// Numeric value (e.g., fractions like "1/5").
    Numeric(String),
}

impl NumericType {
    pub fn is_some(&self) -> bool {
        !matches!(self, NumericType::None)
    }

    pub fn is_none(&self) -> bool {
        matches!(self, NumericType::None)
    }
}

impl TryFrom<(&str, &str, &str)> for NumericType {
    type Error = UnknownPropertyValue;

    fn try_from(v: (&str, &str, &str)) -> Result<Self, Self::Error> {
        let mask = u8::from(!v.0.is_empty())
            | u8::from(!v.1.is_empty()) << 1
            | u8::from(!v.2.is_empty()) << 2;

        let value = match mask {
            0b111 => Self::Decimal(v.0.parse().map_err(|_| v.0.to_string())?),
            0b110 => Self::Digit(v.1.parse().map_err(|_| v.1.to_string())?),
            0b100 => Self::Numeric(v.2.to_owned()),
            0b000 => Self::None,
            _ => unreachable!(),
        };

        Ok(value)
    }
}
