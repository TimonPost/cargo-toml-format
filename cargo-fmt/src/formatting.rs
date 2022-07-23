use std::{cmp::Ordering, str::FromStr};

use strum::EnumProperty;
use toml_edit::{Decor, Document, Item, Key, Table};

use crate::{
    package_order::{PackageOrder, TomlSection},
    toml_config::TomlFormatConfig, sort,
};
pub trait TomlFormatter {
    fn format_toml(
        &self,
        toml_document: &mut Document,
        config: &TomlFormatConfig,
    ) -> anyhow::Result<()>;
}

pub struct OrderSectionsNew;

impl TomlFormatter for OrderSectionsNew {
    fn format_toml(
        &self,
        toml_document: &mut Document,
        config: &TomlFormatConfig,
    ) -> anyhow::Result<()> {
        if !config.order_sections {
            return Ok(());
        }

        sort::sort_toml(toml_document);
        
        Ok(())
    }
}

pub struct OrderSections;

impl TomlFormatter for OrderSections {
    fn format_toml(
        &self,
        toml_document: &mut Document,
        config: &TomlFormatConfig,
    ) -> anyhow::Result<()> {
        if !config.order_sections {
            return Ok(());
        }

        let sections = toml_document.as_table_mut();

        let mut positions = Vec::new();

        // First store positions of all section tables.
        for (_, t) in sections.iter() {
            positions.push(
                t.as_table()
                    .ok_or(anyhow::anyhow!("Section was not a table."))?
                    .position()
                    .ok_or(anyhow::anyhow!("Section had no position"))?,
            );
        }

        // Then, sort the sections by their order.
        sections.sort_values_by(|key_1, _, key_2, _| {                  
            let order_1 = TomlSection::from_str(key_1.get());

            let order_2 = TomlSection::from_str(key_2.get());

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
        });

        // Finally, update the positions of the sorted elements to match the correct old index positions.
        for (i, (_, t)) in sections.iter_mut().enumerate() {
            t.as_table_mut()
                .as_mut()
                .ok_or(anyhow::anyhow!("Section was not a table."))?
                .set_position(positions[i]);
        }

        Ok(())
    }
}

pub struct OrderPackageSection;

impl TomlFormatter for OrderPackageSection {
    fn format_toml(
        &self,
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

pub struct AppendLineAfterSection;

impl TomlFormatter for AppendLineAfterSection {
    fn format_toml(
        &self,
        toml_document: &mut Document,
        _config: &TomlFormatConfig,
    ) -> anyhow::Result<()> {
        // Iterate the section items, trim empty lines, and append newline after each section.
        toml_document
            .as_table_mut()
            .iter_mut()
            .skip(1)
            .for_each(|(_, section)| visit_item_mut(section, false));

        Ok(())
    }
}

pub struct SectionKeyNameTrimmer;

impl TomlFormatter for SectionKeyNameTrimmer {
    fn format_toml(
        &self,
        toml_document: &mut Document,
        _config: &TomlFormatConfig,
    ) -> anyhow::Result<()> {
        // Iterate through toml sections.
        toml_document
            .as_table_mut()
            .iter_mut()
            .for_each(|(mut key, _)| {
                // Trim empty spaces around section key names 'e.g' [ name ] -> [name].
                trim_decor_blank_lines(key.decor_mut());
            });

        Ok(())
    }
}

pub struct KeyTrimmer;

impl TomlFormatter for KeyTrimmer {
    fn format_toml(
        &self,
        toml_document: &mut Document,
        _config: &TomlFormatConfig,
    ) -> anyhow::Result<()> {
        // Iterate through toml sections.
        toml_document
            .as_table_mut()
            .iter_mut()
            .for_each(|(_, section)| {
                // Trim empty spaces and lines around section values. e.g \n[name]\n -> [name]\n.
                visit_item_mut(section, true);
            });

        Ok(())
    }
}

pub struct KeyQuoteTrimmer;

impl TomlFormatter for KeyQuoteTrimmer {
    fn format_toml(
        &self,
        toml_document: &mut Document,
        _config: &TomlFormatConfig,
    ) -> anyhow::Result<()> {
        // Iterate through toml sections.
        toml_document
            .as_table_mut()
            .iter_mut()
            .for_each(|(_, section)| {
                // Trim quotes around section key names 'e.g' "key" = value -> key = value.
                if let Item::Table(table) = section {
                    let mut trimmed_keys = vec![];

                    // Iterate table keys and trim away quotes.
                    table.iter_mut().for_each(|(key, value)| {
                        let trimmed = key.get().trim_matches('"');
                        let decor = key.decor().clone();

                        trimmed_keys.push((
                            Key::new(trimmed.to_string()).with_decor(decor),
                            value.clone(),
                        ));
                    });

                    // Iterate trimmed keys and insert them back into the table.
                    trimmed_keys
                        .into_iter()
                        .for_each(|(trimmed_key, new_value)| {
                            table.remove(trimmed_key.get());
                            table.insert(trimmed_key.get(), new_value);

                            // Unfortunately I can't figure out how to internally update the string representation of the key,
                            // hence we have to do it this way.
                            table.iter_mut().for_each(|(mut key, _)| {
                                key.decor_mut().set_prefix(
                                    trimmed_key.decor().prefix().unwrap_or("").to_string(),
                                );
                                key.decor_mut().set_suffix(
                                    trimmed_key.decor().suffix().unwrap_or("").to_string(),
                                );
                            });
                        });
                }
            });

        Ok(())
    }
}

fn visit_item_mut(item: &mut Item, trimming: bool) {
    if let Item::Table(table) = item {
        visit_table_mut(table, trimming);
    }
}

fn visit_table_mut(table: &mut Table, trimming: bool) {
    if trimming {
        trim_decor_blank_lines(table.decor_mut());
        table.iter_mut().for_each(|(mut key, _)| {
            trim_decor_blank_lines(key.decor_mut());
        });
    } else {
        // Append newline to end of table block.
        let decor = table.decor_mut();
        let prefix = decor.prefix().unwrap_or("").to_owned();
        decor.set_prefix("\n".to_owned() + &prefix);
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

pub struct AddSpaceBetweenAssignments;

impl TomlFormatter for AddSpaceBetweenAssignments {
    fn format_toml(
        &self,
        toml_document: &mut Document,
        _config: &TomlFormatConfig,
    ) -> anyhow::Result<()> {
        // Iterate through toml sections.
        toml_document
            .as_table_mut()
            .iter_mut()
            .for_each(|(_, section)| {
                // Trim empty spaces and lines around section values. e.g \n[name]\n -> [name]\n.
                visit_item_mut(section, true);
            });

        Ok(())
    }
}

impl AddSpaceBetweenAssignments {
    fn fmt_table(table: &mut Table) { 
        for (ref mut key, ref mut val) in table.iter_mut() {
            key.decor_mut().set_suffix(" ");

            Self::fmt_item(val);
        }
    }

    fn fmt_item(item: &mut Item) {
        match item {
            Item::None => {
                todo!();
            }
            Item::Value(value) => {
                value.decor_mut().set_prefix(" ");
            }
            Item::Table(table) => {
                todo!();
            }
            Item::ArrayOfTables(_) => {
                todo!();
            }
        }
    }
}

pub struct MaxLineWidthFmt {
    pub max_line_width: usize,
}

impl MaxLineWidthFmt {}
