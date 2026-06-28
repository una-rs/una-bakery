use crate::errors::UnknownPropertyValue;

/// Simple Uppercase/Lowercase/Titlecase Mapping.
/// Taken from the UCD: fields 12, 13, and 14 of UnicodeData.txt.
///
/// The corresponding uppercase/lowercase/titlecase character — a single code point.
///
/// For more details, see https://www.unicode.org/reports/tr44/#Casemapping.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SimpleCaseMapping {
    None,
    Some(u32),
}

impl SimpleCaseMapping {}

impl TryFrom<&str> for SimpleCaseMapping {
    type Error = UnknownPropertyValue;

    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value.is_empty() {
            true => Self::None,
            false => match u32::from_str_radix(value, 16) {
                Ok(value) => Self::Some(value),
                Err(_) => return Err(value.to_string()),
            },
        })
    }
}
