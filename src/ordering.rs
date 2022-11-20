use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap, HashSet},
    iter::FromIterator,
    ops::Index,
    str::FromStr,
};

use strum::EnumProperty;
use toml_edit::{ArrayOfTables, Decor, Document, Item, Key, Table, Value};

use crate::{
    iter_sections_as_items, iter_sections_as_items_mut,
    package_order::{PackageOrder, TomlSection, TomlSort},
    toml_config::TomlFormatConfig,
    utils::item_len,
};

use super::TomlFormatter;

///
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
        let mut array_of_tables = HashMap::<String, (Key, ArrayOfTables)>::new();

        let mut manifest_sections = TomlSection::manifest_spec();
        let mut sections_from_config = config.custom_section_order.clone();

        // Remove any manifest section if custom ordering is needed for it.
        for section in &sections_from_config {
            manifest_sections
                .iter()
                .position(|n| n == section)
                .map(|e| manifest_sections.remove(e));
        }

        // Add the left over sections from the manifest to the end of the custom ordering.
        for section in manifest_sections {
            sections_from_config.push(section);
        }

        // Collect all section tables
        iter_sections_as_items(toml_document, |section_key, section_item| {
            if let Some(section_table) = section_item.as_table() {
                section_tables.insert(
                    section_key.get().to_string(),
                    (section_key.clone(), section_table.clone()),
                );
            }
            if let Some(tables) = section_item.as_array_of_tables() {
                array_of_tables.insert(
                    section_key.get().to_string(),
                    (section_key.clone(), tables.clone()),
                );
            }
        });

        // Clear the document, lets sort the tables and add them back with their new positions.
        toml_document.clear();

        let mut idx = 0;

        // Iterate tables as they should be ordered.
        for ordered_section in sections_from_config {
            if let Some((section_key, section_table)) = array_of_tables.get(&ordered_section) {
                let mut new_tables = section_table.clone();

                for table in new_tables.iter_mut() {
                    idx += 1;
                    table.set_position(idx);
                }

                toml_document.insert(section_key.get(), Item::ArrayOfTables(new_tables));

                if let Some((mut k, _v)) = toml_document.get_key_value_mut(section_key.get()) {
                    k.decor_mut()
                        .set_prefix(section_key.decor().prefix().unwrap().to_string());
                    k.decor_mut()
                        .set_suffix(section_key.decor().suffix().unwrap().to_string());
                }
            }

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
                            if !table.is_implicit() {
                                // Both section and sub table have no positions.
                                panic!("Not possible")
                            } else {
                                // handle sorting `[target.my-special-i686-platform.dependencies]` like tables.
                            }
                        } else if !subtable_has_pos && section_has_pos {
                            // Sub table does not have any position.
                        } else {
                            panic!("Not possible")
                        }
                    }
                });
            }
        }

        // if let Some(last) = toml_document.as_table_mut().iter_mut().last() {
        //     if let Item::Table(table) = last.1 {
        //         if let Some(last_item) =  table.iter_mut().last() {
        //             if let Item::Table(table) = last_item.1 {
        //                 table.decor_mut().set_suffix("\n");
        //             }
        //             if let Item::Value(value) = last_item.1 {
        //                 value.decor_mut().set_suffix("\n");
        //             }
        //         }
        //     }
        // }

        Ok(())
    }
}

/// See documentation on [crate::TomlFormatConfig::order_package_section].
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
        config: &TomlFormatConfig,
    ) -> anyhow::Result<()> {
        iter_sections_as_items_mut(toml_document, |section_key, item| {
            if !config
                .exclude_tables_from_ordering
                .contains(&section_key.get().to_string())
            {
                // package section is sorted according to the manifest order and not alphabetically.
                Self::order_item(item, config);
            }
        });

        Ok(())
    }
}

impl OrderTableKeysAlphabetically {
    pub fn order_item(item: &mut Item, config: &TomlFormatConfig) {
        match item {
            Item::None => todo!(),
            Item::Value(value) => Self::order_value(value),
            Item::Table(table) => Self::order_table(table, config),
            Item::ArrayOfTables(tables) => tables.iter_mut().for_each(|table| {
                Self::order_table(table, config);
            }),
        }
    }

    pub fn order_table(table: &mut Table, config: &TomlFormatConfig) {
        table.sort_values_by(|key_1, val1, key_2, val2| {
            if config
                .exclude_keys_from_ordering
                .iter()
                .any(|e| key_1.contains(e) || key_2.contains(e))
            {
                Ordering::Equal
            } else {
                key_1.get().cmp(&key_2.get())
            }
        });

        table.iter_mut().for_each(|(_, value)| {
            Self::order_item(value, config);
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
        if !config.order_dependencies_alphabetically {
            return Ok(());
        }

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
        if let Item::Table(ref mut dependencies) = dependencies {
            fn alphabetical_sort(
                key_1: &Key,
                _: &Item,
                key_2: &Key,
                _: &Item,
            ) -> std::cmp::Ordering {
                key_1.get().to_lowercase().cmp(&key_2.get().to_lowercase())
            }

            dependencies.sort_values_by(|key_1, _item_1, key_2, _item_2| {
                alphabetical_sort(key_1, _item_1, key_2, _item_2)
            });
        }

        Ok(())
    }
}

pub struct OrderSectionKeysByGroupAlphabetically;

impl TomlFormatter for OrderSectionKeysByGroupAlphabetically {
    fn visit_document(
        &mut self,
        toml_document: &mut Document,
        config: &TomlFormatConfig,
    ) -> anyhow::Result<()> {
        if !config.order_section_keys_by_group_alphabetically {
            return Ok(());
        }

        iter_sections_as_items_mut(toml_document, |section_key, item| {
            if section_key.get() == "package" {
                return;
            }

            if let Item::Table(table) = item {
                let table_clone = table.clone();

                // The groups separated by white space.
                let mut groups = BTreeMap::new();
                // Stores the first entry of the group to preserve the prefix decor.
                let mut group_header = HashMap::new();

                let mut group_idx = 0;

                for (idx, (mut table_key, table_value)) in table_clone
                    .iter()
                    .map(|(table_key, _)| table.remove_entry(table_key).unwrap())
                    .enumerate()
                {
                    let table_key_decor = table_key.decor_mut();
                    
                    // A group exists if there is at least one blank line between two keys.

                    let blank_lines = table_key_decor
                        .prefix()
                        .map(|prefix| prefix.lines().filter(|l| !l.starts_with('#')).count())
                        .unwrap_or(0);

                    if blank_lines > 0 {                     
                        // We may need the the original decor for the top sorted item of the group.
                        group_header.insert(idx, table_key_decor.clone());
                        
                           // Reset the prefix since a new entry of the group may be sorted to the top.
                           table_key_decor.set_prefix("");

                        groups.entry(idx).or_insert_with(|| vec![(table_key, table_value)]);
                        group_idx = idx;
                    } else {                    
                        if !groups.contains_key(&group_idx) {
                            group_header.insert(group_idx, table_key_decor.clone());
                        }

                        groups.entry(group_idx).or_default().push((table_key, table_value));
                    }
                }

                for (idx, group) in groups.iter_mut() {
                    group.sort_by(|a, b| a.0.cmp(&b.0));

                    for (i, (key, value)) in group.iter_mut().enumerate() {
                        // Only apply the original group header to the first item of the group.
                        if i == 0 {
                            let original_decor = group_header.get(idx).unwrap();
                            key.decor_mut().set_prefix(original_decor.prefix().unwrap_or(""));
                        }

                        table.insert_formatted(&key, value.clone());
                    }
                }
            }
        });

        Ok(())
    }
}
