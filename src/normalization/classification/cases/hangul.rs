use crate::{
    codepoint::Codepoint,
    hangul,
    normalization::{Classifier, DecompositionCase},
};

macro_rules! checks {
    ($($expr: expr),+) => {
        if !($($expr &&)+ true) {
            return None;
        }
    };
}

/// Hangul code point. All Hangul code points are starters.
/// They either decompose (LVT, LV) or combine (L + V + T, LV + T).
pub fn hangul(_: &Classifier, cp: &Codepoint) -> Option<DecompositionCase> {
    // Hangul syllable — decomposes in Form D, corresponds to Form C.
    if hangul::is_syllable(cp.code) {
        return Some(DecompositionCase::HangulSyllable);
    }

    // Leading jamo — already in Form D.
    // Composition into Form C is triggered by the following vowel jamo (V) — DecompositionCase::HangulVowelOrTrailing.
    if hangul::is_leading(cp.code) {
        return Some(DecompositionCase::HangulLeading);
    }

    // Vowel and trailing jamos — already in Form D, combine in Form C with the preceding code point (L for V, LV for T).
    if hangul::is_vowel(cp.code) || hangul::is_trailing(cp.code) {
        return Some(DecompositionCase::HangulVowelOrTrailing);
    }

    None
}

/// Hangul compatibility code points. NFK(C/D) only.
/// Singletons, a starter that decomposes into the combining jamo.
pub fn hangul_compatibility_jamo(cd: &Classifier, cp: &Codepoint) -> Option<DecompositionCase> {
    let dec = cd.decompositions.get(cp.code).as_codepoints(cd.unicode);

    checks!(
        cp.is_starter(),     // Code point is a starter.
        dec.len() == 1,      // Decomposition is a single code point.
        dec[0].is_starter(), // Decomposed code point is a starter.
        hangul::is_hangul_compatibility_jamo(cp) || // Hangul Compatibility Jamo (U+3130–U+318F).
        hangul::is_hangul_halfwidth_and_fullwidth_forms(cp), // Halfwidth and Fullwidth Forms (U+FFA0–U+FFDC).
        hangul::is_hangul(dec[0].code) // Only jamo decompositions.
    );

    let dec = dec[0].clone();

    // Only valid in compatibility decomposition (NFKD/NFKC).

    assert!(cd.norm_type.is_compatibility());

    // Decomposes into a leading/vowel/trailing jamo.

    assert!(hangul::is_leading(dec.code) || hangul::is_V_T(dec.code));

    Some(match hangul::is_leading(dec.code) {
        true => DecompositionCase::HangulCompatibilityIntoLeading { decomposition: dec },
        false => DecompositionCase::HangulCompatibilityIntoVowelOrTrailing { decomposition: dec },
    })
}
