use std::io::Write;

use super::fmt_codepoint::*;
use super::fmt_combines::*;
use super::fmt_decomposition::*;
use crate::codepoint::Codepoint;
use crate::data::UCD;
use crate::errors::OutputError;
use crate::hangul;
use crate::normalization::*;
use crate::utils;

impl DecompositionStats {
    pub fn write_groups(
        &self,
        dirpath: &str,
        ucd: &UCD,
        compositions: &Compositions,
    ) -> Result<(), OutputError> {
        for group in self.groups.iter() {
            match group.meta.stats {
                StatsMarker::All => self.write_block(dirpath, group, ucd, compositions)?,
                _ => continue,
            }
        }

        Ok(())
    }

    fn write_block(
        &self,
        dirpath: &str,
        group: &DecompositionGroupStats,
        ucd: &UCD,
        compositions: &Compositions,
    ) -> Result<(), OutputError> {
        let filepath = format!("{}/{}.md", dirpath, group.meta.filename);
        let mut file = utils::create_file(filepath.as_str())?;

        #[rustfmt::skip]
        macro_rules! writefile { ($p: expr, $($arg:tt)*) => {
                write!(file, $p, $($arg)*).map_err(|e| OutputError::IoError { reason: e.to_string(), path: filepath.clone() })?
            };
        }

        writefile!("# {} ({})\n\n", group.meta.name, group.codepoints.len());

        for (code, case) in group.codepoints.iter() {
            let cp = &ucd.unicode[*code];
            writefile!(
                "### {} — {}{}\n\n---\n\n",
                fmt_codepoint_short(cp),
                cp.name,
                case_details(cp, case, ucd, compositions)
            );
        }

        Ok(())
    }
}

/// Additional decomposition details.
fn case_details(
    cp: &Codepoint,
    case: &DecompositionCase,
    ucd: &UCD,
    _compositions: &Compositions,
) -> String {
    match case {
        DecompositionCase::StarterIgnored => empty(),

        DecompositionCase::StarterCombinesBackwards { combines_backwards } => {
            format!(
                "\n\n\
                **Combines backwards with:**\n\n\
                {}\
                ",
                fmt_combines_backwards(cp, combines_backwards, ucd)
            )
        }

        DecompositionCase::StarterCombinesForwards { combines_forwards } => {
            format!(
                "\n\n\
                **Combines forwards with:**\n\n\
                {}\
                ",
                fmt_combines_forwards(combines_forwards, ucd)
            )
        }

        DecompositionCase::StarterCombinesForwardsBackwards {
            combines_backwards,
            combines_forwards,
        } => {
            format!(
                "\n\n\
                **Combines backwards with:**\n\n\
                {}\n\n\
                **Combines forwards with:**\n\n\
                {}\
                ",
                fmt_combines_backwards(cp, combines_backwards, ucd),
                fmt_combines_forwards(combines_forwards, ucd)
            )
        }

        DecompositionCase::Nonstarter { .. } => empty(),

        DecompositionCase::NonstarterCombinesBackwards {
            combines_backwards, ..
        } => {
            format!(
                "\n\n\
                **Combines backwards with:**\n\n\
                {}\
                ",
                fmt_combines_backwards(cp, combines_backwards, ucd)
            )
        }

        DecompositionCase::Singleton { decomposition } => {
            format!(
                "\n\n\
                **Decomposition:** → {}\
                ",
                fmt_codepoint(decomposition)
            )
        }

        DecompositionCase::SingletonStarterCombinesForwards {
            decomposition,
            combines_forwards,
        } => {
            format!(
                "\n\n\
                **Decomposition:** → {}\n\n\
                **`U+{:04X}` combines forwards with:**\n\n\
                {}\
                ",
                fmt_codepoint(decomposition),
                decomposition.code,
                fmt_combines_forwards(combines_forwards, ucd)
            )
        }

        DecompositionCase::Pair {
            decomposition,
            starter_combines_forwards,
        } => {
            format!(
                "\n\n\
                **Decomposition:** → {}\n\n\
                **`U+{:04X}` combines forwards with:**\n\n\
                {}\
                ",
                fmt_decomposition(decomposition),
                decomposition[0].code,
                fmt_combines_forwards(starter_combines_forwards, ucd)
            )
        }

        DecompositionCase::PairNoRecomposition { decomposition } => {
            format!(
                "\n\n\
                **Decomposition:** → {}\
                ",
                fmt_decomposition(decomposition),
            )
        }

        DecompositionCase::PairNoRecompositionCombinesForwards {
            decomposition,
            starter_combines_forwards,
        } => {
            format!(
                "\n\n\
                **Decomposition:** → {}\n\n\
                **`U+{:04X}` combines forwards:**\n\n\
                {}\
                ",
                fmt_decomposition(decomposition),
                decomposition[0].code,
                fmt_combines_forwards(starter_combines_forwards, ucd),
            )
        }

        DecompositionCase::PairAsSingleton {
            decomposition,
            singleton,
        } => {
            format!(
                "\n\n\
                **Decomposition:** → {} → {}\
                ",
                fmt_decomposition(decomposition),
                fmt_codepoint_short(singleton),
            )
        }

        DecompositionCase::PairAsSingletonCombinesForwards {
            decomposition,
            singleton,
            singleton_combines_forwards,
        } => {
            format!(
                "\n\n\
                **Decomposition:** → {} → {}\n\n\
                **{} combines forwards:**\n\n\
                {}\
                ",
                fmt_decomposition(decomposition),
                fmt_codepoint_short(singleton),
                fmt_codepoint_short(singleton),
                fmt_combines_forwards(singleton_combines_forwards, ucd),
            )
        }

        DecompositionCase::PairRecombinesIntoOriginal { decomposition } => {
            format!(
                "\n\n\
                **Decomposition:** → {} → **original code point**\
                ",
                fmt_decomposition(decomposition),
            )
        }

        DecompositionCase::PairRecombinesIntoOriginalCombinesForwards {
            decomposition,
            combines_forwards,
        } => {
            format!(
                "\n\n\
                **Decomposition:** → {}\n\n\
                **Original code point combines forwards:**\n\n\
                {}\
                ",
                fmt_decomposition(decomposition),
                fmt_combines_forwards(combines_forwards, ucd),
            )
        }

        //
        DecompositionCase::StarterToNonstarters { decomposition }
        | DecompositionCase::NonstarterToNonstarters { decomposition } => {
            format!(
                "\n\n\
                **Decomposition:** → {}\
                ",
                fmt_decomposition(decomposition),
            )
        }

        DecompositionCase::EndsWithStarter {
            decomposition,
            precomposition,
        } => {
            format!(
                "\n\n\
                **Decomposition:** → {}\
                {}\
                ",
                fmt_decomposition(decomposition),
                precomposition_str(decomposition, precomposition),
            )
        }

        DecompositionCase::EndsWithStarterCombinesBackwards {
            decomposition,
            combines_backwards,
        } => {
            format!(
                "\n\n\
                **Decomposition:** → {}\n\n\
                **`U+{:04X}` combines backwards with:**\n\n\
                {}\
                ",
                fmt_decomposition(decomposition),
                decomposition.first().unwrap().code,
                fmt_combines_backwards(&decomposition.first().unwrap(), combines_backwards, ucd)
            )
        }

        DecompositionCase::EndsWithStarterCombinesForwards {
            decomposition,
            precomposition,
            combines_forwards,
        } => {
            format!(
                "\n\n\
                **Decomposition:** → {}\
                {}\n\n\
                **`U+{:04X}` combines forwards with:**\n\n\
                {}\
                ",
                fmt_decomposition(decomposition),
                precomposition_str(decomposition, precomposition),
                decomposition.last().unwrap().code,
                fmt_combines_forwards(combines_forwards, ucd)
            )
        }

        DecompositionCase::EndsWithStarterCombinesBoth {
            decomposition,
            combines_forwards,
            combines_backwards,
        } => {
            format!(
                "\n\n\
                **Decomposition:** → {}\n\n\
                **`U+{:04X}` combines backwards with:**\n\n\
                {}\n\n\
                **`U+{:04X}` combines forwards with:**\n\n\
                {}\
                ",
                fmt_decomposition(decomposition),
                decomposition.first().unwrap().code,
                fmt_combines_backwards(cp, combines_backwards, ucd),
                decomposition.last().unwrap().code,
                fmt_combines_forwards(combines_forwards, ucd)
            )
        }

        DecompositionCase::EndsWithStarterRecombinesIntoOriginal { decomposition } => {
            format!(
                "\n\n\
                **Decomposition:** → {} → **original code point**\
                ",
                fmt_decomposition(decomposition)
            )
        }

        DecompositionCase::EndsWithStarterRecombinesIntoOriginalCombinesForwards {
            decomposition,
            combines_forwards,
        } => {
            format!(
                "\n\n\
                **Decomposition:** → {} → **original code point**\n\n\
                **Combines forwards with:**\n\n\
                {}\
                ",
                fmt_decomposition(decomposition),
                fmt_combines_forwards(combines_forwards, ucd)
            )
        }

        DecompositionCase::EndsWithNonstarter {
            decomposition,
            precomposition,
        } => {
            format!(
                "\n\n\
                **Decomposition:** → {}\
                {}\
                ",
                fmt_decomposition(decomposition),
                precomposition_str(decomposition, precomposition),
            )
        }

        DecompositionCase::EndsWithNonstarterCombinesForwards {
            decomposition,
            precomposition,
            combines_forwards,
        } => {
            format!(
                "\n\n\
                **Decomposition:** → {}\
                {}\n\n\
                **Precomposition last starter combines forwards:**\n\n\
                {}\
                ",
                fmt_decomposition(decomposition),
                precomposition_str(decomposition, precomposition),
                fmt_combines_forwards(combines_forwards, ucd)
            )
        }

        DecompositionCase::EndsWithNonstarterRecombinesIntoOriginal { decomposition } => {
            format!(
                "\n\n\
                **Decomposition:** → {} → **original code point**\
                ",
                fmt_decomposition(decomposition),
            )
        }

        DecompositionCase::EndsWithNonstarterRecombinesIntoOriginalCombinesForwards {
            decomposition,
            combines_forwards,
        } => {
            format!(
                "\n\n\
                **Decomposition:** → {} → **original code point**\n\n\
                **Original code point combines forwards:**\n\n\
                {}\
                ",
                fmt_decomposition(decomposition),
                fmt_combines_forwards(combines_forwards, ucd)
            )
        }

        DecompositionCase::HangulSyllable => {
            let decomposition = &hangul::decompose_syllable(cp.code);

            let dec_type = match decomposition.len() {
                2 => "L/V",
                3 => "L/V/T",
                _ => unreachable!(),
            };

            format!(
                "\n\n\
                **Decomposition ({dec_type}):** → {}\
                ",
                fmt_decomposition_u32(decomposition),
            )
        }

        DecompositionCase::HangulLeading | DecompositionCase::HangulVowelOrTrailing => empty(),

        DecompositionCase::HangulCompatibilityIntoLeading { decomposition } => {
            format!(
                "\n\n\
                **Decomposition**: → {} (L jamo)\
                ",
                fmt_codepoint(decomposition)
            )
        }

        DecompositionCase::HangulCompatibilityIntoVowelOrTrailing { decomposition } => {
            let jamo_type = match hangul::is_vowel(decomposition.code) {
                true => 'V',
                false => {
                    assert!(hangul::is_trailing(decomposition.code));
                    'T'
                }
            };

            format!(
                "\n\n\
                **Decomposition**: → {} ({} jamo)\
                ",
                fmt_codepoint(decomposition),
                jamo_type
            )
        }
    }
}

fn empty() -> String {
    String::new()
}

fn precomposition_str(decomposition: &[Codepoint], precomposition: &[Codepoint]) -> String {
    match decomposition != precomposition {
        true => {
            format!(
                "\n\n\
                **Precomposition:** → {}\
                ",
                fmt_decomposition(precomposition)
            )
        }
        false => empty(),
    }
}
