use toml_edit::{Array, Decor, Document, Item, Key, KeyMut, Table, TableLike, Value};

use crate::{iter_sections_as_tables, toml_config::TomlFormatConfig};

use super::TomlFormatter;

/// Trims empty spaces around the section names.
///
/// For example [ package ] -> [package].
pub struct SectionKeyNameTrimmer;

impl TomlFormatter for SectionKeyNameTrimmer {
    fn visit_document(
        &mut self,
        toml_document: &mut Document,
        _config: &TomlFormatConfig,
    ) -> anyhow::Result<()> {
        iter_sections_as_tables(toml_document, |section_key, _| {
            trim_decor_blank_lines(section_key.decor_mut());
        });
        Ok(())
    }
}

pub struct KeyTrimmer;

impl TomlFormatter for KeyTrimmer {
    fn visit_document(
        &mut self,
        toml_document: &mut Document,
        _config: &TomlFormatConfig,
    ) -> anyhow::Result<()> {
        iter_sections_as_tables(toml_document, |_, section| {
            trim_decor_blank_lines(section.decor_mut());

            section.iter_mut().for_each(|(mut key, _)| {
                trim_decor_blank_lines(key.decor_mut());
            });
        });
        Ok(())
    }
}

pub struct KeyQuoteTrimmer;

impl TomlFormatter for KeyQuoteTrimmer {
    fn visit_document(
        &mut self,
        toml_document: &mut Document,
        _config: &TomlFormatConfig,
    ) -> anyhow::Result<()> {
        iter_sections_as_tables(toml_document, |_, section| {
            Self::visit_table(section);
        });
        Ok(())
    }
}

impl KeyQuoteTrimmer {
    fn visit_table(table: &mut Table) {
        let mut trimmed_keys = vec![];

        // Iterate table keys and trim away quotes.
        table.iter_mut().for_each(|(key, item)| {
            let trimmed_key = Self::remove_quotes(&key);

            trimmed_keys.push((trimmed_key, item.clone()));
        });

        // Iterate trimmed keys and remove them and then insert them back into the table.
        trimmed_keys
            .into_iter()
            .for_each(|(trimmed_key, mut item)| {
                Self::visit_item(&mut item);

                table.remove(trimmed_key.get());
                table.insert(trimmed_key.get(), item);

                // Unfortunately I can't figure out how to internally update the string representation of the key,
                // hence we have to do it this way.
                table.iter_mut().for_each(|(mut key, _)| {
                    Self::trim_key(&mut key, &trimmed_key);
                });
            });
    }

    // Recursively iterate items and trim quotes from key names 'e.g' "key" = value -> key = value.
    fn visit_item(item: &mut Item) {
        match item {
            Item::Value(value) => {
                Self::visit_value(value);
            }
            Item::Table(table) => Self::visit_table(table),
            Item::ArrayOfTables(tables) => {
                tables.iter_mut().for_each(|table| Self::visit_table(table))
            }
            Item::None => {}
        };
    }

    fn visit_value(value: &mut Value) {
        if let Value::InlineTable(inline_table) = value {
            let mut trimmed_key_value = None;
            inline_table.iter_mut().for_each(|(key, inline_value)| {
                let trimmed_key = Self::remove_quotes(&key);

                trimmed_key_value = Some((trimmed_key, inline_value.clone()));
            });

            if let Some((trimmed_key, ref mut value)) = trimmed_key_value {
                Self::visit_value(value);

                inline_table.remove(trimmed_key.get());
                inline_table.insert(trimmed_key.get(), value.clone());

                inline_table.iter_mut().for_each(|(mut key, _)| {
                    Self::trim_key(&mut key, &trimmed_key);
                });
            }
        }
    }

    fn trim_key(original_key: &mut KeyMut, trimmed_key: &Key) {
        original_key
            .decor_mut()
            .set_prefix(trimmed_key.decor().prefix().unwrap_or("").to_string());
        original_key
            .decor_mut()
            .set_suffix(trimmed_key.decor().suffix().unwrap_or("").to_string());
    }

    fn remove_quotes(key: &KeyMut) -> Key {
        let raw_key = key.to_repr().as_raw().to_string();
        let trimmed = raw_key.trim_matches('"');
        Key::new(trimmed.to_string()).with_decor(key.decor().clone())
    }
}

fn trim_decor_blank_lines(decor: &mut Decor) {
    let prefix = decor.prefix().unwrap_or("").to_owned();
    let suffix = decor.suffix().unwrap_or("").to_owned();
    decor.set_prefix(trim_blank_lines(prefix.as_str()));
    decor.set_suffix(trim_blank_lines(suffix.as_str()));
}

/// trim blank lines at the beginning and end
fn trim_blank_lines(s: &str) -> String {
    return format!("{}", s.trim());
}

/// Formats tables and inline tables.
///
/// - Preserves order.
/// - Add spaces between assignments of key and values.
/// - Removing leading whitespace from values.
/// - Preserving leading comments after values.
pub struct TableFormatting;

impl TomlFormatter for TableFormatting {
    fn visit_document(
        &mut self,
        toml_document: &mut Document,
        _config: &TomlFormatConfig,
    ) -> anyhow::Result<()> {
        iter_sections_as_tables(toml_document, |section_key, section| {
            // Remove spaces from section key [ section ] -> [section]
            section_key.fmt();

            section.decor_mut().set_suffix("");

            // Recursively iterate table key values and format them.
            self.fmt_table(section, 0);

            let prefix = Self::fmt_prefix_and_preserve_comments(
                section.decor().prefix().unwrap_or_default(),
            );
            section.decor_mut().set_prefix(prefix);
        });
        Ok(())
    }
}

impl TableFormatting {
    /// Visit the item and format its contained type.
    fn visit_item(&self, key: &mut KeyMut, item: &mut Item, depth: usize) {
        let trimmed_prefix =
            Self::fmt_prefix_and_preserve_comments(key.decor_mut().prefix().unwrap());
        key.decor_mut().set_prefix(trimmed_prefix);

        match item {
            Item::Value(value) => {
                self.fmt_value(value);
                key.decor_mut().set_suffix(" ");
            }
            Item::Table(table) => {
                self.fmt_table(table, 0);
                key.decor_mut().set_suffix("");
            }
            Item::ArrayOfTables(tables) => {
                for table in tables.iter_mut() {
                    self.fmt_table(table, depth);
                    key.decor_mut().set_suffix("");
                }
            }
            Item::None => {}
        }
    }

    /// Iterate the value and recursively format its contained types.
    fn fmt_value(&self, value: &mut Value) {
        // Fetch comment if there is anyone after the value e.g ("key" = "value" # comment) will return '# comment'.
        let (prefix_comment, suffix_comment) = Self::get_comments(value.decor());

        match value {
            Value::Array(array) => {
                self.fmt_array(array);
            }
            Value::InlineTable(inline_table) => {
                // First iterate table key values and recursively format them.
                for (_, value) in inline_table.iter_mut() {
                    self.fmt_value(value)
                }

                // Format all key and value pairs. This strips unnecessarily whitespace and adds spaces between key and value.
                // e.g '{key=   value}' -> '{ key = value }'
                inline_table.fmt();

                // Remove prefix and postfix white spaces from the inline table.
                inline_table.decor_mut().set_prefix(" ");
                inline_table.decor_mut().set_suffix("");
            }
            Value::Float(..)
            | Value::String(..)
            | Value::Datetime(..)
            | Value::Integer(..)
            | Value::Boolean(_) => {
                // Remove prefix and postfix white spaces from the inline table.
                // e.g 'a =    true    ' -> 'a = true'
                value.decor_mut().set_prefix(" ");
                value.decor_mut().set_suffix("");
            }
        }

        // Adds back the comment that was trimmed during formatting.
        Self::set_comment(prefix_comment, suffix_comment, value.decor_mut());
    }

    // Iterate array of `Values` and format them.
    fn fmt_array(&self, array: &mut Array) {
        let (prefix_comment, suffix_comment) = Self::get_comments(array.decor_mut());

        for element in array.iter_mut() {
            self.fmt_value(element);
        }

        array.fmt();

        array.decor_mut().set_prefix(" ");
        array.decor_mut().set_suffix("");

        Self::set_comment(prefix_comment, suffix_comment, array.decor_mut());
    }

    // Iterate table key values and recursively format them.
    fn fmt_table(&self, table: &mut Table, depth: usize) {
        for (ref mut key, ref mut val) in table.iter_mut() {
            self.visit_item(key, val, depth + 1);
        }
    }

    fn fmt_prefix_and_preserve_comments(prefix: &str) -> String {
        let trimmed = prefix
            .lines()
            .into_iter()
            .filter_map(|line| {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(format!("{trimmed}"))
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        if trimmed.is_empty() {
            "".to_string()
        } else {
            format!("{}\n", trimmed)
        }
    }

    fn get_comments(decor: &Decor) -> (String, String) {
        let prefix_comment = Self::get_comment(decor.prefix().unwrap_or_default());
        let suffix_comment = Self::get_comment(decor.suffix().unwrap_or_default());

        (prefix_comment, suffix_comment)
    }

    fn set_comment(prefix_comment: String, suffix_comment: String, decor_mut: &mut Decor) {
        if !suffix_comment.is_empty() {
            Self::set_comment_suffix(&suffix_comment, decor_mut);
        }

        if !prefix_comment.is_empty() {
            Self::set_comment_prefix(&prefix_comment, decor_mut);
        }
    }

    /// Strips the suffix from an item preserving comments.
    fn set_comment_suffix(suffix: &str, decor_mut: &mut Decor) {
        decor_mut.set_suffix(format!(" #{}", suffix));
    }

    /// Strips the suffix from an item preserving comments.
    fn set_comment_prefix(suffix: &str, decor_mut: &mut Decor) {
        decor_mut.set_prefix(format!(" #{}", suffix));
    }

    /// Returns the comment. if present, fetched by splitting the string at #.
    fn get_comment(input: &str) -> String {
        let split = input.split("#").collect::<Vec<&str>>();

        split
            .get(1)
            .map_or("".to_string(), |s| s.trim_end().to_string())
    }
}

/// Appends a line after the last item in a table at the end of each section.
/// Sections are to be separated by one line.
pub struct AppendLineAfterSection;

impl TomlFormatter for AppendLineAfterSection {
    fn visit_document(
        &mut self,
        toml_document: &mut Document,
        _config: &TomlFormatConfig,
    ) -> anyhow::Result<()> {
        iter_sections_as_tables(toml_document, |section_key, section| {
            // Package section is the first section in the file hence we dont want to prepend it with a new line.
            if section_key.get() != "package" {
                Self::append_empty_line(section);
            }
        });
        Ok(())
    }
}

impl AppendLineAfterSection {
    fn append_empty_line(table: &mut Table) {
        let decor = table.decor_mut();
        let prefix = decor.prefix().unwrap_or("").to_owned();
        decor.set_prefix("\n".to_owned() + &prefix);
    }
}

/// Wraps an array when it surpasses a configurable line length.
pub struct WrapArray;

impl TomlFormatter for WrapArray {
    fn visit_document(
        &mut self,
        toml_document: &mut Document,
        config: &TomlFormatConfig,
    ) -> anyhow::Result<()> {
        iter_sections_as_tables(toml_document, |section_key, section| {
            if section_key.get() != "package" {
                self.visit_table(section, config.wrap_array.unwrap())
            }
        });

        Ok(())
    }
}

impl WrapArray {
    fn visit_table(&self, table: &mut Table, wrap_array: usize) {
        for (key, value) in table.iter_mut() {
            // Only deal with array values.
            if let Item::Value(Value::Array(array)) = value {
                self.format_array(key, array, wrap_array);
            }
        }
    }

    /// Formats an array by wrapping it when it surpasses a configurable line length.
    fn format_array(&self, key: KeyMut<'_>, array: &mut Array, wrap_array: usize) {
        // Format to [item1, item2, ...]
        array.fmt();

        // Length of key doesn't include decor. Length of array does. So we add 2 (" =").
        if key.get().len() + 1 + array.to_string().chars().count() > wrap_array {
            array.iter_mut().for_each(|item| {
                item.decor_mut().set_prefix("\n    ");
            });

            array
                .iter_mut()
                .last()
                .map(|l| l.decor_mut().set_suffix("\n"));
        }
    }
}

/// If a root-level key-value pair is to long, create the table as a separate section.
///
/// ```
/// [dependency]
/// a = {version="0.4.1", path="some_very_long_path_to_directory", git="https://github.come/some_weird_long_repository_name"}
/// ```
///
/// to
///
/// ```
/// [dependency.a]
/// version="0.4.1"
/// path="some_very_long_path_to_directory"
/// git="https://github.come/some_weird_long_repository_name"
/// ```
pub struct InlineTableWrap {
    long_tables: Vec<(String, String, Item)>,
    current_section_key: String,
}

impl TomlFormatter for InlineTableWrap {
    fn visit_document(
        &mut self,
        toml_document: &mut Document,
        config: &TomlFormatConfig,
    ) -> anyhow::Result<()> {
        iter_sections_as_tables(toml_document, |section_key, section| {
            // Package section should remain as it is written.
            if section_key.get() != "package" {
                self.current_section_key = section_key.to_owned();
                self.fmt_table(section, config.wrap_table.unwrap());
            }
        });

        let mut long_tables = vec![];

        std::mem::swap(&mut self.long_tables, &mut long_tables);

        long_tables.into_iter().for_each(|(section, key, table)| {
            if let Item::Value(Value::InlineTable(table)) = table {
                toml_document[&section][&key] = Item::Table(table.into_table());
            }
        });

        Ok(())
    }
}

impl InlineTableWrap {
    pub fn new() -> Self {
        Self {
            long_tables: Vec::new(),
            current_section_key: String::new(),
        }
    }

    fn fmt_table(&mut self, table: &mut dyn TableLike, max_width: usize) {
        let mut long_table_keys = vec![];

        table.iter_mut().for_each(|(key, node)| {
            if let Some(table) = node.as_inline_table_mut() {
                // Length of key doesn't include decor. Length of array does. So we add 2 (" =").
                if key.get().len() + 1 + table.to_string().len() > max_width {
                    long_table_keys.push(key.get().to_owned());
                }
            }
        });

        long_table_keys.into_iter().for_each(|key| {
            let item = table.remove(&key).unwrap();
            self.long_tables
                .push((self.current_section_key.clone(), key, item));
        });

        table.iter_mut().for_each(|(_, node)| {
            self.visit_item(node, max_width);
        });
    }

    fn visit_item(&mut self, item: &mut Item, max_width: usize) {
        match item {
            Item::Value(value) => match value {
                Value::InlineTable(inline_table) => self.fmt_table(inline_table, max_width),
                Value::String(_)
                | Value::Integer(_)
                | Value::Float(_)
                | Value::Boolean(_)
                | Value::Datetime(_)
                | Value::Array(_) => {}
            },
            Item::Table(table) => self.fmt_table(table, max_width),
            Item::ArrayOfTables(array) => {
                array.iter_mut().for_each(|x| self.fmt_table(x, max_width))
            }
            Item::None => {}
        }
    }
}
