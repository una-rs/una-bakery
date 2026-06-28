use std::io::Write as _;
use std::mem::size_of;

use super::*;
use crate::errors::OutputError;
use crate::normalization::BakedCompositions;
use crate::utils;

/// Composition pairs.
pub fn write_baked_compositions(
    filename: &str,
    compositions: &BakedCompositions,
    link_section: &str,
) -> Result<usize, OutputError> {
    let mut file = utils::create_file(filename)?;

    macro_rules! writefile {
        ($p: expr, $($arg:tt)*) => {
            write!(file, $p, $($arg)*).map_err(|e| OutputError::IoError {
                reason: e.to_string(),
                path: filename.into(),
            })?
        };
    }

    let values = utils::format_numeric_slice(&compositions.table, LINE_WIDTH, INDENT_LEN, true);

    let len = compositions.len();
    let bytesize = len * size_of::<u64>();
    let align = bytesize.next_power_of_two();

    let stats = format!("/*\n    Compositions size: {bytesize} bytes.\n*/");
    let link_section = match !link_section.is_empty() {
        true => format!("#[link_section = \"{link_section}\"]\n",),
        false => "".to_owned(),
    };

    writefile!(
        "\
            {PREFIX_STR}\n\n{stats}\n\n\
            \
            #[repr(align({align}))]\n\
            pub struct Aligned<T>(pub T);\n\n\
            \
            {link_section}\
            #[rustfmt::skip]\n\
            #[used]\n\
            \
            pub static COMPOSITIONS: Aligned<[u64; {len}]> = Aligned([\n    \
            {values}\n\
            ]);\n\
        ",
    );

    Ok(bytesize)
}
