use std::collections::HashMap;

use crate::data::UCD;
use crate::errors::TableError;
use crate::normalization::*;

// Last code point in the UCD that has a decomposition.
pub const LAST_DECOMPOSITION_CODEPOINT: u32 = 0x2FA1D;

// Number of bits required to encode a code point with decomposition (U+2FA1D).
const CODEPOINT_BITS: u8 = 18;
// Number of high bits (out of 18) used as page index.
const INDEX_BITS: u8 = 11;
// Number of middle bits used as leaf index within a page.
const PAGE_BITS: u8 = CODEPOINT_BITS - INDEX_BITS - LEAF_BITS;
// Number of low bits used as index within a leaf block.
const LEAF_BITS: u8 = 3;

// Total number of pages.
const PAGES_TOTAL: u32 =
    (LAST_DECOMPOSITION_CODEPOINT + CODEPOINTS_IN_PAGE - 1) / CODEPOINTS_IN_PAGE;
// Number of leaf blocks per page.
const LEAFS_IN_PAGE: u32 = 1 << PAGE_BITS;
// Number of code points per page.
const CODEPOINTS_IN_PAGE: u32 = 1 << (PAGE_BITS + LEAF_BITS);
// Number of code points per leaf block.
const CODEPOINTS_IN_LEAF: u32 = 1 << LEAF_BITS;

// Up to this code point (inclusive), leaf block indices are stored contiguously.
const CONTINUOUS_BLOCK_END: u32 = 0xFFF;

const CONTINUOUS_BLOCK_ALIGN: u32 =
    (((CONTINUOUS_BLOCK_END >> (PAGE_BITS + LEAF_BITS)) + 1) * LEAFS_IN_PAGE).next_power_of_two();

// Offset in the index array where page index blocks begin.
// Computed by aligning PAGES_TOTAL, accounting for CONTINUOUS_BLOCK_END mask.
const PAGES_OFFSET: u16 =
    ((PAGES_TOTAL + CONTINUOUS_BLOCK_ALIGN - 1) & !(CONTINUOUS_BLOCK_ALIGN - 1)) as u16;

pub struct BakedDecompositions {
    pub data: Vec<u32>,
    pub index: Vec<u16>,
}

impl BakedDecompositions {
    pub fn bake(
        dec_map: &HashMap<u32, DecompositionCase>,
        ucd: &UCD,
        norm_type: NormType,
        table_type: TableType,
    ) -> Result<Self, TableError> {
        let compositions = Compositions::generate(&ucd.unicode, &ucd.composition_exclusions);
        let ctable = BakedCompositions::bake(&compositions);

        let source = EncodedData::encode(
            0,
            CODEPOINTS_IN_PAGE * PAGES_TOTAL - 1,
            &dec_map,
            &ucd.compressed_ccc,
            ucd.quick_checks.composing(norm_type),
            &ctable,
            table_type,
            ucd,
        );

        let (mut data, index) = bake_base_table(&source)?;

        bake_expansions_into_data(&mut data, &index, &source, table_type);

        Ok(Self { data, index })
    }

    pub fn codepoint_data(&self, cp: u32) -> (u32, u16) {
        let idx = get_data_index(cp, &self.index);
        let val = self.data[idx as usize];

        (val, idx)
    }
}

/// Bakes the base lookup table.
fn bake_base_table(source: &EncodedData) -> Result<(Vec<u32>, Vec<u16>), TableError> {
    let mut baked: Vec<u32> = vec![];
    let mut index: Vec<u16> = vec![];

    (0..PAGES_OFFSET).for_each(|_| index.push(0));

    for page_idx in 0..PAGES_TOTAL {
        let mut page: Vec<u16> = vec![];

        for leaf_idx in 0..LEAFS_IN_PAGE {
            let leaf = get_leaf_from_source(page_idx * LEAFS_IN_PAGE + leaf_idx, source);

            let index = match contains_block(&leaf, &baked) {
                Some(index) => index,
                None => {
                    let index = baked.len();
                    baked.extend_from_slice(&leaf);
                    index
                }
            };

            if index > u16::MAX as usize {
                return Err(TableError::OutOfRange {
                    codepoint: (page_idx * LEAFS_IN_PAGE + leaf_idx) << LEAF_BITS,
                });
            }

            page.push(index as u16);
        }

        match (page_idx + 1) * CODEPOINTS_IN_PAGE < CONTINUOUS_BLOCK_END {
            true => index.extend_from_slice(page.as_slice()),
            false => match contains_block(&page, &index.as_slice()[PAGES_OFFSET as usize..]) {
                Some(existing_index) => {
                    index[page_idx as usize] = PAGES_OFFSET as u16 + existing_index as u16;
                }
                None => {
                    index[page_idx as usize] = index.len() as u16;
                    index.extend_from_slice(&page);
                }
            },
        }
    }

    Ok((baked, index))
}

/// Retrieves a leaf block from source data by its index.
fn get_leaf_from_source(leaf_idx: u32, source: &EncodedData) -> [u32; CODEPOINTS_IN_LEAF as usize] {
    let mut leaf = [0u32; CODEPOINTS_IN_LEAF as usize];

    let from = leaf_idx << LEAF_BITS;
    let to = from + CODEPOINTS_IN_LEAF;

    if from >= source.len() {
        return leaf;
    }

    for (i, cp) in (from..std::cmp::min(to, source.len())).enumerate() {
        leaf[i] = source.get(cp).unwrap();
    }

    leaf
}

/// Bakes expansion data into the main table.
fn bake_expansions_into_data(
    data: &mut Vec<u32>,
    index: &Vec<u16>,
    source: &EncodedData,
    table_type: TableType,
) {
    let base_data_len = data.len();

    for cp in 0..=LAST_DECOMPOSITION_CODEPOINT {
        let idx = get_data_index(cp, index);

        let exp = match source.get_expansion(cp, table_type) {
            Some(exp) => exp,
            None => continue,
        };

        let exp_len = exp.len();
        assert!(exp_len > 0 && exp_len <= u8::MAX as usize);

        let mut exp_idx = data.len();

        if table_type.is_decomposition_only()
            && let Some(gap) = find_gap(&data[..base_data_len], exp_len)
        {
            exp_idx = gap;

            for (i, &val) in exp.iter().enumerate() {
                data[gap + i] |= val;
            }
        } else {
            data.extend_from_slice(exp);
        }

        data[idx as usize] = replace_expansion_index(data[idx as usize], exp_idx as u16);
    }
}

/// Finds a free gap in data for overlaying an expansion.
fn find_gap(data: &[u32], len: usize) -> Option<usize> {
    let mut start = 0;
    let mut found_len = 0;

    for (i, &val) in data.iter().enumerate() {
        // Starter / ignoring the stop flag.
        if val & !1 != 0 {
            found_len = 0;
            continue;
        };

        if found_len == 0 {
            start = i;
        }

        found_len += 1;

        if found_len == len {
            return Some(start);
        }
    }

    None
}

/// Gets the data index for a code point.
fn get_data_index(cp: u32, index: &[u16]) -> u16 {
    let data_offset = cp & (CODEPOINTS_IN_LEAF - 1);

    if cp <= CONTINUOUS_BLOCK_END {
        let leaf_index = PAGES_OFFSET | ((cp >> LEAF_BITS) as u16);
        return index[leaf_index as usize] | data_offset as u16;
    }

    let page_index = cp >> (PAGE_BITS + LEAF_BITS);
    let page_base = index[page_index as usize];
    let leaf_offset = (cp >> LEAF_BITS) & (LEAFS_IN_PAGE - 1);

    index[(page_base + leaf_offset as u16) as usize] | data_offset as u16
}

/// Checks if a block exists in data. Returns the start index if found.
fn contains_block<T: PartialEq>(block: &[T], data: &[T]) -> Option<usize> {
    if data.is_empty() {
        return None;
    }

    for i in 0..(data.len() / block.len() as usize) {
        let existing = &data[i * block.len()..(i + 1) * block.len()];

        if block == existing {
            return Some(i * block.len());
        }
    }

    None
}
