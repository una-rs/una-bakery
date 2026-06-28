use crate::codepoint::Codepoint;

/// Start of the Hangul syllables block.
pub const HANGUL_S_BASE: u32 = 0xAC00;
/// Start of the leading consonant (L) jamo block.
pub const HANGUL_L_BASE: u32 = 0x1100;
/// Start of the vowel (V) jamo block.
pub const HANGUL_V_BASE: u32 = 0x1161;
/// Start of the trailing consonant (T) jamo block.
pub const HANGUL_T_BASE: u32 = 0x11A8;

/// Total number of Hangul syllables in Unicode.
pub const HANGUL_S_COUNT: u32 = 11172;
/// Number of leading consonants (L).
pub const HANGUL_L_COUNT: u32 = 19;
/// Number of vowels (V).
pub const HANGUL_V_COUNT: u32 = 21;
/// Number of trailing consonants (T).
pub const HANGUL_T_COUNT: u32 = 27;

/// Number of code points per LV block (T_COUNT + 1, including no-trail case).
pub const HANGUL_T_BLOCK_SIZE: u32 = HANGUL_T_COUNT + 1;
/// HANGUL_V_COUNT × HANGUL_T_BLOCK_SIZE (= 21 × 28 = 588).
pub const HANGUL_N_COUNT: u32 = HANGUL_V_COUNT * HANGUL_T_BLOCK_SIZE;

/// Hangul syllable into L, V, T.
pub fn syllable_to_lvt(code: u32) -> Option<(u8, u8, Option<u8>)> {
    if !is_syllable(code) {
        return None;
    }

    let s = code - HANGUL_S_BASE;
    let l = (s / HANGUL_N_COUNT) as u8;
    let v = ((s % HANGUL_N_COUNT) / HANGUL_T_BLOCK_SIZE) as u8;
    let t_raw = s % HANGUL_T_BLOCK_SIZE;

    let t = match t_raw != 0 {
        true => Some((t_raw - 1) as u8),
        false => None,
    };

    Some((l, v, t))
}

/// Compose Hangul code points: L + V → LV, or LV + T → LVT.
pub fn compose_hangul(first: u32, second: u32) -> Option<u32> {
    // L + V: first is a leading jamo, second is a vowel jamo.
    let l = first.wrapping_sub(HANGUL_L_BASE);
    let v = second.wrapping_sub(HANGUL_V_BASE);

    if l < HANGUL_L_COUNT && v < HANGUL_V_COUNT {
        return Some(HANGUL_S_BASE + l * HANGUL_N_COUNT + v * HANGUL_T_BLOCK_SIZE);
    }

    // LV + T: first is an LV syllable, second is a trailing jamo.
    let lv = first.wrapping_sub(HANGUL_S_BASE);
    let t = second.wrapping_sub(HANGUL_T_BASE);

    if lv < HANGUL_S_COUNT && lv % HANGUL_T_BLOCK_SIZE == 0 && t < HANGUL_T_COUNT {
        return Some(first + t + 1);
    }

    None
}

// Hangul syllable or jamo.
pub fn is_hangul(code: u32) -> bool {
    is_syllable(code) || is_L_LV(code) || is_V_T(code)
}

// Returns true if the code point is a Hangul syllable (LVT / LV).
pub fn is_syllable(code: u32) -> bool {
    code.wrapping_sub(HANGUL_S_BASE) < HANGUL_S_COUNT
}

/// Decomposes hangul syllable.
pub fn decompose_syllable(code: u32) -> Vec<u32> {
    let mut result = vec![];

    match syllable_to_lvt(code) {
        Some((l, v, t)) => {
            result.push(HANGUL_L_BASE + l as u32);
            result.push(HANGUL_V_BASE + v as u32);

            if let Some(t) = t {
                result.push(HANGUL_T_BASE + t as u32);
            }
        }
        None => (),
    }

    result
}

// Returns true if the code point is an LV syllable (i.e., has no trailing jamo).
pub fn is_lv_syllable(code: u32) -> bool {
    let lv = code.wrapping_sub(HANGUL_S_BASE);
    lv < HANGUL_S_COUNT && lv % HANGUL_T_BLOCK_SIZE == 0
}

// Returns true if the code point is a leading consonant jamo (L).
pub fn is_leading(code: u32) -> bool {
    code.wrapping_sub(HANGUL_L_BASE) < HANGUL_L_COUNT
}

// Returns true if the code point is a vowel jamo (V).
pub fn is_vowel(code: u32) -> bool {
    code.wrapping_sub(HANGUL_V_BASE) < HANGUL_V_COUNT
}

// Returns true if the code point is a trailing consonant jamo (T).
pub fn is_trailing(code: u32) -> bool {
    code.wrapping_sub(HANGUL_T_BASE) < HANGUL_T_COUNT
}

// Returns true if the code point can serve as the left operand in Hangul composition (L or LV).
#[allow(non_snake_case)]
pub fn is_L_LV(code: u32) -> bool {
    is_leading(code) || is_lv_syllable(code)
}

// Returns true if the code point can serve as the right operand in Hangul composition (V or T).
#[allow(non_snake_case)]
pub fn is_V_T(code: u32) -> bool {
    is_vowel(code) || is_trailing(code)
}

pub fn is_hangul_compatibility_jamo(cp: &Codepoint) -> bool {
    if let Some(block) = &cp.block {
        return block.name == "Hangul Compatibility Jamo";
    };

    false
}

pub fn is_hangul_halfwidth_and_fullwidth_forms(cp: &Codepoint) -> bool {
    if let Some(block) = &cp.block {
        if block.name != "Halfwidth and Fullwidth Forms" {
            return false;
        }

        return cp.name.contains("HANGUL");
    };

    false
}
