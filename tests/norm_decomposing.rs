use una_bakery::codepoint::Codepoint;
use una_bakery::normalization::decomposing::normalize;
use una_bakery::normalization::{Decompositions, NormType};

macro_rules! check {
    ($e: ident, $i: expr, $expected: expr, $source: expr, $case: expr, $unicode: expr, $dec: expr) => {
        let got = normalize(&$source, $unicode, $dec);

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
fn ucd_decomposing_tests() {
    let ucd = una_bakery::data::ucd().unwrap();
    let uni = &ucd.unicode;

    let tests = una_bakery::data::normalization_test().unwrap();

    let canonical = Decompositions::generate(&ucd.unicode, NormType::Canonical);
    let compat = Decompositions::generate(&ucd.unicode, NormType::Compatibility);

    for (i, e) in tests.iter().enumerate() {
        // NFD:
        //   c3 == toNFD(c1) == toNFD(c2) == toNFD(c3)
        //   c5 == toNFD(c4) == toNFD(c5)

        check!(e, i, e.c3, e.c1, "c3 == toNFD(c1)", uni, &canonical);
        check!(e, i, e.c3, e.c2, "c3 == toNFD(c2)", uni, &canonical);
        check!(e, i, e.c3, e.c3, "c3 == toNFD(c3)", uni, &canonical);
        check!(e, i, e.c5, e.c4, "c5 == toNFD(c4)", uni, &canonical);
        check!(e, i, e.c5, e.c5, "c5 == toNFD(c5)", uni, &canonical);

        // NFKD:
        //   c5 == toNFKD(c1) == toNFKD(c2) == toNFKD(c3) == toNFKD(c4) == toNFKD(c5)

        check!(e, i, e.c5, e.c1, "c5 = toNFKD(c1)", uni, &compat);
        check!(e, i, e.c5, e.c2, "c5 = toNFKD(c2)", uni, &compat);
        check!(e, i, e.c5, e.c3, "c5 = toNFKD(c3)", uni, &compat);
        check!(e, i, e.c5, e.c4, "c5 = toNFKD(c4)", uni, &compat);
        check!(e, i, e.c5, e.c5, "c5 = toNFKD(c5)", uni, &compat);
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
