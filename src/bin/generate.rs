use una_bakery::data;
use una_bakery::normalization::*;

macro_rules! unwrap {
    ($e: expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => {
                println!("error: {}", e);
                std::process::exit(-1);
            }
        }
    };
}

fn main() {
    let ucd = unwrap!(data::ucd());

    let canonical_map = unwrap!(
        Classifier::new(
            &ucd.unicode,
            &ucd.composition_exclusions,
            &ucd.quick_checks.nfc,
            NormType::Canonical
        )
        .create_map()
    );

    let compat_map = unwrap!(
        Classifier::new(
            &ucd.unicode,
            &ucd.composition_exclusions,
            &ucd.quick_checks.nfkc,
            NormType::Compatibility
        )
        .create_map()
    );

    let compositions = Compositions::generate(&ucd.unicode, &ucd.composition_exclusions);
    let baked_compositions = BakedCompositions::bake(&compositions);

    // -- Decomposition statistics.

    let canonical_stats = DecompositionStats::collect(&canonical_map, &ucd);
    let compat_stats = DecompositionStats::collect(&compat_map, &ucd);

    unwrap!(canonical_stats.write_index("output/stats/canonical"));
    unwrap!(canonical_stats.write_groups("output/stats/canonical", &ucd, &compositions));

    unwrap!(compat_stats.write_index("output/stats/compatibility"));
    unwrap!(compat_stats.write_groups("output/stats/compatibility", &ucd, &compositions));

    println!("normalization stats: done");

    // -- Compositions table.

    let compositions_size = unwrap!(write_baked_compositions(
        "output/tables/compositions.rs",
        &baked_compositions,
        "una_norm_comp"
    ));

    println!("\ncompositions: {compositions_size} bytes.\n");

    unwrap!(write_consts("output/tables/consts.rs", &ucd));

    println!("normalization consts: done\n");

    // -- Decomposition tables.

    macro_rules! table {
        ($name: expr, $map: ident, $norm_type: expr, $table_type: expr, $output: expr, $ttype: expr, $index_section: expr, $data_section: expr) => {
            let baked = unwrap!(BakedDecompositions::bake(
                &$map,
                &ucd,
                $norm_type,
                $table_type,
            ));

            let (index_size, data_size) = unwrap!(write_baked_decompositions(
                $output,
                &baked,
                $ttype,
                $index_section,
                $data_section,
                $name
            ));

            println!(
                "{}:  index {index_size} bytes, data {data_size} bytes.",
                $name
            );
        };
    }

    table!(
        "NFD",
        canonical_map,
        NormType::Canonical,
        TableType::DecompositionOnly,
        "output/tables/nfd.rs",
        "CANONICAL",
        "una_norm_nfd",
        "una_norm_nfd"
    );

    table!(
        "NFC",
        canonical_map,
        NormType::Canonical,
        TableType::CompositionOnly,
        "output/tables/nfc.rs",
        "CANONICAL",
        "una_norm_nfc",
        "una_norm_nfc"
    );

    table!(
        "NFKD",
        compat_map,
        NormType::Compatibility,
        TableType::DecompositionOnly,
        "output/tables/nfkd.rs",
        "COMPAT",
        "una_norm_nfkd",
        "una_norm_nfkd"
    );

    table!(
        "NFKC",
        compat_map,
        NormType::Compatibility,
        TableType::CompositionOnly,
        "output/tables/nfkc.rs",
        "COMPAT",
        "una_norm_nfkc",
        "una_norm_nfkc"
    );

    table!(
        "NFD+NFC",
        canonical_map,
        NormType::Canonical,
        TableType::Both,
        "output/tables/nfd_nfc.rs",
        "CANONICAL",
        "una_norm_nfd_nfc",
        "una_norm_nfd_nfc"
    );

    table!(
        "NFKD+NFKC",
        compat_map,
        NormType::Compatibility,
        TableType::Both,
        "output/tables/nfkd_nfkc.rs",
        "COMPAT",
        "una_norm_nfkd_nfkc",
        "una_norm_nfkd_nfkc"
    );
}
