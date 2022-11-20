pub mod cargo_toml;
pub mod formatting;
pub mod ordering;
pub mod package_order;
pub mod toml_config;
pub mod utils;
pub mod verify;

pub use formatting::{
    AppendLineAfterSection, InlineTableWrap, KeyQuoteTrimmer, KeyTrimmer, SectionKeyNameTrimmer,
    TableFormatting, WrapArray,
};
pub use ordering::{
    OrderDependencies, OrderPackageSection, OrderSections, OrderTableKeysAlphabetically,
};

use toml_config::TomlFormatConfig;
use toml_edit::{Document, Item, Key, KeyMut, Table};

pub trait TomlFormatter {
    fn visit_document(
        &mut self,
        toml_document: &mut Document,
        _config: &TomlFormatConfig,
    ) -> anyhow::Result<()>;
}

fn iter_sections_as_tables<F: FnMut(&mut KeyMut, &mut Table)>(document: &mut Document, mut cb: F) {
    document
        .iter_mut()
        .for_each(|(mut key, section)| match section {
            Item::None => { /* should not occur */ }
            Item::Value(_) => { /* should not occur */ }
            Item::Table(table) => {
                cb(&mut key, table);
            }
            Item::ArrayOfTables(tables) => {
                for table in tables.iter_mut() {
                    cb(&mut key, table);
                }
            }
        });
}

fn iter_sections_as_items<F: FnMut(&Key, &Item)>(document: &mut Document, mut cb: F) {
    document.iter().for_each(|(key, _section)| {
        let (section_key, section_item) = document.get_key_value(key).unwrap();

        cb(section_key, section_item);
    });
}

fn iter_sections_as_items_mut<F: FnMut(&mut KeyMut, &mut Item)>(
    document: &mut Document,
    mut cb: F,
) {
    document.iter_mut().for_each(|(mut key, section)| {
        cb(&mut key, section);
    });
}
