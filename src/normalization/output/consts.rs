use std::io::Write as _;

use super::*;
use crate::data::UCD;
use crate::errors::OutputError;
use crate::normalization::*;
use crate::utils;

/// Constaints.
pub fn write_consts(filename: &str, ucd: &UCD) -> Result<(), OutputError> {
    let mut file = utils::create_file(filename)?;

    macro_rules! writefile {
        ($p: expr, $($arg:tt)*) => {
            write!(file, $p, $($arg)*).map_err(|e| OutputError::IoError {
                reason: e.to_string(),
                path: filename.into(),
            })?
        };
    }

    let nfd_min = first_nonignorable_codepoint(&ucd.quick_checks.nfd);
    let nfkd_min = first_nonignorable_codepoint(&ucd.quick_checks.nfkd);

    let nfc_min = first_nonignorable_codepoint(&ucd.quick_checks.nfc);
    let nfkc_min = first_nonignorable_codepoint(&ucd.quick_checks.nfkc);

    writefile!("{}\n\n", PREFIX_STR);

    macro_rules! writeconst {
        ($s: expr, $e: ident) => {
            writefile!(
                "\
                    /// U+{:04X} — {}\n\
                    pub const {}_FIRST_CHECK_CODE: u32 = 0x{:04X};\n\n\
                    /// U+{:04X} — {}\n\
                    pub const {}_FIRST_CHECK_BYTE: u8 = 0x{:02X};\n\n\
                ",
                $e.code,
                ucd.unicode[$e.code].name,
                $s,
                $e.code,
                $e.code,
                ucd.unicode[$e.code].name,
                $s,
                $e.first_utf8_byte
            );
        };
    }

    writeconst!("NFD", nfd_min);
    writeconst!("NFKD", nfkd_min);
    writeconst!("NFC", nfc_min);
    writeconst!("NFKC", nfkc_min);

    Ok(())
}
