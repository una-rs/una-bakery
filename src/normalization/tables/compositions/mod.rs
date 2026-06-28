use std::collections::HashMap;

use crate::normalization::Compositions;

/// Information about stored compositions for a starter.
#[derive(Default, Clone, Copy)]
pub struct CompositionInfo {
    /// Index of the first entry in the composition table.
    pub index: u16,
    /// Number of compositions for this starter.
    pub count: u8,
}

impl CompositionInfo {
    /// Stored composition info in compressed form (LE):
    /// [zzzz zzzz zzz] [n nnnn]
    ///      11 bits      5 bits      
    ///        \           \-------- number of pairs
    ///         \------------------- index in pair table
    pub fn baked(&self) -> u16 {
        assert!(self.index <= 0x7FF);
        assert!(self.count <= 0x1F);

        self.index | ((self.count as u16) << 11)
    }

    pub fn from(value: u16) -> Option<Self> {
        match value {
            0 => None,
            _ => {
                let index = value & 0x7FF;
                let count = (value >> 11) as u8;

                Some(Self { index, count })
            }
        }
    }
}

/// Baked compositions — value array and indices for code points.
///
/// Record format in table (LE):
/// xxxx xxxx  xxxx xxxx    xxyy yyyy  yyyy yyyy    yyyy ____  ____ ____    iiii iiii  iiii iiii
///
///   - xx.. — second code point
///   - yy.. — combination result
///   - ii.. — compressed composition info for the result (see CompositionInfo)
#[derive(Clone)]
pub struct BakedCompositions {
    pub table: Vec<u64>,
    pub index: HashMap<u32, CompositionInfo>,
    // pub backwards_index: HashMap<u32, CompositionInfo>,
}

impl BakedCompositions {
    pub fn bake(compositions: &Compositions) -> Self {
        let mut table = Vec::new();
        let mut index = HashMap::new();
        // let mut backwards_index = HashMap::new();

        let mut firsts = compositions.forwards_list();
        firsts.sort();

        // Table of records for each composable starter: code point it combines with, result.
        for first in firsts {
            let pairs = compositions.forwards(first).unwrap();

            let mut seconds: Vec<u32> = pairs.keys().copied().collect();
            seconds.sort();

            index.insert(
                first,
                CompositionInfo {
                    index: table.len() as u16,
                    count: seconds.len() as u8,
                },
            );

            for second in seconds {
                // Value must store:
                //
                // 1. Second code point.
                // 2. Resulting code point.
                // 3. If the resulting code point can be combined further — offset and variant count, added in next step.

                let combined = *pairs.get(&second).unwrap();
                let value = (second as u64) | ((combined as u64) << 18);

                table.push(value);
            }
        }

        // For each recorded composable pair, write additional info:
        // reference to combination variants of the resulting code point and their count.
        for value in table.iter_mut() {
            let result = (*value >> 18) as u32;

            if let Some(info) = index.get(&result) {
                *value |= (info.baked() as u64) << 48;
            }
        }

        Self {
            table,
            index,
            // backwards_index,
        }
    }

    pub fn len(&self) -> usize {
        self.table.len()
    }
}
