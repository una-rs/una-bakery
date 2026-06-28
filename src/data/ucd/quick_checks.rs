use std::collections::HashMap;

use crate::errors::*;
use crate::normalization::NormType;

const NAME: &str = "DerivedNormalizationProps.txt";

pub struct QuickChecks {
    pub nfd: QCMap,
    pub nfc: QCMap,
    pub nfkd: QCMap,
    pub nfkc: QCMap,
}

pub struct QCMap(HashMap<u32, char>);

impl QCMap {
    pub fn get(&self, code: u32) -> char {
        match self.0.get(&code) {
            Some(&c) => c,
            None => 'Y',
        }
    }
}

/// Quick Checks: DerivedNormalizationProps.txt from the UCD.
///
/// https://www.unicode.org/reports/tr44/#Decompositions_and_Normalization
impl QuickChecks {
    pub fn parse(source: &str) -> Result<Self, ParserError> {
        let mut nfd = HashMap::new();
        let mut nfc = HashMap::new();
        let mut nfkd = HashMap::new();
        let mut nfkc = HashMap::new();

        for (i, line) in source.lines().enumerate() {
            macro_rules! err {
                (InvalidSourceLine) => {
                    ParserError::InvalidSourceLine {
                        source: NAME,
                        line: i + 1,
                    }
                };
                (InvalidPropertyValue, $p:expr, $v:expr) => {
                    ParserError::InvalidPropertyValue {
                        source: NAME,
                        line: i + 1,
                        property: $p,
                        value: $v.to_string(),
                    }
                };
            }

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let [codes, group, value]: [&str; 3] = match line
                .split(';')
                .map(str::trim)
                .collect::<Vec<_>>()
                .try_into()
            {
                Ok(v) => v,
                Err(_) => continue,
            };

            let mark = value
                .trim()
                .chars()
                .next()
                .ok_or_else(|| err!(InvalidSourceLine))?;

            if (["NFD_QC", "NFKD_QC"].contains(&group) && !['Y', 'N'].contains(&mark))
                || (["NFC_QC", "NFKC_QC"].contains(&group) && !['Y', 'N', 'M'].contains(&mark))
            {
                return Err(err!(InvalidPropertyValue, "value", mark));
            }

            let map = match group {
                "NFD_QC" => &mut nfd,
                "NFKD_QC" => &mut nfkd,
                "NFC_QC" => &mut nfc,
                "NFKC_QC" => &mut nfkc,
                _ => continue,
            };

            match codes.split_once("..") {
                Some((from_str, to_str)) => {
                    let from = u32::from_str_radix(from_str, 16)
                        .map_err(|_| err!(InvalidPropertyValue, "code:from", codes))?;

                    let to = u32::from_str_radix(to_str, 16)
                        .map_err(|_| err!(InvalidPropertyValue, "code:to", codes))?;

                    for code in from..=to {
                        map.insert(code, mark);
                    }
                }
                None => {
                    let code = u32::from_str_radix(codes, 16)
                        .map_err(|_| err!(InvalidPropertyValue, "code", codes))?;
                    map.insert(code, mark);
                }
            };
        }

        Ok(Self {
            nfd: QCMap(nfd),
            nfc: QCMap(nfc),
            nfkd: QCMap(nfkd),
            nfkc: QCMap(nfkc),
        })
    }

    pub fn composing(&self, norm_type: NormType) -> &QCMap {
        match norm_type {
            NormType::Canonical => &self.nfc,
            NormType::Compatibility => &self.nfkc,
        }
    }
}
