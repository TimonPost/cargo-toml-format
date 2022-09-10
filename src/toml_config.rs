use serde::{Deserialize, Serialize};

use crate::package_order::TomlSort;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Hash, Serialize, Deserialize)]
pub struct TomlFormatConfig {
    pub order_sections: bool,
    pub order_dependencies: Option<Vec<TomlSort>>,
    pub order_package_section: bool,
    pub order_table_keys: bool,
    pub trim_section_key_names: bool,
    pub trim_all_keys: bool,
    pub trim_key_quotes: bool,
    pub add_newline_after_section: bool,
    pub table_formatting: bool,
    pub wrap_array: Option<usize>,
    pub wrap_table: Option<usize>,
}

impl TomlFormatConfig {
    pub fn new() -> TomlFormatConfig {
        TomlFormatConfig {
            order_sections: false,
            order_dependencies: None,
            order_package_section: false,
            order_table_keys: false,
            trim_section_key_names: false,
            trim_all_keys: false,
            add_newline_after_section: false,
            trim_key_quotes: false,
            table_formatting: false,
            wrap_array: None,
            wrap_table: None,
        }
    }
}

impl Default for TomlFormatConfig {
    fn default() -> Self {
        Self {
            order_sections: true,
            order_dependencies: Default::default(),
            order_package_section: true,
            order_table_keys: true,
            trim_section_key_names: true,
            trim_all_keys: true,
            trim_key_quotes: true,
            add_newline_after_section: true,
            table_formatting: true,
            wrap_array: Some(50),
            wrap_table: Some(50),
        }
    }
}
