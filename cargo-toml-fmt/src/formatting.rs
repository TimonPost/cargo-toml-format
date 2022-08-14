use crate::toml_config::TomlFormatConfig;

mod format;
mod ordering;

pub use format::*;
pub use ordering::*;
use toml_edit::Document;

pub trait TomlFormatter {
    fn visit_document(
        &mut self,
        _toml_document: &mut Document,
        _config: &TomlFormatConfig,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

pub struct MaxLineWidthFmt {
    pub max_line_width: usize,
}

impl MaxLineWidthFmt {}
