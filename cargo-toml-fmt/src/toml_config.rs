use serde::{Deserialize, Serialize};

use crate::package_order::TomlSort;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Hash, Serialize, Deserialize)]
pub struct TomlFormatConfig {
    pub order_sections: bool,
    pub dependency_sorts: Option<Vec<TomlSort>>,
    pub order_package_section: bool,
    pub order_table_keys: bool,
    pub trim_section_keys: bool,
    pub trim_keys: bool,
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
            dependency_sorts: None,
            order_package_section: false,
            order_table_keys: false,
            trim_section_keys: false,
            trim_keys: false,
            add_newline_after_section: false,
            trim_key_quotes: false,
            table_formatting: false,
            wrap_array: None,
            wrap_table: None,
        }
    }
}
