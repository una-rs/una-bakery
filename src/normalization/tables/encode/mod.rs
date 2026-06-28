use std::collections::HashMap;
use std::ops::Range;

use crate::codepoint::*;
use crate::data::{UCD, ucd::*};
use crate::normalization::*;

mod cases;
mod encode;
mod expansions;
mod value;

pub use cases::*;
pub use encode::*;
pub use expansions::*;
pub use value::*;

/// Encoded data for baking into tables.
pub struct EncodedData {
    range: Range<u32>,
    data: Vec<u32>,
    expansions: Vec<u32>,
}

impl EncodedData {
    /// Encodes a code point range for subsequent baking into tables.
    /// Sets expansion indices.
    pub fn encode(
        from: u32,
        to: u32,
        dec_map: &HashMap<u32, DecompositionCase>,
        ccc_map: &CompressedCCCMap,
        qc_map: &QCMap,
        compositions: &BakedCompositions,
        table_type: TableType,
        ucd: &UCD,
    ) -> Self {
        let range = Range {
            start: from,
            end: to + 1,
        };

        let mut data = vec![];
        let mut expansions = vec![];

        for code in from..=to {
            if dec_map.get(&code).is_none() {
                data.push(0);
                continue;
            };

            let mut encoded = encode(
                code,
                dec_map,
                ccc_map,
                qc_map,
                compositions,
                table_type,
                ucd,
            );

            if let Some(marker) = encoded.marker()
                && (marker == Marker::Expansion || marker == Marker::CombinesBackwardsOrNonstarters)
            {
                let index = expansions.len() as u16;
                let exp = encoded.expansion.unwrap();
                let exp_len = exp.len();

                assert!(index <= 0x3FFF); // obvious, but kept for clarity.
                assert!(exp_len <= 0x1F); // max decomposition len: 18 code points, see U+FDFA. max capacity: 0x1F.

                encoded.value = replace_expansion_index(encoded.value, index);
                expansions.extend_from_slice(&exp);
            }

            data.push(encoded.value);
        }

        Self {
            range,
            data,
            expansions,
        }
    }

    /// Data size.
    pub fn len(&self) -> u32 {
        self.data.len() as u32
    }

    /// Value for a code point.
    pub fn get(&self, cp: u32) -> Option<u32> {
        if !self.range.contains(&cp) {
            return None;
        }

        Some(self.data[cp as usize])
    }

    /// Get decomposition/composition expansion for a code point.
    pub fn get_expansion(&self, cp: u32, table_type: TableType) -> Option<&[u32]> {
        if !self.range.contains(&cp) {
            return None;
        }

        let val = self.data[cp as usize];

        match Marker::from_baked(val) {
            Some(Marker::Expansion) | Some(Marker::CombinesBackwardsOrNonstarters) => (),
            _ => return None,
        };

        let (idx, mut len) = (val >> 18, (val >> (8 + 5)) & 0x1F);

        if !table_type.is_decomposition_only() {
            let comp_info =
                ExpansionCompositionInfo::from_u32(self.expansions[idx as usize + len as usize]);

            len += 1;

            len += match comp_info.marker {
                CompositionCaseMarker::Precomposition => comp_info.length as u32,
                _ => 0,
            };
        }

        Some(&self.expansions[idx as usize..(idx + len) as usize])
    }
}
