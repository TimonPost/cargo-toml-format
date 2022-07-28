use toml_edit::{Document, Item, Key, Table, Value};

use crate::formatting::{KeyQuoteTrimmer, KeyTrimmer, OrderSections, TomlFormatter, AddSpaceBetweenAssignments};
use crate::{
    formatting::{AppendLineAfterSection, OrderPackageSection, SectionKeyNameTrimmer},
    package_order::TomlSort,
    toml_config::TomlFormatConfig,
};

pub struct CargoToml {
    pub toml_document: Document,
}

impl CargoToml {
    /// Crates a in-memory toml definition from the given toml contents.
    pub fn new(toml_contents: String) -> anyhow::Result<Self> {
        let toml_document = toml_contents
            .parse::<Document>()
            .map_err(|e| anyhow::anyhow!("Failed to parse toml. {e}"))?;

        Ok(Self { toml_document })
    }

    pub fn format(&mut self, config: TomlFormatConfig) -> anyhow::Result<()> {
        self.format_document(&config);
        self.format_sections(&config);
        self.sort_dependencies(&config)
    }

    pub fn dependencies(&mut self) -> anyhow::Result<&mut Item> {
        self.toml_document
            .get_mut("dependencies")
            .ok_or_else(|| anyhow::anyhow!("Dependencies tag not found in toml document"))
    }

    pub fn format_sections(&mut self, config: &TomlFormatConfig) {
        if config.order_package_section {
            OrderPackageSection
                .format_toml(&mut self.toml_document, &config)
                .unwrap();
        }

        if config.order_sections {
            OrderSections
                .format_toml(&mut self.toml_document, &config)
                .unwrap();
        }
    }
    pub fn format_document(&mut self, config: &TomlFormatConfig) {
        if config.trim_keys {
            KeyTrimmer
                .format_toml(&mut self.toml_document, &config)
                .unwrap();
        }

        if config.trim_section_keys {
            SectionKeyNameTrimmer
                .format_toml(&mut self.toml_document, &config)
                .unwrap();
        }

        if config.add_newline_after_section {
            AppendLineAfterSection
                .format_toml(&mut self.toml_document, &config)
                .unwrap();
        }

        if config.trim_key_quotes {
            KeyQuoteTrimmer
                .format_toml(&mut self.toml_document, &config)
                .unwrap();
        }

        if config.space_between_assignment {
            AddSpaceBetweenAssignments.format_toml(&mut self.toml_document, config).unwrap();
        }
    }

    pub fn sort_dependencies(&mut self, config: &TomlFormatConfig) -> anyhow::Result<()> {
        let dependency_sorts = if let Some(dependency_sorts) = &config.dependency_sorts {
            dependency_sorts
        } else {
            return Ok(());
        };

        if let Item::Table(ref mut dependencies) = self.dependencies()? {
            let alphabetical_sort_enable = dependency_sorts
                .iter()
                .filter(|d| *d == &TomlSort::Alphabetical)
                .count()
                > 0;
            let length_sort_enable = dependency_sorts
                .iter()
                .filter(|d| *d == &TomlSort::Length)
                .count()
                > 0;

            fn len_sort(
                key_1: &Key,
                item_1: &Item,
                key_2: &Key,
                item_2: &Item,
            ) -> std::cmp::Ordering {
                let key_1_count = key_1.get().char_indices().count();
                let key_2_count = key_2.get().char_indices().count();

                let item_1_len = item_len(item_1);
                let item_2_len = item_len(item_2);

                let entry_1_len = key_1_count + item_1_len;
                let entry_2_len = key_2_count + item_2_len;

                if entry_1_len < entry_2_len {
                    std::cmp::Ordering::Less
                } else if entry_1_len > entry_2_len {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            }

            fn alphabetical_sort(
                key_1: &Key,
                _: &Item,
                key_2: &Key,
                _: &Item,
            ) -> std::cmp::Ordering {
                key_1.get().to_lowercase().cmp(&key_2.get().to_lowercase())
            }

            if alphabetical_sort_enable && !length_sort_enable {
                dependencies.sort_values_by(|key_1, _item_1, key_2, _item_2| {
                    alphabetical_sort(key_1, _item_1, key_2, _item_2)
                });
            }

            if alphabetical_sort_enable && length_sort_enable {
                dependencies.sort_values_by(|key_1, item_1, key_2, item_2| {
                    let alphabetic_order = alphabetical_sort(key_1, item_1, key_2, item_2);
                    let length_order = len_sort(key_1, item_1, key_2, item_2);

                    match alphabetic_order {
                        std::cmp::Ordering::Less => {
                            if length_order == std::cmp::Ordering::Less {
                                std::cmp::Ordering::Less
                            } else {
                                std::cmp::Ordering::Greater
                            }
                        }
                        std::cmp::Ordering::Equal => {
                            if length_order == std::cmp::Ordering::Less {
                                std::cmp::Ordering::Less
                            } else {
                                std::cmp::Ordering::Greater
                            }
                        }
                        std::cmp::Ordering::Greater => {
                            if length_order == std::cmp::Ordering::Less {
                                std::cmp::Ordering::Less
                            } else {
                                std::cmp::Ordering::Greater
                            }
                        }
                    }
                });
            }

            if length_sort_enable && !alphabetical_sort_enable {
                dependencies.sort_values_by(|key_1, item_1, key_2, item_2| {
                    len_sort(key_1, item_1, key_2, item_2)
                });
            }
        }

        Ok(())
    }
}

fn item_len(item: &Item) -> usize {
    match item {
        Item::None => 0,
        Item::Value(val) => match val {
            toml_edit::Value::String(str) => return str.value().char_indices().count(),
            toml_edit::Value::Integer(int) => return int.to_repr().as_raw().char_indices().count(),
            toml_edit::Value::Float(float) => {
                return float.to_repr().as_raw().char_indices().count()
            }
            toml_edit::Value::Boolean(boolean) => {
                return boolean.to_repr().as_raw().char_indices().count()
            }
            toml_edit::Value::Datetime(datetime) => {
                return datetime.to_repr().as_raw().char_indices().count()
            }
            toml_edit::Value::Array(a) => a.iter().map(|i| item_len(&Item::Value(i.clone()))).sum(),
            toml_edit::Value::InlineTable(inline_table) => table_len(inline_table.get_values()),
        },
        Item::Table(table) => table_len(table.get_values()),
        Item::ArrayOfTables(array_table) => {
            array_table.iter().map(|t| table_len(t.get_values())).sum()
        }
    }
}

fn table_len(key_values: Vec<(Vec<&Key>, &Value)>) -> usize {
    key_values
        .iter()
        .map(|(keys, value)| -> usize {
            let keys_len: usize = keys
                .iter()
                .map(|key| key.get().char_indices().count())
                .sum();
            let value_len = item_len(&Item::Value((*value).clone()));
            keys_len + value_len
        })
        .sum()
}
