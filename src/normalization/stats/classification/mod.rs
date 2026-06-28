use std::collections::HashMap;
use std::ops::RangeInclusive;

use crate::codepoint::CodepointsBlock;
use crate::data::UCD;
use crate::normalization::*;

mod cases;
mod fmt_codepoint;
mod fmt_combines;
mod fmt_decomposition;
mod index;

pub struct DecompositionStats {
    groups: Vec<DecompositionGroupStats>,
}

impl DecompositionStats {
    pub fn collect(cmap: &HashMap<u32, DecompositionCase>, ucd: &UCD) -> Self {
        let mut data: Vec<DecompositionGroupStats> = DecompositionCaseTag::table()
            .iter()
            .map(|meta| DecompositionGroupStats {
                meta: meta.clone(),
                codepoints: Vec::new(),
                blocks: Vec::new(),
            })
            .collect();

        let mut codes: Vec<&u32> = cmap.keys().collect();
        codes.sort();

        for code in codes {
            let case = &cmap[code];

            data[case.tag() as usize].push(
                *code,
                case.clone(),
                ucd.blocks.get_by_codepoint(*code).unwrap(),
            );
        }

        Self { groups: data }
    }
}

pub struct DecompositionGroupStats {
    pub meta: DecompositionCaseMeta,
    pub codepoints: Vec<(u32, DecompositionCase)>,
    pub blocks: Vec<CodepointsBlock>,
}

impl DecompositionGroupStats {
    pub fn range(&self) -> Option<RangeInclusive<u32>> {
        if self.codepoints.is_empty() {
            return None;
        }

        Some(self.codepoints[0].0..=self.codepoints.last().unwrap().0)
    }

    pub fn push(&mut self, code: u32, case: DecompositionCase, block: CodepointsBlock) {
        if !self.blocks.contains(&block) {
            self.blocks.push(block);
        }

        self.codepoints.push((code, case));
    }
}
