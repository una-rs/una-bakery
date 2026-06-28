use crate::codepoint::Codepoint;

pub fn fmt_codepoint(cp: &Codepoint) -> String {
    format!("`U+{:04X}` [{}] {}", cp.code, cp.ccc.u8(), cp.name)
}

pub fn fmt_codepoint_short(cp: &Codepoint) -> String {
    format!("`U+{:04X}` [{}]", cp.code, cp.ccc.u8())
}
