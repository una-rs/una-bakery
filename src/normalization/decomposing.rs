use crate::codepoint::Codepoint;
use crate::data::ucd::UnicodeData;
use crate::hangul;
use crate::normalization::Decompositions;

/// Decomposing normalization.
pub fn normalize(
    source: &[u32],
    unicode: &UnicodeData,
    dec_map: &Decompositions,
) -> Vec<Codepoint> {
    let mut codepoints: Vec<Codepoint> = vec![];

    for &cp in source {
        let mut dec = dec_map.get(cp).as_codepoints(&unicode);

        if dec.len() == 0 {
            dec = vec![unicode[cp].clone()];
        }

        for cp in dec {
            match hangul::is_syllable(cp.code) {
                true => hangul::decompose_syllable(cp.code)
                    .iter()
                    .for_each(|&c| codepoints.push(unicode[c].clone())),
                false => codepoints.push(cp),
            }
        }
    }

    let mut i = 0;

    loop {
        if i == codepoints.len() {
            break;
        }

        if codepoints[i].is_starter() {
            i += 1;
            continue;
        }

        let mut to = i;

        for cp in codepoints[i..].iter() {
            if cp.is_starter() {
                break;
            }

            to += 1;
        }

        codepoints[i..to].sort_by(|a, b| a.ccc.u8().cmp(&b.ccc.u8()));

        i = to;
    }

    codepoints
}
