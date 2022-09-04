pub mod cargo_toml;
pub mod formatting;
pub mod ordering;
pub mod package_order;
pub mod toml_config;
pub mod utils;

pub use formatting::{
    AppendLineAfterSection, InlineTableWrap, KeyQuoteTrimmer, KeyTrimmer, SectionKeyNameTrimmer,
    TableFormatting, WrapArray,
};
pub use ordering::{
    OrderDependencies, OrderPackageSection, OrderSections, OrderTableKeysAlphabetically,
};

use toml_config::TomlFormatConfig;
use toml_edit::Document;

pub trait TomlFormatter {
    fn visit_document(
        &mut self,
        toml_document: &mut Document,
        _config: &TomlFormatConfig,
    ) -> anyhow::Result<()>;
}
