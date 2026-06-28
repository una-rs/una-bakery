/*
    These checks are entirely optional, but they help clarify and verify certain properties of Unicode data,
    as some details may be non-obvious or only briefly mentioned (or omitted) in the official documentation.
*/

const LAST_CODEPOINT_WITH_DECOMPOSITION: u32 = 0x2FA1D;
const LAST_CODEPOINT_IN_DECOMPOSITION: u32 = 0x2A600;

/// This check confirms that if a code point is not listed in the `UnicodeData.txt` table,
/// it will never appear in the decomposition of other code points.
#[test]
pub fn assert_missing_codepoint_decomposition_rule() {
    let unicode = una_bakery::data::ucd().unwrap().unicode;

    for codepoint in unicode.values() {
        for &dec_code in codepoint.decomposition.iter() {
            if unicode.get(&dec_code).is_none() {
                panic!(
                    "not mentioned in UnicodeData.txt U+{:X} found in U+{:X} decomposition",
                    dec_code, codepoint.code
                )
            }
        }
    }
}

/// We already know from the standard that the highest codepoint with a decomposition is `U+2FA1D`. Let's verify this.
/// Also, lets check the last codepoint which can be mentioned in a decomposition (`U+2A600`).
#[test]
pub fn assert_last_decomposition_codepoint() {
    let unicode = una_bakery::data::ucd().unwrap().unicode;

    let mut keys: Vec<_> = unicode.keys().collect();
    keys.sort_unstable_by(|a, b| b.cmp(a));

    let mut last_with: u32 = 0;
    let mut last_in: u32 = 0;

    for &key in keys {
        let codepoint = &unicode[key];
        if codepoint.decomposition.len() != 0 {
            last_with = codepoint.code;

            codepoint
                .decomposition
                .iter()
                .for_each(|&c| last_in = core::cmp::max(last_in, c));

            break;
        }
    }

    if last_with != LAST_CODEPOINT_WITH_DECOMPOSITION {
        panic!(
            "last codepoint having decomposition: expected U+{:X}, found U+{:X}",
            LAST_CODEPOINT_WITH_DECOMPOSITION, last_with
        )
    }

    if last_in != LAST_CODEPOINT_IN_DECOMPOSITION {
        panic!(
            "last codepoint included in decomposition: expected U+{:X}, found U+{:X}",
            LAST_CODEPOINT_IN_DECOMPOSITION, last_in
        )
    }
}

/// It's important to understand that decompositions listed in UnicodeData.txt can be further "unfolded", even for NFD.
#[test]
pub fn assert_unicode_nfd_expandable() {
    let unicode = una_bakery::data::ucd().unwrap().unicode;

    let mut expands = false;

    'parent: for codepoint in unicode.values() {
        if codepoint.decomposition_tag.is_some() || codepoint.decomposition.is_empty() {
            continue;
        }

        for &dec_code in codepoint.decomposition.iter() {
            // We can safely unwrap here because we know that every code point appearing in any decomposition has an entry in the table.
            let dec_codepoint = &unicode[dec_code];

            // Any code point appearing in a decomposition has its own NFD decomposition.
            if dec_codepoint.decomposition_tag.is_none() && !dec_codepoint.decomposition.is_empty()
            {
                expands = true;
                break 'parent;
            }
        }
    }

    if !expands {
        panic!("UnicodeData.txt does not contain expandable NFD decompositions")
    }
}
