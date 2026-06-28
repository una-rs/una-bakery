use crate::codepoint::CompressedCCCMap;
use crate::errors::ParserError;

pub mod ucd;

pub fn ucd() -> Result<UCD, ParserError> {
    UCD::new(
        include_str!("./../../input/UCD/Blocks.txt"),
        include_str!("./../../input/UCD/UnicodeData.txt"),
        include_str!("./../../input/UCD/CompositionExclusions.txt"),
        include_str!("./../../input/UCD/DerivedNormalizationProps.txt"),
    )
}

pub fn normalization_test() -> Result<ucd::NormalizationTest, ParserError> {
    ucd::NormalizationTest::parse(include_str!("./../../input/UCD/NormalizationTest.txt"))
}

pub struct UCD {
    pub blocks: ucd::Blocks,
    pub unicode: ucd::UnicodeData,
    pub composition_exclusions: ucd::CompositionExclusions,
    pub quick_checks: ucd::QuickChecks,
    pub compressed_ccc: CompressedCCCMap,
}

impl UCD {
    pub fn new(
        blocks_source: &str,
        unicode_source: &str,
        exclusions_source: &str,
        quick_checks_source: &str,
    ) -> Result<Self, ParserError> {
        let blocks = ucd::Blocks::parse(blocks_source)?;
        let unicode = ucd::UnicodeData::parse(unicode_source, &blocks)?;
        let composition_exclusions = ucd::CompositionExclusions::parse(exclusions_source)?;
        let quick_checks = ucd::QuickChecks::parse(quick_checks_source)?;
        let compressed_ccc = CompressedCCCMap::generate(&unicode);

        Ok(Self {
            blocks,
            unicode,
            composition_exclusions,
            quick_checks,
            compressed_ccc,
        })
    }
}
