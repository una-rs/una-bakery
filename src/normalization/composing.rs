use std::collections::HashMap;

use crate::codepoint::Codepoint;
use crate::data::ucd::UnicodeData;
use crate::hangul;
use crate::normalization::decomposing::normalize as decomposing_normalize;
use crate::normalization::{Compositions, Decompositions};

/// Composing normalization for the already decomposed source.
pub fn normalize(
    source: &[u32],
    unicode: &UnicodeData,
    dec_map: &Decompositions,
    compositions: &Compositions,
) -> Vec<Codepoint> {
    let decomposed = decomposing_normalize(source, unicode, dec_map);
    normalize_decomposed(&decomposed, unicode, compositions)
}

// Composing normalization.
pub fn normalize_decomposed(
    source: &[Codepoint],
    unicode: &UnicodeData,
    compositions: &Compositions,
) -> Vec<Codepoint> {
    let mut iter = ComposingIter::from(source);
    let mut result = vec![];

    let mut tail = vec![];
    let mut starter = unicode[0].clone();
    let mut combined = false;

    while iter.next() {
        if !combined {
            starter = match iter.starter() {
                Some(starter) => starter.clone(),
                None => {
                    iter.nonstarters()
                        .iter()
                        .for_each(|c| result.push(c.clone()));
                    continue;
                }
            };
        }

        tail.clear();
        combined = false;
        let mut last_ccc = 0;

        for nonstarter in iter.nonstarters().iter() {
            let forwards = match compositions.forwards(starter.code) {
                Some(map) => map.clone(),
                None => HashMap::new(),
            };

            match forwards.contains_key(&nonstarter.code) && last_ccc != nonstarter.ccc.u8() {
                true => {
                    starter = unicode[forwards[&nonstarter.code]].clone();
                }
                false => {
                    tail.push(nonstarter.clone());
                    last_ccc = nonstarter.ccc.u8();
                }
            }
        }

        if tail.is_empty() {
            if let Some(next) = iter.next_starter() {
                if let Some(code) = hangul::compose_hangul(starter.code, next.code) {
                    starter = unicode[code].clone();
                    combined = true;
                    continue;
                }

                let forwards = match compositions.forwards(starter.code) {
                    Some(map) => map.clone(),
                    None => HashMap::new(),
                };

                if forwards.contains_key(&next.code) {
                    starter = unicode[forwards[&next.code]].clone();
                    combined = true;
                };
            }
        }

        if !combined {
            result.push(starter.clone());
            result.append(&mut tail);
        }
    }

    if combined {
        result.push(starter.clone());
        result.append(&mut tail);
    }

    result
}

struct ComposingIter<'a> {
    source: &'a [Codepoint],
    pos: usize,
    next: usize,
}

impl<'a> ComposingIter<'a> {
    pub fn from(source: &'a [Codepoint]) -> Self {
        Self {
            source,
            pos: 0,
            next: 0,
        }
    }

    pub fn is_finished(&self) -> bool {
        self.pos >= self.source.len()
    }

    pub fn starter(&self) -> Option<&'a Codepoint> {
        match !self.is_finished() {
            true => {
                let cp = &self.source[self.pos];

                match cp.is_starter() {
                    true => Some(cp),
                    false => None,
                }
            }
            false => None,
        }
    }

    pub fn next_starter(&self) -> Option<&'a Codepoint> {
        match self.next < self.source.len() {
            true => {
                let cp = &self.source[self.next];

                match cp.is_starter() {
                    true => Some(cp),
                    false => None,
                }
            }
            false => None,
        }
    }

    pub fn nonstarters(&self) -> &'a [Codepoint] {
        if self.is_finished() {
            return &self.source[0..0];
        }

        let mut from = self.pos;

        if self.source[self.pos].is_starter() {
            from += 1;
        };

        let mut to = from;

        while to < self.source.len() && self.source[to].is_nonstarter() {
            to += 1
        }

        &self.source[from..to]
    }

    pub fn next(&mut self) -> bool {
        self.pos = self.next;

        if self.is_finished() {
            return false;
        }

        self.next = self.pos + self.nonstarters().len();

        if self.source[self.pos].is_starter() {
            self.next += 1;
        };

        return true;
    }
}

/// Minimal CCC from the combining pairs.
pub fn get_min_nonstarter_ccc(forwards: &HashMap<u32, u32>, unicode: &UnicodeData) -> u8 {
    let mut min_ccc = u8::MAX;

    let seconds = forwards
        .keys()
        .map(|&c| &unicode[c])
        .collect::<Vec<&Codepoint>>();

    for second in seconds {
        let ccc = second.ccc.u8();

        if ccc < min_ccc {
            min_ccc = ccc;
        }
    }

    min_ccc
}

/// Composing the starter with code points having minimal CCC.
pub fn compose_starter_with_min_ccc_nonstarters(
    starter: Codepoint,
    nonstarters: &[Codepoint],
    unicode: &UnicodeData,
    compositions: &Compositions,
) -> Vec<Codepoint> {
    let mut last_ccc = 0;
    let mut result = vec![starter.clone()];
    let mut combined = false;

    let mut forwards = get_forwards(starter.code, compositions);

    for nonstarter in nonstarters {
        let min_ccc = get_min_nonstarter_ccc(&forwards, unicode);

        if combined || !forwards.contains_key(&nonstarter.code) || last_ccc == nonstarter.ccc.u8() {
            result.push(nonstarter.clone());
            last_ccc = nonstarter.ccc.u8();

            continue;
        }

        if nonstarter.ccc.u8() != min_ccc {
            combined = true;
            result.push(nonstarter.clone());

            continue;
        }

        result[0] = unicode[forwards[&nonstarter.code]].clone();
        forwards = get_forwards(result[0].code, compositions)
    }

    result
}

fn get_forwards(code: u32, compositions: &Compositions) -> HashMap<u32, u32> {
    match compositions.forwards(code) {
        Some(map) => map.clone(),
        None => HashMap::new(),
    }
}
