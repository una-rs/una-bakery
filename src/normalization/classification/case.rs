use std::collections::HashMap;

use crate::codepoint::{CanonicalCombiningClass, Codepoint};

#[derive(Clone)]
pub enum DecompositionCase {
    /// Starter that does not participate in decomposition or composition.
    StarterIgnored,

    /// Starter with no decomposition.
    ///   - May combine with the preceding code point.
    ///   - Does not combine with the subsequent code points.
    StarterCombinesBackwards {
        combines_backwards: HashMap<u32, u32>,
    },

    /// Starter with no decomposition.
    ///   - Does not combine with the preceding code point.
    ///   - May be combined with the subsequent code points.
    StarterCombinesForwards {
        combines_forwards: HashMap<u32, u32>,
    },

    /// Starter with no decomposition.
    ///   - May combine with both preceding and following code points.
    StarterCombinesForwardsBackwards {
        combines_backwards: HashMap<u32, u32>,
        combines_forwards: HashMap<u32, u32>,
    },

    /// Nonstarter that does not combine with the preceding code point.
    Nonstarter { ccc: CanonicalCombiningClass },

    /// Nonstarter that combines with the preceding code point.
    NonstarterCombinesBackwards {
        ccc: CanonicalCombiningClass,
        combines_backwards: HashMap<u32, u32>,
    },

    /// Singleton: starter → starter.
    ///   - Resulting starter combines neither with the preceding nor following code points.
    Singleton { decomposition: Codepoint },

    /// Singleton: starter → starter.
    ///   - Resulting starter does not combine with the preceding code point.
    ///   - Resulting starter may be combined with the following code point.
    SingletonStarterCombinesForwards {
        decomposition: Codepoint,
        combines_forwards: HashMap<u32, u32>,
    },

    /// Pair: starter ←→ starter + nonstarter.
    ///   - May be combined into original code point, if the subsequent code point's CCC >= pair nonstarter's CCC.
    ///   - Does not combine with the preceding code point.
    ///   - Decomposition starter may be combined with the subsequent code points.
    Pair { decomposition: Vec<Codepoint>, starter_combines_forwards: HashMap<u32, u32> },

    /// Pair: starter → .. → starter + nonstarter.
    ///   - Cannot be recombined into the original code point.
    ///   - Does not combine with the preceding code point.
    ///   - Decomposition starter does not combine with any subsequent code point.
    PairNoRecomposition { decomposition: Vec<Codepoint> },

    /// Pair: starter → .. → starter + nonstarter.
    ///   - Cannot be recombined into the original code point.
    ///   - Does not combine with the preceding code point.
    ///   - Decomposition starter may combine with subsequent code points.
    PairNoRecompositionCombinesForwards { decomposition: Vec<Codepoint>, starter_combines_forwards: HashMap<u32, u32> },

    /// Pair: starter → .. → starter + nonstarter → starter.
    ///   - Acts as a singleton in the composing normalization.
    ///   - Does not combine with the preceding code point.
    ///   - Decomposition starter does not combine with subsequent code points.
    PairAsSingleton { decomposition: Vec<Codepoint>, singleton: Codepoint },

    /// Pair: starter → .. → starter + nonstarter → starter.
    ///   - Acts as a singleton in the composing normalization.
    ///   - Does not combine with the preceding code point.
    ///   - Decomposition starter may combine with subsequent code points.
    PairAsSingletonCombinesForwards { decomposition: Vec<Codepoint>, singleton: Codepoint, singleton_combines_forwards: HashMap<u32, u32> },

    /// Pair: starter ←→ starter + nonstarter.
    ///   - Always recombines into the original code point.
    ///   - Does not combine with the preceding code point.
    ///   - Does not combine with any subsequent code point. 
    PairRecombinesIntoOriginal {
        decomposition: Vec<Codepoint>,
    },

    /// Pair: starter ←→ starter + nonstarter.
    ///   - Always recombines into the original code point.
    ///   - Does not combine with the preceding code point,
    ///   - May combine with subsequent code points.
    PairRecombinesIntoOriginalCombinesForwards {
        decomposition: Vec<Codepoint>,
        combines_forwards: HashMap<u32, u32>,
    },

    /// Starter → nonstarter(s).
    StarterToNonstarters { decomposition: Vec<Codepoint> },

    /// Nonstarter → nonstarter(s).
    NonstarterToNonstarters { decomposition: Vec<Codepoint> },

    /// Starter → starter + .. + starter.
    ///   - Does not combine with the preceding code point.
    ///   - The last starter in the precomposition does not combine with subsequent code points.
    EndsWithStarter {
        decomposition: Vec<Codepoint>,
        precomposition: Vec<Codepoint>,
    },

    /// Starter → starter + .. + starter.
    ///   - Does not combine with the preceding code point.
    ///   - The last starter in the precomposition may combine with subsequent code points.
    EndsWithStarterCombinesForwards {
        decomposition: Vec<Codepoint>,
        precomposition: Vec<Codepoint>,
        combines_forwards: HashMap<u32, u32>,
    },

    /// Starter → starter + .. + starter.
    ///   - The first starter in the decomposition may combine with the preceding code point.
    ///   - The last starter in the precomposition does not combine with subsequent code points.
    EndsWithStarterCombinesBackwards {
        decomposition: Vec<Codepoint>,
        combines_backwards: HashMap<u32, u32>,
    },

    /// Starter → starter + .. + starter.
    ///   - The first starter in the decomposition may combine with the preceding code point.
    ///   - The last starter in the precomposition may combine with subsequent code points.
    EndsWithStarterCombinesBoth {
        decomposition: Vec<Codepoint>,
        combines_backwards: HashMap<u32, u32>,
        combines_forwards: HashMap<u32, u32>,
    },

    /// Starter ←→ starter + .. + starter.
    ///   - Recombines into the original code point.
    ///   - Does not combine with any preceding or subsequent code points.
    EndsWithStarterRecombinesIntoOriginal { decomposition: Vec<Codepoint> },

    /// Starter ←→ starter + .. + starter.
    ///   - Recombines into the original code point.
    ///   - The code point does not combine with the preceding code point.
    ///   - May combine with subsequent code points.
    EndsWithStarterRecombinesIntoOriginalCombinesForwards {
        decomposition: Vec<Codepoint>,
        combines_forwards: HashMap<u32, u32>,
    },

    /// Starter → starter + .. + nonstarter.
    ///   - Cannot be recombined into the original code point.
    ///   - Does not combine with the preceding code point.
    ///   - The last starter may combine with subsequent code points.
    EndsWithNonstarter {
        decomposition: Vec<Codepoint>,
        precomposition: Vec<Codepoint>,
    },

    /// Starter → starter + .. + nonstarter.
    ///   - Cannot be recombined into the original code point.
    ///   - Does not combine with the preceding code point.
    ///   - The last starter may combine with subsequent code points.
    EndsWithNonstarterCombinesForwards {
        decomposition: Vec<Codepoint>,
        precomposition: Vec<Codepoint>,
        combines_forwards: HashMap<u32, u32>,
    },

    /// Starter ←→ starter + .. + nonstarter.
    ///   - Does not combine with the preceding code point.
    ///   - Recombines into the original code point.
    EndsWithNonstarterRecombinesIntoOriginal { decomposition: Vec<Codepoint> },

    /// Starter ←→ starter + .. + nonstarter.
    ///   - Does not combine with the preceding code point.
    ///   - Recombines into the original code point.
    ///   - May combine with subsequent code points.
    EndsWithNonstarterRecombinesIntoOriginalCombinesForwards {
        decomposition: Vec<Codepoint>,
        combines_forwards: HashMap<u32, u32>,
    },

    /// Hangul syllable (LVT or LV).
    ///   - Decomposes into L+V or L+V+T.
    HangulSyllable,

    /// Hangul leading jamo (L).
    ///   - No decomposition.
    HangulLeading,

    /// Hangul vowel jamo (V) or trailing jamo (T).
    ///   - No decomposition.
    ///   - Combines with the preceding code point (L for V, LV for T).
    HangulVowelOrTrailing,

    /// Hangul singleton: starter → L jamo.
    ///   - Compatibility form only.
    ///   - Decomposes into the leading jamo (L).
    HangulCompatibilityIntoLeading { decomposition: Codepoint },

    /// Hangul singleton: starter → V/T jamo.
    ///   - Compatibility form only.
    ///   - Decomposes into V/T jamo.
    HangulCompatibilityIntoVowelOrTrailing { decomposition: Codepoint },
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum DecompositionCaseTag {
    StarterIgnored,
    StarterCombinesBackwards,
    StarterCombinesForwards,
    StarterCombinesForwardsBackwards,

    Nonstarter,
    NonstarterCombinesBackwards,

    Singleton,
    SingletonStarterCombinesForwards,

    Pair,
    PairNoRecomposition,
    PairNoRecompositionCombinesForwards,
    PairAsSingleton,
    PairAsSingletonCombinesForwards,
    PairRecombinesIntoOriginal,
    PairRecombinesIntoOriginalCombinesForwards,

    StarterToNonstarters,
    NonstarterToNonstarters,

    EndsWithStarter,
    EndsWithStarterCombinesForwards,
    EndsWithStarterCombinesBackwards,
    EndsWithStarterCombinesBoth,
    EndsWithStarterRecombinesIntoOriginal,
    EndsWithStarterRecombinesIntoOriginalCombinesForwards,

    EndsWithNonstarter,
    EndsWithNonstarterCombinesForwards,
    EndsWithNonstarterRecombinesIntoOriginal,
    EndsWithNonstarterRecombinesIntoOriginalCombinesForwards,

    HangulSyllable,
    HangulLeading,
    HangulVowelOrTrailing,
    HangulCompatibilityIntoLeading,
    HangulCompatibilityIntoVowelOrTrailing,
}

impl DecompositionCase {
     pub fn is_starter_case(&self) -> bool {
        match self {
            DecompositionCase::StarterIgnored
            | DecompositionCase::StarterCombinesBackwards { .. }
            | DecompositionCase::StarterCombinesForwards { .. }
            | DecompositionCase::StarterCombinesForwardsBackwards { .. } => true,
            _ => false,
        }
    }

    pub fn is_pairs_case(&self) -> (bool, &[Codepoint]) {
        match self {
            DecompositionCase::Pair { decomposition, .. }
            | DecompositionCase::PairNoRecomposition { decomposition }
            | DecompositionCase::PairNoRecompositionCombinesForwards { decomposition, .. }
            | DecompositionCase::PairAsSingleton { decomposition, .. } 
            | DecompositionCase::PairAsSingletonCombinesForwards { decomposition, .. }
            | DecompositionCase::PairRecombinesIntoOriginal { decomposition }
            | DecompositionCase::PairRecombinesIntoOriginalCombinesForwards { decomposition, .. } => (true, decomposition),
            _ => (false, &[])
        }
    }

    pub fn is_ends_with_starter_case(&self) -> (bool, &[Codepoint]) {
        match self {
            DecompositionCase::EndsWithStarter { decomposition, .. }
            | DecompositionCase::EndsWithStarterCombinesForwards { decomposition, .. }
            | DecompositionCase::EndsWithStarterCombinesBackwards { decomposition, .. }
            | DecompositionCase::EndsWithStarterCombinesBoth { decomposition, .. }
            | DecompositionCase::EndsWithStarterRecombinesIntoOriginal { decomposition }
            | DecompositionCase::EndsWithStarterRecombinesIntoOriginalCombinesForwards { decomposition, .. } => (true, decomposition),
            _ => (false, &[])
        }
    }

    pub fn is_ends_with_nonstarter_case(&self) -> (bool, &[Codepoint]) {
        match self {
            DecompositionCase::EndsWithNonstarter { decomposition, .. }
            | DecompositionCase::EndsWithNonstarterCombinesForwards { decomposition, .. }
            | DecompositionCase::EndsWithNonstarterRecombinesIntoOriginal { decomposition, .. }
            | DecompositionCase::EndsWithNonstarterRecombinesIntoOriginalCombinesForwards { decomposition, .. } => (true, decomposition),
            _ => (false, &[])
        }
    }

    #[rustfmt::skip]
    pub fn tag(&self) -> DecompositionCaseTag {
        match self {
            DecompositionCase::StarterIgnored                                        => DecompositionCaseTag::StarterIgnored,
            DecompositionCase::StarterCombinesBackwards { .. }                       => DecompositionCaseTag::StarterCombinesBackwards,
            DecompositionCase::StarterCombinesForwards { .. }                        => DecompositionCaseTag::StarterCombinesForwards,
            DecompositionCase::StarterCombinesForwardsBackwards { .. }               => DecompositionCaseTag::StarterCombinesForwardsBackwards,

            DecompositionCase::Nonstarter { .. }                                     => DecompositionCaseTag::Nonstarter,
            DecompositionCase::NonstarterCombinesBackwards { .. }                    => DecompositionCaseTag::NonstarterCombinesBackwards,
            
            DecompositionCase::Singleton { .. }                                      => DecompositionCaseTag::Singleton,
            DecompositionCase::SingletonStarterCombinesForwards { .. }               => DecompositionCaseTag::SingletonStarterCombinesForwards,

            DecompositionCase::Pair { .. }                                           => DecompositionCaseTag::Pair,
            DecompositionCase::PairNoRecomposition { .. }                            => DecompositionCaseTag::PairNoRecomposition,
            DecompositionCase::PairNoRecompositionCombinesForwards { .. }            => DecompositionCaseTag::PairNoRecompositionCombinesForwards,
            DecompositionCase::PairAsSingleton { .. }                                => DecompositionCaseTag::PairAsSingleton,
            DecompositionCase::PairAsSingletonCombinesForwards { .. }                => DecompositionCaseTag::PairAsSingletonCombinesForwards,
            DecompositionCase::PairRecombinesIntoOriginal { .. }                     => DecompositionCaseTag::PairRecombinesIntoOriginal,
            DecompositionCase::PairRecombinesIntoOriginalCombinesForwards { .. }     => DecompositionCaseTag::PairRecombinesIntoOriginalCombinesForwards,

            DecompositionCase::StarterToNonstarters { .. }                           => DecompositionCaseTag::StarterToNonstarters,
            DecompositionCase::NonstarterToNonstarters { .. }                        => DecompositionCaseTag::NonstarterToNonstarters,

            // ---

            DecompositionCase::EndsWithStarter { .. }                                => DecompositionCaseTag::EndsWithStarter,
            DecompositionCase::EndsWithStarterCombinesForwards { .. }                => DecompositionCaseTag::EndsWithStarterCombinesForwards,
            DecompositionCase::EndsWithStarterCombinesBackwards { .. }               => DecompositionCaseTag::EndsWithStarterCombinesBackwards,
            DecompositionCase::EndsWithStarterCombinesBoth { .. }                    => DecompositionCaseTag::EndsWithStarterCombinesBoth,
            DecompositionCase::EndsWithStarterRecombinesIntoOriginal { .. }          => DecompositionCaseTag::EndsWithStarterRecombinesIntoOriginal,
            DecompositionCase::EndsWithStarterRecombinesIntoOriginalCombinesForwards { .. } => DecompositionCaseTag::EndsWithStarterRecombinesIntoOriginalCombinesForwards,

            // ---

            DecompositionCase::EndsWithNonstarter { .. }                             => DecompositionCaseTag::EndsWithNonstarter,
            DecompositionCase::EndsWithNonstarterCombinesForwards { .. }             => DecompositionCaseTag::EndsWithNonstarterCombinesForwards,
            DecompositionCase::EndsWithNonstarterRecombinesIntoOriginal { .. }       => DecompositionCaseTag::EndsWithNonstarterRecombinesIntoOriginal,
            DecompositionCase::EndsWithNonstarterRecombinesIntoOriginalCombinesForwards { .. } => DecompositionCaseTag::EndsWithNonstarterRecombinesIntoOriginalCombinesForwards,

            // ---

            DecompositionCase::HangulSyllable                                        => DecompositionCaseTag::HangulSyllable,
            DecompositionCase::HangulLeading                                         => DecompositionCaseTag::HangulLeading,
            DecompositionCase::HangulVowelOrTrailing                                 => DecompositionCaseTag::HangulVowelOrTrailing,
            DecompositionCase::HangulCompatibilityIntoLeading { .. }                 => DecompositionCaseTag::HangulCompatibilityIntoLeading,
            DecompositionCase::HangulCompatibilityIntoVowelOrTrailing { .. }         => DecompositionCaseTag::HangulCompatibilityIntoVowelOrTrailing,
        }
    }
}
