use std::{cmp::Ordering, str::FromStr, collections::HashMap};

use strum::EnumProperty;
use toml::value::Map;
use toml_edit::{Decor, Value, Document, Item, Key, Table, KeyMut, Array};

use crate::{
    package_order::{PackageOrder, TomlSection},
    toml_config::TomlFormatConfig, 
};
pub trait TomlFormatter {
    fn format_toml(
        &self,
        toml_document: &mut Document,
        config: &TomlFormatConfig,
    ) -> anyhow::Result<()>;
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

        let mut section_tables = HashMap::<String, (Key, Table)>::new();
        let all_sections = TomlSection::manifest_spec();
    
        // Collect all section tables
        toml_document.iter().for_each(|(section_key, _)| {          
            let (section_key, section_item) = toml_document.get_key_value(section_key).unwrap();
            let section_table = section_item.as_table().unwrap();

            section_tables.insert(section_key.get().to_string(), (section_key.clone(), section_table.clone()));
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
                    
                let new_table = if let Some((mut k,v)) = toml_document.get_key_value_mut(section_key.get()) {
                    k.decor_mut().set_prefix(section_key.decor().prefix().unwrap().to_string());
                    k.decor_mut().set_suffix(section_key.decor().suffix().unwrap().to_string());
                    v.as_table_mut().unwrap()
                } else {
                    panic!();
                };
            
                let section_has_pos = new_table.position().is_some();

                // Iterate the sub tables and see if they need new indexes.
                // TODO: add alphabetical sort.
                new_table.iter_mut().for_each(|(recursive_item_key,recursive_item)| {            
                    // Only tables have positions.
                    if let Some(table) = recursive_item.as_table_mut() {                    
                        let subtable_has_pos = table.position().is_some();

                        if (subtable_has_pos && section_has_pos) || (subtable_has_pos && !section_has_pos)  {  
                            idx += 1;             
                            table.set_position(idx);           
                        } else if !subtable_has_pos && !section_has_pos {   
                            // Both section and sub table have no positions.   
                            panic!("can not occur");
                        } else if !subtable_has_pos && section_has_pos {          
                            // Sub table does not have any position.
                        }
                        else{panic!("Not possible")}  
                    } 
                });        
            }           
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
            .for_each(|(mut key, section)| {
                key.decor_mut().set_suffix(" ");
                
                self.fmt_item(section, 0);
            });

        Ok(())
    }
}

// standalone table
// table as value 
// root key
// key in table
impl AddSpaceBetweenAssignments {
    fn fmt_table(&self, table: &mut Table, depth: usize) { 
        for (ref mut key, ref mut val) in table.iter_mut() {
            key.decor_mut().set_suffix(" ");
            if depth > 0 {
                key.decor_mut().set_prefix(" ");
            }

           self.fmt_item(val, depth);
        }
    }

    fn fmt_standalone_table(&self, table: &mut Table, depth: usize) { 
        table.decor_mut().set_prefix("");
        table.decor_mut().set_prefix("");
    }

    fn fmt_table_in_array(&self, table: &mut Table, index: usize) { 
        panic!();
        if index > 0 {
            table.decor_mut().set_prefix(" ");
        }
        table.decor_mut().set_suffix("");
    }

    fn fmt_table_as_value(&self, table: &mut Table, depth: usize) { 
        table.decor_mut().set_prefix(" ");
        table.decor_mut().set_prefix("");
    }

    fn fmt_root_key(&self, key: &mut KeyMut) {
        key.decor_mut().set_suffix(" ");
    }

    fn fmt_table_key(&self, key: &mut KeyMut) {
        key.decor_mut().set_suffix(" ");
    }

    fn fmt_item(&self, item: &mut Item, depth: usize) {
        match item {
            Item::None => {
                todo!();
            }
            Item::Value(value) => {
                self.fmt_value(value,depth > 1, false);
            }
            Item::Table(table) => {
                self.fmt_table(table, depth + 1);
            }
            Item::ArrayOfTables(tables) => {
               for (i, table) in tables.iter_mut().enumerate() {
                    self.fmt_table_in_array(table, i);
               }
            }
        }
    }

    fn fmt_value(&self, value: &mut Value, add_space_perfix: bool, add_space_suffix: bool) {
        match value {
            _ => {}
            toml_edit::Value::Array(array) => {
                self.fmt_array(array);
            },
            toml_edit::Value::InlineTable(inline_table) => {
                for (i, (key, value)) in inline_table.iter_mut().enumerate() {
                    self.fmt_value(value, i>0, false);
               }
            }
        }

        if add_space_suffix {
            value.decor_mut().set_suffix(" ");
        }

        if add_space_perfix {
            value.decor_mut().set_prefix(" ");
        }
    }

    fn fmt_array(&self, array: &mut Array) {
       for element in array.iter_mut() {
           self.fmt_value(element, true, false)
       }
    }
}

pub struct MaxLineWidthFmt {
    pub max_line_width: usize,
}

impl MaxLineWidthFmt {}
