use serde::{Deserialize, Serialize};

use crate::package_order::{TomlSection, TomlSort};

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Hash, Serialize, Deserialize)]
pub struct TomlFormatConfig {
    /// Order sections in the toml document according to the [manifest's][1] order.
    ///
    /// ## Example
    /// ```
    /// [[bin]]
    /// bench = false
    /// name = "cool-tool"
    /// test = false
    ///
    /// [package]
    /// name = "some-crate"
    /// version = "0.0.0"
    /// authors = [""]
    ///
    /// [lib]
    /// bench = false
    /// crate-type = ["cdylib"]
    /// ```
    ///
    /// To:
    ///
    /// ```
    /// [package]
    /// name = "some-crate"
    /// version = "0.0.0"
    /// authors = [""]
    ///
    /// [lib]
    /// bench = false
    /// crate-type = ["cdylib"]
    ///
    /// [[bin]]
    /// bench = false
    /// name = "cool-tool"
    /// test = false
    /// ```
    ///
    /// [1]: https://doc.rust-lang.org/cargo/reference/manifest.html
    pub order_sections: bool,

    /// Overwrites the default manifesto order when [TomlFormatConfig::order_sections] is enabled.
    /// The provided elements will be used sequentially to order the sections.
    /// If there is a section that does not exist in the provided list, it will be appended to the end in the order of the manifesto.
    pub custom_section_order: Vec<String>,

    /// Order dependencies alphabetically.
    pub order_dependencies_alphabetically: bool,

    /// Order the package section items according to the [manifest's][1] order.
    ///
    /// ```
    /// [package]
    /// version = "0.0.0"
    /// rust-version = "1.6.3.0"
    /// authors = [""]
    /// edition = "1"
    /// name = "some-crate"
    /// ```
    /// TO:
    ///
    /// ```
    /// [package]
    /// name = "some-crate"
    /// version = "0.0.0"
    /// authors = [""]
    /// edition = "1"
    /// rust-version = "1.6.3.0"
    /// ```
    /// [1]: https://doc.rust-lang.org/cargo/reference/manifest.html
    pub order_package_section: bool,

    //// Order table keys alphabetically.
    ///
    /// ```
    /// [some-table]
    /// b = "b"
    /// c = "c"
    /// a = "a"
    /// ```
    ///
    /// TO:
    ///
    /// ```
    /// [some-table]
    /// a = "a"
    /// b = "b"
    /// c = "c"
    /// ```
    pub order_table_keys_alphabetically: bool,
    /// Order the section keys, except from package, by group then alphabetically. 
    /// A group is defined by a white space after an table item. Comments are skipped when counting whitespaces. 
    /// 
    ///  /// ```
    /// [some-table] 
    /// c = "b"         # <-- group 1 start
    /// a = "c"
    ///                 # <-- group 2 start
    /// d = "a"
    /// b = "a"
    /// 
    /// # Section 3     # <-- group 3 start
    /// 
    /// f = "a"
    /// e = "a"
    /// ```
    ///
    /// TO:
    /// ```
    /// [some-table]
    /// a = "a"
    /// c = "a"
    /// 
    /// b = "a"
    /// d = "a"
    /// 
    /// # Section 2
    /// 
    /// e = "a"
    /// f = "a"
    /// ```
    pub order_section_keys_by_group_alphabetically: bool,
    /// When [TomlFormatConfig::order_table_keys_alphabetically] is enabled, exclude certain keys from being reordered.
    pub exclude_keys_from_ordering: Vec<String>,
    /// When [TomlFormatConfig::order_table_keys_alphabetically] is enabled, exclude certain tables from their **items** being reordered.
    pub exclude_tables_from_ordering: Vec<String>,

    /// Trims empty spaces around the section names.
    ///
    /// ```
    /// [ package ]
    /// ```
    ///
    /// TO:
    ///
    /// ```
    /// [package]
    /// ```
    pub trim_section_key_names: bool,

    /// Trims empty spaces around to level section items their keys.
    ///
    /// ```
    /// [package]
    /// a = "a"
    ///
    ///     b = "b"
    /// # comment
    ///
    /// c = "c"     
    /// ```
    ///
    /// TO:
    ///
    /// ```
    /// [package]
    /// a = "a"
    /// b = "b"
    /// # comment
    /// c = "c"
    /// ```
    pub trim_section_item_keys: bool,

    /// Trims quotes from table keys.
    ///
    /// ```
    /// [package]
    /// "a" = {"a"="a"}
    /// ```
    /// TO
    /// ```
    /// [package]
    /// a = {a="a"}
    /// ```
    pub trim_quotes_table_keys: bool,

    pub add_newline_after_section: bool,
    /// Formats all tables in the toml document.
    ///
    /// - Preserves order.
    /// - Add spaces between assignments of key and values.
    /// - Formats arrays.
    /// - Removing leading whitespace from values.
    /// - Preserving leading comments after values.
    pub table_formatting: bool,
    pub wrap_array: Option<usize>,
    pub wrap_table: Option<usize>,
}

impl TomlFormatConfig {
    /// Creates a config that will not do any formatting.
    /// Use `default()` to get default formatting configuration.
    pub fn new() -> TomlFormatConfig {
        TomlFormatConfig {
            order_sections: false,
            custom_section_order: vec![],
            order_dependencies_alphabetically: false,
            order_package_section: false,
            order_table_keys_alphabetically: false,
            exclude_keys_from_ordering: vec![],
            exclude_tables_from_ordering: vec![],
            trim_section_key_names: false,
            trim_section_item_keys: false,
            add_newline_after_section: false,
            trim_quotes_table_keys: false,
            table_formatting: false,
            wrap_array: None,
            wrap_table: None,
            order_section_keys_by_group_alphabetically: false,
        }
    }
}

impl Default for TomlFormatConfig {
    /// The default toml config formatting configuration.
    fn default() -> Self {
        Self {
            order_sections: true,
            custom_section_order: TomlSection::manifest_spec(),
            order_dependencies_alphabetically: true,
            order_package_section: true,
            order_table_keys_alphabetically: true,
            exclude_keys_from_ordering: vec![],
            exclude_tables_from_ordering: vec!["package".to_string()],
            trim_section_key_names: true,
            trim_section_item_keys: true,
            trim_quotes_table_keys: true,
            add_newline_after_section: true,
            table_formatting: true,
            wrap_array: Some(50),
            wrap_table: Some(50),
            order_section_keys_by_group_alphabetically: false,
        }
    }
}
