use std::fmt::Display;

use crate::normalization::DecompositionCaseTag;

#[derive(Clone, Copy)]
pub struct DecompositionCaseMeta {
    pub id: DecompositionCaseTag,
    pub name: &'static str,
    pub filename: &'static str,
    pub stats: StatsMarker,
}

#[derive(Clone, Copy, PartialEq)]
pub enum StatsMarker {
    None,
    Header,
    All,
}

impl DecompositionCaseTag {
    #[rustfmt::skip]
    pub fn table() -> Vec<DecompositionCaseMeta> {
        let mut table: Vec<DecompositionCaseMeta> = vec![];
        let mut i = 0;

        macro_rules! case {
            ($id: ident, $name: expr, $filename: expr, $marker:ident) => {
                table.push(DecompositionCaseMeta {
                    id: {
                        i += 1; 
                        assert_eq!(i - 1, Self::$id as u8);
                        Self::$id
                    }, 
                    name: $name, 
                    filename: $filename, 
                    stats: StatsMarker::$marker,
                })
            };
        }

        case!(StarterIgnored, "Starter — ignored", "1.starters/1.ignored", None);
        case!(StarterCombinesBackwards, "Starter — combines backwards", "1.starters/2.comb_backwards", All);
        case!(StarterCombinesForwards, "Starter — combines forwards", "1.starters/3.comb_forwards", All);
        case!(StarterCombinesForwardsBackwards, "Starter — combines forwards & backwards", "1.starters/4.comb_both", All);

        case!(Nonstarter, "Nonstarter", "2.nonstarters/1.nonstarter", All);
        case!(NonstarterCombinesBackwards, "Nonstarter (combines backwards)", "2.nonstarters/2.comb_backwards", All);

        case!(Singleton, "Singleton (starter → starter)", "3.singletons/1.singleton", All);
        case!(SingletonStarterCombinesForwards, "Singleton (starter → starter); decomposition starter combines forwards", "3.singletons/2.comb_forwards", All);

        case!(Pair, "Pair (starter ←→ starter + nonstarter)", "4.pairs/1.may_be_recomposed", All);
        case!(PairNoRecomposition, "Pair (starter → .. → starter + nonstarter)", "4.pairs/2.no_recomp", All);
        case!(PairNoRecompositionCombinesForwards, "Pair (starter → .. → starter + nonstarter) — starter combines forwards", "4.pairs/3.no_recomp_comb_forwards", All);
        case!(PairAsSingleton, "Pair (starter → .. → starter + nonstarter) → starter", "4.pairs/4.as_singleton", All);
        case!(PairAsSingletonCombinesForwards, "Pair (starter → .. → starter + nonstarter) → starter — combines forwards", "4.pairs/5.as_singleton_comb_forwards", All);
        case!(PairRecombinesIntoOriginal, "Pair (starter ←→ starter + nonstarter)", "4.pairs/6.original", All);
        case!(PairRecombinesIntoOriginalCombinesForwards, "Pair (starter ←→ starter + nonstarter) — combines forwards", "4.pairs/7.original_comb_forwards", All);

        case!(StarterToNonstarters, "Starter → nonstarters", "5.to_nonstarters/1.from_starter", All);
        case!(NonstarterToNonstarters, "Nonstarter → nonstarter(s)", "5.to_nonstarters/2.from_nonstarter", All);

        // ---

        case!(EndsWithStarter, "Starter → starter + .. + starter", "6.st_end/1.st_end", All);
        case!(EndsWithStarterCombinesForwards, "Starter → starter + .. + starter — last precomposition starter combines forwards", "6.st_end/2.comb_forwards", All);
        case!(EndsWithStarterCombinesBackwards, "Starter → starter + .. + starter — first decomposition starter combines backwards", "6.st_end/3.comb_backwards", All);
        case!(EndsWithStarterCombinesBoth, "Starter → starter + .. + starter — combines forwards & backwards", "6.st_end/4.comb_both", All);
        case!(EndsWithStarterRecombinesIntoOriginal, "Starter → starter + .. + starter → original", "6.st_end/5.original", All);
        case!(EndsWithStarterRecombinesIntoOriginalCombinesForwards, "Starter → starter + .. + starter → original — combines forwards", "6.st_end/6.original_comb_forwards", All);

        // ---

        case!(EndsWithNonstarter, "Starter → starter + .. + nonstarter", "7.ns_end/1.ns_end", All);
        case!(EndsWithNonstarterCombinesForwards, "Starter → starter + .. + nonstarter — combines forwards", "7.ns_end/2.comb_forwards", All);
        case!(EndsWithNonstarterRecombinesIntoOriginal, "Starter ←→ starter + .. + nonstarter", "7.ns_end/3.original", All);
        case!(EndsWithNonstarterRecombinesIntoOriginalCombinesForwards, "Starter ←→ starter + .. + nonstarter — combines forwards", "7.ns_end/4.original_comb_forwards", All);

        // ---
        
        case!(HangulSyllable, "Hangul syllable", "8.hangul/1.syllable", All);
        case!(HangulLeading, "Hangul leading", "8.hangul/2.leading", All);
        case!(HangulVowelOrTrailing, "Hangul vowel or trailing", "8.hangul/3.vowel_trailing",  All);
        case!(HangulCompatibilityIntoLeading, "Hangul compatibility codepoint, decomposes into L jamo", "8.hangul/4.compat.leading", All);
        case!(HangulCompatibilityIntoVowelOrTrailing, "Hangul compatibility codepoint, decomposes into V/T jamo", "8.hangul/5.compat.vowel_trailing",  All);

        table
    }
}

impl Display for DecompositionCaseTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}