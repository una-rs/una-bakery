use crate::errors::UnknownPropertyValue;

/// General Category (GC).
/// Taken from the UCD: second field of UnicodeData.txt.
///
/// There are 31 possible values, fitting into 5 bits.
/// The variants are ordered to facilitate bitwise operations for checking broader categories.
///
/// Broad categories:
///
///   - LC (Lu, Ll, Lt): cased letters.
///   - L  (Lu, Ll, Lt, Lm, Lo): letters.
///   - M  (Mn, Mc, Me): combining marks.
///   - N  (Nd, Nl, No): numbers and numeric symbols.
///   - P  (Pc, Pd, Ps, Pe, Pi, Pf, Po): punctuation.
///   - S  (Sm, Sc, Sk, So): symbols (mathematical, currency, etc.).
///   - Z  (Zs, Zl, Zp): separators.
///   - C  (Cc, Cf, Cs, Co, Cn): control and other special-purpose code points.
///
/// https://www.unicode.org/reports/tr44/#General_Category
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum GeneralCategory {
    /// Cn – unassigned, reserved, or non-character code point.
    /// Default value when a code point has no explicit entry in the UCD.
    Unassigned = 0, // 0b_0000_0000

    /// Lu – uppercase letter.
    UppercaseLetter = 1, // 0b_0000_0001
    /// Ll – lowercase letter.
    LowercaseLetter = 2, // 0b_0000_0010
    /// Lt – titlecase letter (a digraph encoded as a single character with an initial uppercase part).
    TitlecaseLetter = 3, // 0b_0000_0011

    /// Lm – modifier letter.
    ModifierLetter = 4, // 0b_0000_0100
    /// Lo – other letters, including syllables and ideographs.
    OtherLetter = 5, // 0b_0000_0101

    /// Mn – nonspacing combining mark (zero advance width).
    NonspacingMark = 6, // 0b_0000_0110
    /// Mc – spacing combining mark (positive advance width).
    SpacingMark = 7, // 0b_0000_0111
    /// Me – enclosing combining mark.
    EnclosingMark = 8, // 0b_0000_1000

    /// Nd – decimal digit.
    DecimalNumber = 9, // 0b_0000_1001
    /// Nl – letterlike numeric character.
    LetterNumber = 10, // 0b_0000_1010
    /// No – other numeric characters.
    OtherNumber = 11, // 0b_0000_1011

    /// Zs – space separator.
    SpaceSeparator = 12, // 0b_0000_1100
    /// Zl – line separator.
    LineSeparator = 13, // 0b_0000_1101
    /// Zp – paragraph separator.
    ParagraphSeparator = 14, // 0b_0000_1110

    /// Cc – control code (C0 or C1).
    Control = 16, // 0b_0001_0000
    /// Cf – format control character.
    Format = 17, // 0b_0001_0001
    /// Cs – surrogate code point.
    Surrogate = 18, // 0b_0001_0010
    /// Co – private-use character.
    PrivateUse = 19, // 0b_0001_0011

    /// Pc – connector punctuation (e.g., underscore '_').
    ConnectorPunctuation = 20, // 0b_0001_0100
    /// Pd – dash or hyphen punctuation.
    DashPunctuation = 21, // 0b_0001_0101
    /// Ps – opening punctuation (of a pair).
    OpenPunctuation = 22, // 0b_0001_0110
    /// Pe – closing punctuation (of a pair).
    ClosePunctuation = 23, // 0b_0001_0111
    /// Pi – initial quotation mark.
    InitialPunctuation = 24, // 0b_0001_1000
    /// Pf – final quotation mark.
    FinalPunctuation = 25, // 0b_0001_1001
    /// Po – other punctuation.
    OtherPunctuation = 26, // 0b_0001_1010

    /// Sm – mathematical symbol.
    MathSymbol = 28, // 0b_0001_1100
    /// Sc – currency symbol.
    CurrencySymbol = 29, // 0b_0001_1101
    /// Sk – modifier symbol (non-letterlike).
    ModifierSymbol = 30, // 0b_0001_1110
    /// So – other symbol.
    OtherSymbol = 31, // 0b_0001_1111
}

impl GeneralCategory {
    /// Returns `true` if the category is a cased letter (LC: Lu, Ll, Lt).
    #[inline]
    pub fn is_cased_letter(&self) -> bool {
        !self.is_unassigned() && u8::from(*self) < 4
    }

    /// Returns `true` if the category is a letter (L: Lu, Ll, Lt, Lm, Lo).
    #[inline]
    pub fn is_letter(&self) -> bool {
        !self.is_unassigned() && u8::from(*self) < 6
    }

    /// Returns `true` if the category is a combining mark (M: Mn, Mc, Me).
    #[inline]
    pub fn is_combining_mark(&self) -> bool {
        let value = u8::from(*self);

        value & 0b_1111_1110 == 0b_0000_0110 || value == 0b_0000_1000
    }

    /// Returns `true` if the category is numeric (N: Nd, Nl, No).
    #[inline]
    pub fn is_numeric(&self) -> bool {
        let value = u8::from(*self);

        value & 0b_1111_1100 == 0b_0000_1000 && value != 0b_0000_1000
    }

    /// Returns `true` if the category is a separator (Z: Zs, Zl, Zp).
    #[inline]
    pub fn is_separator(&self) -> bool {
        u8::from(*self) & 0b_1111_1100 == 0b_0000_1100
    }

    /// Returns `true` if the category is a control or special-purpose code point (C: Cc, Cf, Cs, Co, Cn).
    #[inline]
    pub fn is_control(&self) -> bool {
        self.is_unassigned() || u8::from(*self) & 0b_1111_1100 == 0b_0001_0000
    }

    /// Returns `true` if the category is unassigned (Cn).
    #[inline]
    pub fn is_unassigned(&self) -> bool {
        u8::from(*self) == 0
    }

    /// Returns `true` if the category is punctuation (P: Pc, Pd, Ps, Pe, Pi, Pf, Po).
    #[inline]
    pub fn is_punctuation(&self) -> bool {
        let masked = u8::from(*self) & 0b_1111_1100;
        masked == 0b_0001_0100 || masked == 0b_0001_1000
    }

    /// Returns `true` if the category is a symbol (S: Sm, Sc, Sk, So).
    #[inline]
    pub fn is_symbol(&self) -> bool {
        u8::from(*self) & 0b_1111_1100 == 0b_0001_1100
    }
}

impl TryFrom<&str> for GeneralCategory {
    type Error = UnknownPropertyValue;

    #[inline]
    fn try_from(abbr: &str) -> Result<Self, Self::Error> {
        Ok(match abbr {
            "Cn" | "" => Self::Unassigned,
            "Lu" => Self::UppercaseLetter,
            "Ll" => Self::LowercaseLetter,
            "Lt" => Self::TitlecaseLetter,
            "Lm" => Self::ModifierLetter,
            "Lo" => Self::OtherLetter,
            "Mn" => Self::NonspacingMark,
            "Mc" => Self::SpacingMark,
            "Me" => Self::EnclosingMark,
            "Nd" => Self::DecimalNumber,
            "Nl" => Self::LetterNumber,
            "No" => Self::OtherNumber,
            "Zs" => Self::SpaceSeparator,
            "Zl" => Self::LineSeparator,
            "Zp" => Self::ParagraphSeparator,
            "Cc" => Self::Control,
            "Cf" => Self::Format,
            "Cs" => Self::Surrogate,
            "Co" => Self::PrivateUse,
            "Pc" => Self::ConnectorPunctuation,
            "Pd" => Self::DashPunctuation,
            "Ps" => Self::OpenPunctuation,
            "Pe" => Self::ClosePunctuation,
            "Pi" => Self::InitialPunctuation,
            "Pf" => Self::FinalPunctuation,
            "Po" => Self::OtherPunctuation,
            "Sm" => Self::MathSymbol,
            "Sc" => Self::CurrencySymbol,
            "Sk" => Self::ModifierSymbol,
            "So" => Self::OtherSymbol,
            _ => return Err(abbr.to_string()),
        })
    }
}

impl From<GeneralCategory> for u8 {
    #[inline]
    fn from(value: GeneralCategory) -> Self {
        unsafe { *(&value as *const GeneralCategory as *const u8) }
    }
}
