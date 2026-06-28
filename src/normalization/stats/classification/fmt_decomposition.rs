use super::fmt_codepoint::*;
use crate::codepoint::Codepoint;

pub fn fmt_decomposition(dec: &[Codepoint]) -> String {
    dec.iter()
        .map(fmt_codepoint_short)
        .collect::<Vec<String>>()
        .join(" + ")
}

pub fn fmt_decomposition_u32(dec: &[u32]) -> String {
    dec.iter()
        .map(|&c| format!("`U+{:04X}`", c))
        .collect::<Vec<String>>()
        .join(" + ")
}
