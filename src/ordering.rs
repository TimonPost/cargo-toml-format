use std::{cmp::Ordering, collections::HashMap, str::FromStr};

use strum::EnumProperty;
use toml_edit::{Document, Item, Key, Table, Value};

use crate::{
    package_order::{PackageOrder, TomlSection, TomlSort},
    toml_config::TomlFormatConfig,
    utils::item_len,
};

use super::TomlFormatter;

/// Order sections in the toml document according to the [manifest's][1] order.
///
/// [1]: https://doc.rust-lang.org/cargo/reference/manifest.html
pub struct OrderSections;

impl TomlFormatter for OrderSections {
    fn visit_document(
        &mut self,
        toml_document: &mut Document,
        config: &TomlFormatConfig,
    ) -> anyhow::Result<()> {
        if !config.order_sections {
            return Ok(());
        }

        let mut section_tables = HashMap::<String, (Key, Table)>::new();
        let all_sections = TomlSection::manifest_spec();

        // Collect all section tables
        toml_document.iter().for_each(|(section_key, _)| {
            let (section_key, section_item) = toml_document.get_key_value(section_key).unwrap();
            let section_table = section_item.as_table().unwrap();

            section_tables.insert(
                section_key.get().to_string(),
                (section_key.clone(), section_table.clone()),
            );
        });

        // Clear the document, lets sort the tables and add them back with their new positions.
        toml_document.clear();

        let mut idx = 0;

        // Iterate tables as they should be ordered.
        for ordered_section in all_sections {
            // Process the table if it exists within the document.
            if let Some((section_key, section_table)) = section_tables.get(&ordered_section) {
                let mut new_table = section_table.clone();

                // Thee possibilities:
                // [section] and [section.sub] (both section and sub table have positions)
                // [section] (section has position)
                // [section.sub] (section does not and sub table does have a position)

                // When parent has position, assign its new index.
                if new_table.position().is_some() {
                    idx += 1;
                    new_table.set_position(idx);
                }

                // Add table back to the document.
                toml_document.insert(section_key.get(), Item::Table(new_table.clone()));

                let new_table =
                    if let Some((mut k, v)) = toml_document.get_key_value_mut(section_key.get()) {
                        k.decor_mut()
                            .set_prefix(section_key.decor().prefix().unwrap().to_string());
                        k.decor_mut()
                            .set_suffix(section_key.decor().suffix().unwrap().to_string());
                        v.as_table_mut().unwrap()
                    } else {
                        panic!();
                    };

                let section_has_pos = new_table.position().is_some();

                // Iterate the sub tables and see if they need new indexes.
                // TODO: add alphabetical sort.
                new_table.iter_mut().for_each(|(_, recursive_item)| {
                    // Only tables have positions.
                    if let Some(table) = recursive_item.as_table_mut() {
                        let subtable_has_pos = table.position().is_some();

                        if (subtable_has_pos && section_has_pos)
                            || (subtable_has_pos && !section_has_pos)
                        {
                            idx += 1;
                            table.set_position(idx);
                        } else if !subtable_has_pos && !section_has_pos {
                            // Both section and sub table have no positions.
                            panic!("can not occur");
                        } else if !subtable_has_pos && section_has_pos {
                            // Sub table does not have any position.
                        } else {
                            panic!("Not possible")
                        }
                    }
                });
            }
        }
        Ok(())
    }
}

/// Order the package section according to the [manifest's][1] order.
///
/// [1]: https://doc.rust-lang.org/cargo/reference/manifest.html
pub struct OrderPackageSection;

impl TomlFormatter for OrderPackageSection {
    fn visit_document(
        &mut self,
        toml_document: &mut Document,
        config: &TomlFormatConfig,
    ) -> anyhow::Result<()> {
        if !config.order_package_section {
            return Ok(());
        }

        if let Some(Item::Table(package_section)) = toml_document.get_mut("package") {
            package_section.sort_values_by(|key_1, _, key_2, _| {
                let order_1 = PackageOrder::from_str(key_1.get());

                let order_2 = PackageOrder::from_str(key_2.get());

                if order_1.is_err() || order_2.is_err() {
                    return Ordering::Equal;
                }

                let key_1_order = order_1
                    .unwrap()
                    .get_str("order")
                    .expect("order should be defined in enum")
                    .parse::<u8>()
                    .expect("order should be a u8 integer");
                let key_2_order = order_2
                    .unwrap()
                    .get_str("order")
                    .expect("order should be defined in enum")
                    .parse::<u8>()
                    .expect("order should be a u8 integer");

                key_1_order.cmp(&key_2_order)
            })
        }
        Ok(())
    }
}

/// Order all table and inline table keys alphabetical order.
pub struct OrderTableKeysAlphabetically;

impl TomlFormatter for OrderTableKeysAlphabetically {
    fn visit_document(
        &mut self,
        toml_document: &mut Document,
        _config: &TomlFormatConfig,
    ) -> anyhow::Result<()> {
        toml_document.iter_mut().for_each(|(section_key, item)| {
            if section_key.get() != "package" {
                // package section is sorted according to the manifest order and not alphabetically.
                Self::order_item(item);
            }
        });
        Ok(())
    }
}

impl OrderTableKeysAlphabetically {
    pub fn order_item(item: &mut Item) {
        match item {
            Item::None => todo!(),
            Item::Value(value) => Self::order_value(value),
            Item::Table(table) => Self::order_table(table),
            Item::ArrayOfTables(tables) => tables.iter_mut().for_each(|table| {
                Self::order_table(table);
            }),
        }
    }

    pub fn order_table(table: &mut Table) {
        table.sort_values_by(|key_1, _, key_2, _| key_1.get().cmp(&key_2.get()));

        table.iter_mut().for_each(|(_, value)| {
            Self::order_item(value);
        })
    }

    pub fn order_value(value: &mut toml_edit::Value) {
        match value {
            Value::Array(array) => array.iter_mut().for_each(|value| {
                Self::order_value(value);
            }),
            Value::InlineTable(inline_table) => {
                inline_table.sort_values_by(|key_1, _, key_2, _| key_1.get().cmp(&key_2.get()));

                inline_table.iter_mut().for_each(|(_, value)| {
                    Self::order_value(value);
                })
            }
            Value::String(_)
            | Value::Integer(_)
            | Value::Float(_)
            | Value::Boolean(_)
            | Value::Datetime(_) => {}
        }
    }
}

pub struct OrderDependencies;

impl TomlFormatter for OrderDependencies {
    fn visit_document(
        &mut self,
        toml_document: &mut Document,
        config: &TomlFormatConfig,
    ) -> anyhow::Result<()> {
        let dependencies = toml_document
            .get_mut("dependencies")
            .ok_or_else(|| anyhow::anyhow!("Dependencies tag not found in toml document"))?;

        self.sort_dependencies(dependencies, config).unwrap();

        Ok(())
    }
}

impl OrderDependencies {
    pub fn sort_dependencies(
        &mut self,
        dependencies: &mut Item,
        config: &TomlFormatConfig,
    ) -> anyhow::Result<()> {
        let dependency_sorts = if let Some(dependency_sorts) = &config.order_dependencies {
            dependency_sorts
        } else {
            return Ok(());
        };

        if let Item::Table(ref mut dependencies) = dependencies {
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
