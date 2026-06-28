use std::ops::Deref;

use crate::errors::ParserError;

const NAME: &str = "NormalizationTest.txt";

/// NormalizationTest.txt from the UCD.
///
/// https://www.unicode.org/reports/tr41/tr41-36.html#Tests15
pub struct NormalizationTest(pub Vec<NormalizationTestEntry>);

pub struct NormalizationTestEntry {
    pub name: String,
    pub c1: Vec<u32>,
    pub c2: Vec<u32>,
    pub c3: Vec<u32>,
    pub c4: Vec<u32>,
    pub c5: Vec<u32>,
}

impl Deref for NormalizationTest {
    type Target = Vec<NormalizationTestEntry>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl NormalizationTest {
    pub fn parse(source: &str) -> Result<Self, ParserError> {
        let mut result: Vec<NormalizationTestEntry> = vec![];

        for (i, line) in source.lines().enumerate() {
            macro_rules! err {
                () => {
                    ParserError::InvalidSourceLine {
                        source: NAME,
                        line: i + 1,
                    }
                };
            }

            if line.is_empty() || line.starts_with(['#', '@']) {
                continue;
            }

            let (codes, name) = line.split_once("; # ").ok_or_else(|| err!())?;

            let row: Vec<&str> = codes.split(";").collect();

            if row.len() < 5 {
                err!();
            }

            let mut values: Vec<Vec<u32>> = vec![];

            for &col in row[..5].iter() {
                let mut cols: Vec<u32> = vec![];
                let codes: Vec<&str> = col.split_ascii_whitespace().collect();

                for code in codes {
                    cols.push(u32::from_str_radix(code, 16).map_err(|_| err!())?)
                }

                values.push(cols);
            }

            result.push(NormalizationTestEntry {
                name: name.to_owned(),
                c1: values[0].clone(),
                c2: values[1].clone(),
                c3: values[2].clone(),
                c4: values[3].clone(),
                c5: values[4].clone(),
            })
        }

        Ok(Self(result))
    }
}
