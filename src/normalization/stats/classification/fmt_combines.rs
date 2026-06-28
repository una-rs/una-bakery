use std::collections::HashMap;

use super::fmt_codepoint::*;
use crate::codepoint::Codepoint;
use crate::data::UCD;

pub fn fmt_combines_forwards(forwards: &HashMap<u32, u32>, ucd: &UCD) -> String {
    let mut seconds = forwards
        .keys()
        .map(|&c| &ucd.unicode[c])
        .collect::<Vec<&Codepoint>>();

    seconds.sort_by(|&a, &b| {
        let ccc_ordered = a.ccc.u8().cmp(&b.ccc.u8());
        match ccc_ordered {
            std::cmp::Ordering::Equal => a.code.cmp(&b.code),
            _ => ccc_ordered,
        }
    });

    seconds
        .iter()
        .map(|&second| {
            let result = &ucd.unicode[forwards[&second.code]];
            format!(
                "- **code point** + {} → {}",
                fmt_codepoint_short(second),
                fmt_codepoint(result)
            )
        })
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn fmt_combines_backwards(cp: &Codepoint, backwards: &HashMap<u32, u32>, ucd: &UCD) -> String {
    let mut prevs = backwards
        .keys()
        .map(|&c| &ucd.unicode[c])
        .collect::<Vec<&Codepoint>>();

    prevs.sort_by(|&a, &b| a.code.cmp(&b.code));

    prevs
        .iter()
        .map(|&prev| {
            let result = &ucd.unicode[backwards[&prev.code]];
            format!(
                "- {} + **code point** [{}] → {}",
                fmt_codepoint_short(prev),
                cp.ccc.u8(),
                fmt_codepoint(result)
            )
        })
        .collect::<Vec<String>>()
        .join("\n")
}
