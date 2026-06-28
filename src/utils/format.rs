/// Slice as a text: a table with separators and indents.
pub fn format_numeric_slice<T: Into<u64> + Copy>(
    values: &[T],
    limit: usize,
    indent_len: usize,
    trim_last_sep: bool,
) -> String {
    let mut result = String::new();
    let mut cur_len = 0;

    let indent = " ".repeat(indent_len);

    const SEP_STR: &str = ", ";
    const SEP_LEN: usize = SEP_STR.len();

    for &val in values {
        let strval = match val.into() {
            0 => "0".to_owned(),
            val => format!("0x{:0X}", val),
        };

        if limit != 0 && (cur_len + strval.len() + SEP_LEN > limit) {
            cur_len = indent_len;
            result.push_str(format!("\n{}", indent).as_str());
        }

        cur_len += SEP_LEN + strval.len();
        result.push_str(format!("{}{}", strval, SEP_STR).as_str());
    }

    if trim_last_sep {
        result.truncate(result.len() - SEP_LEN)
    }

    result
}
