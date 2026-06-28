mod bidi_class;
mod bidi_mirrored;
mod block;
mod canonical_combining_class;
mod decomposition;
mod general_category;
mod numeric_type;
mod simple_case_mapping;

pub use bidi_class::*;
pub use bidi_mirrored::*;
pub use block::*;
pub use canonical_combining_class::*;
pub use decomposition::*;
pub use general_category::*;
pub use numeric_type::*;
pub use simple_case_mapping::*;

/// A Unicode code point.
/// Source: UCD, UnicodeData.txt.
///
/// https://www.unicode.org/reports/tr44/
#[derive(Debug, Clone, PartialEq)]
pub struct Codepoint {
    /// Code point value.
    pub code: u32,
    /// Name of the character.
    pub name: String,
    /// General category.
    pub gc: GeneralCategory,
    /// Canonical combining class.
    pub ccc: CanonicalCombiningClass,
    /// Bidirectional class.
    pub bc: BidiClass,
    /// Numeric type and value.
    pub numeric: NumericType,
    /// Indicates whether the character is mirrored in bidirectional text.
    pub bidi_mirrored: BidiMirrored,
    /// Simple uppercase mapping (single-character result).
    pub simple_uppercase_mapping: SimpleCaseMapping,
    /// Simple lowercase mapping (single-character result).
    pub simple_lowercase_mapping: SimpleCaseMapping,
    /// Simple titlecase mapping (single-character result).
    pub simple_titlecase_mapping: SimpleCaseMapping,
    /// Decomposition tag (e.g., <font>, <compat>, etc.), if present.
    pub decomposition_tag: Option<DecompositionTag>,
    /// Decomposition mapping as a sequence of code points.
    pub decomposition: Vec<u32>,
    /// Block to which the code point belongs (from Blocks.txt).
    pub block: Option<CodepointsBlock>,
}

impl Codepoint {
    pub fn is_starter(&self) -> bool {
        self.ccc.is_starter()
    }

    pub fn is_nonstarter(&self) -> bool {
        self.ccc.is_nonstarter()
    }
}
