mod classification;
mod composition;
mod consts;
mod decomposition;
mod output;
mod stats;
mod tables;

pub mod composing;
pub mod decomposing;

pub use classification::*;
pub use composition::*;
pub use consts::*;
pub use decomposition::*;
pub use output::*;
pub use stats::*;
pub use tables::*;

#[derive(Clone, Copy, PartialEq)]
pub enum NormType {
    Canonical,
    Compatibility,
}

impl NormType {
    pub fn is_canonical(&self) -> bool {
        matches!(self, NormType::Canonical)
    }

    pub fn is_compatibility(&self) -> bool {
        matches!(self, NormType::Compatibility)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum TableType {
    DecompositionOnly,
    CompositionOnly,
    Both,
}

impl TableType {
    pub fn is_decomposition_only(&self) -> bool {
        *self == TableType::DecompositionOnly
    }

    pub fn is_composition_only(&self) -> bool {
        *self == TableType::CompositionOnly
    }
}
