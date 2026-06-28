use crate::codepoint::{Codepoint, CompressedCCCMap};
use crate::normalization::{BakedCompositions, CompositionInfo};

/// Expansion information: 24 bits:
///   - 5 — last starter index,
///   - 5 — decomposition length,
///   - 14 — position.
pub struct ExpansionInfo {
    pub length: u8,
    pub last_starter: u8,
    pub offset: u16,
}

impl ExpansionInfo {
    /// Expansion info data.
    /// Does not store any precomposition info / last starter composition info.
    pub fn from(offset: usize, dec: &[Codepoint]) -> Self {
        let trailing_nonstarters = dec.iter().rev().take_while(|&c| c.is_nonstarter()).count();

        let last_starter = (dec.len() - trailing_nonstarters).saturating_sub(1);

        assert!(dec.len() <= 0x1F);
        assert!(last_starter <= 0x1F);
        assert!(offset <= 0x3FFF);

        Self {
            length: dec.len() as u8,
            last_starter: last_starter as u8,
            offset: offset as u16,
        }
    }

    /// Expansion info → baked value (not shifted).
    pub fn baked(&self) -> u32 {
        self.last_starter as u32 | ((self.length as u32) << 5) | ((self.offset as u32) << 10)
    }

    /// Baked value (not shifted) → expansion info.
    pub fn from_u32(value: u32) -> Self {
        let last_starter = value as u8 & 0x1F;
        let length = (value >> 5) as u8 & 0x1F;
        let offset = (value >> 10) as u16;

        Self {
            length,
            last_starter,
            offset,
        }
    }
}

/// Replaces expansion index in the Marker::Expansion value.
pub fn replace_expansion_index(val: u32, idx: u16) -> u32 {
    assert!(idx < 0x3FFF);

    let mask: u32 = (1 << 18) - 1; // 0x3FFFF, first 18 bits.
    let no_idx_value = val & mask;
    let idx_shifted = (idx as u32) << 18;

    no_idx_value | idx_shifted
}

/// Expansion.
pub fn expansion_vec(dec: &[Codepoint], comp_ccc: &CompressedCCCMap) -> Vec<u32> {
    dec.iter()
        .map(|cp| {
            let ccc_u32 = comp_ccc.get(&cp.ccc).u8() as u32;
            let cp_u32 = cp.code;

            (ccc_u32 | (cp_u32 << 6)) << 8
        })
        .collect::<Vec<u32>>()
}

/// Starter-only expansion.
pub fn starter_expansion_vec(code: u32) -> Vec<u32> {
    vec![code << 14]
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum CompositionCaseMarker {
    Decomposition = 0,
    Precomposition = 1,
    AlwaysRecombines = 2,
    Nonstarters = 3,
}

/// Composition info, 12 bits + 16 bits:
///   - 2 bits — composition case marker.
///   - 5 bits — last starter index,
///   - 5 bits — precomposition length,
///   - 16 bits - composition info.
pub struct ExpansionCompositionInfo {
    pub marker: CompositionCaseMarker,
    pub length: u8,
    pub last_starter: u8,
    pub composition_info: Option<CompositionInfo>,
}

impl ExpansionCompositionInfo {
    /// Expansion composition info.
    pub fn from(
        marker: CompositionCaseMarker,
        dec: &[Codepoint],
        compositions: &BakedCompositions,
    ) -> Self {
        assert!(!dec.is_empty());

        let trailing_nonstarters = dec.iter().rev().take_while(|&c| c.is_nonstarter()).count();
        let last_starter_idx = (dec.len() - trailing_nonstarters).saturating_sub(1);
        let last_starter = &dec[last_starter_idx];

        assert!(last_starter.is_starter());
        assert!(dec.len() <= 0x1F);
        assert!(last_starter_idx <= 0x1F);

        let composition_info = match compositions.index.get(&last_starter.code) {
            Some(composition_info) => Some(composition_info.clone()),
            None => None,
        };

        Self {
            marker,
            length: dec.len() as u8,
            last_starter: last_starter_idx as u8,
            composition_info,
        }
    }

    /// Recomposition / singleton case.
    pub fn from_code(
        marker: CompositionCaseMarker,
        code: u32,
        compositions: &BakedCompositions,
    ) -> Self {
        let composition_info = match compositions.index.get(&code) {
            Some(composition_info) => Some(composition_info.clone()),
            None => None,
        };

        Self {
            marker,
            length: 1,
            last_starter: 0,
            composition_info,
        }
    }

    /// Nonstarters decomposition.
    pub fn nonstarters() -> Self {
        Self {
            marker: CompositionCaseMarker::Nonstarters,
            length: 0,
            last_starter: 0,
            composition_info: None,
        }
    }

    /// Expansion precomposition info → baked value (not shifted).
    pub fn baked(&self) -> u32 {
        let composition_info = match self.composition_info {
            Some(c) => c.baked(),
            None => 0,
        };

        self.marker as u32
            | ((self.last_starter as u32) << 2)
            | ((self.length as u32) << 7)
            | ((composition_info as u32) << 16)
    }

    /// Baked value → expansion precomposition info.
    pub fn from_u32(value: u32) -> Self {
        let marker = match value & 0b_11 {
            0 => CompositionCaseMarker::Decomposition,
            1 => CompositionCaseMarker::Precomposition,
            2 => CompositionCaseMarker::AlwaysRecombines,
            3 => CompositionCaseMarker::Nonstarters,

            _ => unreachable!(),
        };

        let last_starter = ((value >> 2) as u8) & 0x1F;
        let length = (value >> 7) as u8 & 0x1F;
        let composition_info = CompositionInfo::from((value >> 16) as u16);

        Self {
            marker,
            length,
            last_starter,
            composition_info,
        }
    }
}
