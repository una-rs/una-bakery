use std::io::Write as _;
use std::mem::size_of;

use super::*;
use crate::errors::OutputError;
use crate::normalization::BakedDecompositions;
use crate::utils;

const MAX_ALIGN_OVERHEAD: usize = 4096;

/// Data & index tables.
pub fn write_baked_decompositions(
    filename: &str,
    decompositions: &BakedDecompositions,
    name: &str,
    index_link_section: &str,
    data_link_section: &str,
    const_type: &str,
) -> Result<(usize, usize), OutputError> {
    let mut file = utils::create_file(filename)?;

    macro_rules! writefile {
        ($p: expr, $($arg:tt)*) => {
            write!(file, $p, $($arg)*).map_err(|e| OutputError::IoError {
                reason: e.to_string(),
                path: filename.into(),
            })?
        };
    }

    let data = utils::format_numeric_slice(&decompositions.data, LINE_WIDTH, INDENT_LEN, true);
    let index = utils::format_numeric_slice(&decompositions.index, LINE_WIDTH, INDENT_LEN, true);

    let data_len = decompositions.data.len();
    let data_bytesize = data_len * size_of::<u32>();
    let mut data_align = data_bytesize.next_power_of_two();

    loop {
        let chunks = (data_bytesize + data_align - 1) / data_align;
        let gap = (chunks * data_align) - data_bytesize;

        if gap < MAX_ALIGN_OVERHEAD {
            break;
        }

        data_align >>= 1;
    }

    let index_len = decompositions.index.len();
    let index_bytesize = index_len * size_of::<u16>();
    let index_align = index_bytesize.next_power_of_two();

    let index_link_section = match !index_link_section.is_empty() {
        true => format!("#[link_section = \"{index_link_section}\"]\n",),
        false => "".to_owned(),
    };

    let data_link_section = match !data_link_section.is_empty() {
        true => format!("#[link_section = \"{data_link_section}\"]\n",),
        false => "".to_owned(),
    };

    let stats = format!(
        "/*\n    Index size: {index_bytesize} bytes,\n    Data size: {data_bytesize} bytes.\n*/"
    );

    writefile!(
        "\
            {PREFIX_STR}\n\n{stats}\n\n\
            \
            pub const {name}_TYPE: &str = \"{const_type}\";\n\n\
            \
            #[repr(align({index_align}))]\n\
            pub struct AlignedIndex<T>(pub T);\n\n\
            \
            #[repr(align({data_align}))]\n\
            pub struct AlignedData<T>(pub T);\n\n\
            \
            {data_link_section}\
            #[rustfmt::skip]\n\
            #[used]\n\
            \
            pub static {name}_DATA: AlignedData<[u32; {data_len}]> = AlignedData([\n    {data}\n]);\n\n\
            \
            {index_link_section}\
            #[rustfmt::skip]\n\
            #[used]\n\
            \
            pub static {name}_INDEX: AlignedIndex<[u16; {index_len}]> = AlignedIndex([\n    {index}\n]);\n\n\
        ",
    );

    Ok((index_bytesize, data_bytesize))
}
