/*
    In this test, we verify that storing the Canonical Combining Class does not require all 8 bits—only 6 bits are sufficient.
*/

const REQUIRED_BITS: u32 = 6;

/// Let's ensure the compressed CCC value fits within 6 bits.
#[test]
pub fn assert_ccc_6bits() {
    let ucd = una_bakery::data::ucd().unwrap();

    let m = ucd.compressed_ccc.max().u8();
    let required_bits = m.ilog2() + 1;

    if REQUIRED_BITS != required_bits {
        panic!(
            "required number of bits to store the compressed CCC value = {}, expected: {}",
            required_bits, REQUIRED_BITS
        )
    }
}
