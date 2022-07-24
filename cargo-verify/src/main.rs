use std::{collections::HashMap, hash::Hash};

use cargo_fmt::package_order::TomlSection;
use toml_edit::{Item, Key, KeyMut, Table, Value};
mod sort;

/*
    [workspace] // pos 1

    [package.abc] // pos 2
    a="b"

    [workspace.test] // pos 3
    a="b"

    [package] // pos 4
    b="c"

    [bench.def] // pos 5
    d="1"
*/
fn main() {
    let before = r#"
  
    [target.btc]
  
    [target]
    
    "#;

    let after = "[workspace.test]";

    let mut toml_document = before.parse::<toml_edit::Document>().unwrap();

    println!("{}", toml_document.to_string());

    let mut tables = HashMap::<String, (Key, Table)>::new();
    let mut all_sections = TomlSection::manifest_spec();

    toml_document.iter().for_each(|(section_key, _)| {
        let (section_key, section_item) = toml_document.get_key_value(section_key).unwrap();
        let section_table = section_item.as_table().unwrap();

        println!(
            "====\nsection {} {:?}",
            section_key,
            section_table.position()
        );

        if section_key.get() == "target" {
            println!("{:?}", section_table);
        }

        tables.insert(
            section_key.get().to_string(),
            (section_key.clone(), section_table.clone()),
        );
    });

    toml_document.clear();

    let mut idx = 0;

    for section in all_sections {
        if let Some((section_key, section_table)) = tables.get(&section) {
            let mut new_table = section_table.clone();

            if section_table.position().is_some() {
                idx += 1;
                new_table.set_position(idx);

                println!(
                    "section {} {:?} -> {idx}",
                    section_key.get(),
                    section_table.position()
                );
            }

            toml_document.insert(section_key.get(), Item::Table(new_table.clone()));

            let new_table =
                if let Some((mut k, v)) = toml_document.get_key_value_mut(section_key.get()) {
                    set_decor(&mut k, section_key);
                    v.as_table_mut().unwrap()
                } else {
                    panic!();
                };

            let has_section_pos = new_table.position().is_some();

            new_table
                .iter_mut()
                .for_each(|(recursive_item_key, recursive_item)| {
                    if let Some(table) = recursive_item.as_table_mut() {
                        let has_recursive_pos = table.position().is_some();

                        if (has_recursive_pos && has_section_pos) {
                            idx += 1;
                            table.set_position(idx);
                            println!(
                                "1. item {}.{} {:?} -> {}",
                                section_key.get(),
                                recursive_item_key.get(),
                                table.position(),
                                idx
                            );
                        } else if (has_recursive_pos && !has_section_pos) {
                            idx += 1;
                            table.set_position(idx);

                            println!(
                                "2. item {}.{} {:?} -> {}",
                                section_key.get(),
                                recursive_item_key.get(),
                                table.position(),
                                idx
                            );
                        } else if !has_recursive_pos && !has_section_pos {
                            panic!("?");
                        } else if !has_recursive_pos && has_section_pos {
                        } else {
                            panic!()
                        }
                    } else {
                        //panic!();
                    }
                });
        }
    }

    println!("validation");

    toml_document.iter().for_each(|(section_key, _)| {
        let (section_key, section_item) = toml_document.get_key_value(section_key).unwrap();
        let section_table = section_item.as_table().unwrap();

        println!(
            "====\nsection {} {:?}",
            section_key,
            section_table.position()
        );

        section_table
            .iter()
            .for_each(|(recursive_item_key, recursive_item)| {
                if let Some(table) = recursive_item.as_table() {
                    println!(
                        "item {}.{} {:?} ",
                        section_key.get(),
                        recursive_item_key,
                        table.position()
                    );
                }
            });
    });

    println!("=====\n{}", toml_document.to_string());

    assert_eq!(toml_document.to_string(), after);
}

pub fn debug_table(table: &Table) {
    println!("{}", table.to_string());
}

fn set_decor(key: &mut KeyMut, original: &Key) {
    key.decor_mut()
        .set_prefix(original.decor().prefix().unwrap().to_string());
    key.decor_mut()
        .set_suffix(original.decor().suffix().unwrap().to_string());
}
