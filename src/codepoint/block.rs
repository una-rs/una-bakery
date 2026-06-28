use std::ops::RangeInclusive;

#[derive(Debug, Clone, PartialEq)]
pub struct CodepointsBlock {
    pub name: String,
    pub from: u32,
    pub to: u32,
}

impl CodepointsBlock {
    pub fn range(&self) -> RangeInclusive<u32> {
        self.from..=self.to
    }
}
