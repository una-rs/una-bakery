use una_bakery::codepoint::Codepoint;
use una_bakery::normalization::composing::normalize;
use una_bakery::normalization::{Compositions, Decompositions, NormType};

macro_rules! check {
    ($e: ident, $i: expr, $expected: expr, $source: expr, $case: expr, $unicode: expr, $dec: expr, $comp: expr) => {
        let got = normalize(&$source, $unicode, $dec, $comp);

        assert_eq!(
            $expected,
            got.iter().map(|c| c.code).collect::<Vec<u32>>(),
            "\n\n#{}: {} — {}\n\nexpected: {}\n     got: {}\n    from: {}\n",
            $i + 1,
            $case,
            $e.name,
            format_dec(&$expected),
            format_dec_codepoints(&got),
            format_dec(&$source),
        )
    };
}

#[test]
fn ucd_composing_tests() {
    let ucd = una_bakery::data::ucd().unwrap();
    let uni = &ucd.unicode;

    let tests = una_bakery::data::normalization_test().unwrap();

    let canonical = Decompositions::generate(&ucd.unicode, NormType::Canonical);
    let compat = Decompositions::generate(&ucd.unicode, NormType::Compatibility);

    let comp = Compositions::generate(&ucd.unicode, &ucd.composition_exclusions);

    for (i, e) in tests.iter().enumerate() {
        // NFC
        //   c2 == toNFC(c1) == toNFC(c2) == toNFC(c3)
        //   c4 == toNFC(c4) == toNFC(c5)

        check!(e, i, e.c2, e.c1, "c2 == toNFC(c1)", uni, &canonical, &comp);
        check!(e, i, e.c2, e.c2, "c2 == toNFC(c2)", uni, &canonical, &comp);
        check!(e, i, e.c2, e.c3, "c2 == toNFC(c3)", uni, &canonical, &comp);
        check!(e, i, e.c4, e.c4, "c4 == toNFC(c4)", uni, &canonical, &comp);
        check!(e, i, e.c4, e.c5, "c4 == toNFC(c5)", uni, &canonical, &comp);

        // NFKC
        //   c4 == toNFKC(c1) == toNFKC(c2) == toNFKC(c3) == toNFKC(c4) == toNFKC(c5)

        check!(e, i, e.c4, e.c1, "c4 = toNFKC(c1)", uni, &compat, &comp);
        check!(e, i, e.c4, e.c2, "c4 = toNFKC(c2)", uni, &compat, &comp);
        check!(e, i, e.c4, e.c3, "c4 = toNFKC(c3)", uni, &compat, &comp);
        check!(e, i, e.c4, e.c4, "c4 = toNFKC(c4)", uni, &compat, &comp);
        check!(e, i, e.c4, e.c5, "c4 = toNFKC(c5)", uni, &compat, &comp);
    }
}

fn format_dec(dec: &[u32]) -> String {
    dec.iter()
        .map(|&c| format!("U+{:04X}", c))
        .collect::<Vec<String>>()
        .join(" ")
}

fn format_dec_codepoints(dec: &[Codepoint]) -> String {
    dec.iter()
        .map(|c| format!("U+{:04X} [{}]", c.code, c.ccc.u8()))
        .collect::<Vec<String>>()
        .join(" ")
}
