use crate::data::ucd::QCMap;
use crate::normalization::LAST_DECOMPOSITION_CODEPOINT;

#[derive(Clone, Copy, Debug)]
pub struct FirstNonignorable {
    pub code: u32,
    pub first_utf8_byte: u8,
}

pub fn first_nonignorable_codepoint(qc: &QCMap) -> FirstNonignorable {
    let mut code = 0;

    while code <= LAST_DECOMPOSITION_CODEPOINT {
        if qc.get(code) != 'Y' {
            break;
        }

        code += 1;
    }

    let char = char::from_u32(code).unwrap();

    let mut buf = [0u8; 4];
    let s = char.encode_utf8(&mut buf);

    FirstNonignorable {
        code,
        first_utf8_byte: s.as_bytes()[0],
    }
}
