use std::io::Write;

use crate::codepoint::CodepointsBlock;
use crate::errors::OutputError;
use crate::normalization::{DecompositionStats, StatsMarker};
use crate::utils;

const BLOCKS_LEN: usize = 120;

impl DecompositionStats {
    /// Normalization cases index file.
    pub fn write_index(&self, dirpath: &str) -> Result<(), OutputError> {
        let filepath = format!("{}.md", dirpath.to_owned());

        let mut file = utils::create_file(filepath.as_str())?;

        #[rustfmt::skip]
        macro_rules! writefile { ($p: expr, $($arg:tt)*) => {
                write!(file, $p, $($arg)*).map_err(|e| OutputError::IoError { reason: e.to_string(), path: filepath.clone() })?
            };
        }

        for group in self.groups.iter() {
            if group.meta.stats == StatsMarker::None {
                continue;
            }

            writefile!(
                "### {}. {} ({})\n\n",
                group.meta.id,
                group.meta.name,
                group.codepoints.len(),
            );

            if !group.blocks.is_empty() {
                writefile!("{}\n\n", blocks_as_string(&group.blocks, "   ", BLOCKS_LEN));
            }
        }

        Ok(())
    }
}

fn blocks_as_string(blocks: &Vec<CodepointsBlock>, indent: &str, line_len: usize) -> String {
    let mut res = indent.to_owned();
    let indent_len = indent.chars().count();
    let mut cur_len = indent_len;

    for (i, block) in blocks.iter().enumerate() {
        let join = match i == blocks.len() - 1 {
            true => "",
            false => ", ",
        };

        let block_name = format!("`{}`{}", block.name, join);
        let block_name_len = block_name.chars().count();

        if cur_len + block_name_len > line_len {
            res.push_str(format!("\n{}", indent).as_str());
            cur_len = indent_len;
        };

        res.push_str(block_name.as_str());
        cur_len += block_name_len;
    }

    res
}
