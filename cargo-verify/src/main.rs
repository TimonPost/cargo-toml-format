use std::{collections::HashMap, hash::Hash};

use cargo_fmt::package_order::TomlSection;
use toml_edit::{Key, Value, Item, Table, KeyMut};
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
    [workspace]

    [package.abc]
    a="b"

    [workspace.test]
    a="b"

    [package]
    b="c"

    [bench.def]

    [profile.def]
    d="1"

    [patch]    
    d="1"
    "#;

    let after = "[workspace.test]";

    let mut toml_document = before.parse::<toml_edit::Document>().unwrap();

    println!("{}", toml_document.to_string());

    let mut tables = HashMap::<String, (Key, Table)>::new();
    let mut recursive_tables = HashMap::<String, Vec<Key>>::new();
    let mut all_sections = TomlSection::manifest_spec();
    
    toml_document.iter().for_each(|(section_key, _)| {          
        let (section_key, section_item) = toml_document.get_key_value(section_key).unwrap();
        let section_table = section_item.as_table().unwrap();

        println!("section {} {:?}", section_key, section_table.position());

        tables.insert(section_key.get().to_string(), (section_key.clone(), section_table.clone()));
       
        section_table.iter().for_each(|(k, _)| {
            let (recursive_item_key, recursive_item) = section_table.get_key_value(k).unwrap();
            println!("{:?}", recursive_item);
            
            if let Some(table) = recursive_item.as_table() {
                println!("item {} {:?}", recursive_item_key, table.position());

                let entry = recursive_tables.entry(section_key.get().to_string()).or_insert(vec![]);                
                entry.push(recursive_item_key.clone());
            } else {
                panic!();
            }
        });        
    });

    println!("Clearing, and readding");

    toml_document.clear();

    let mut idx = 0;
        
    println!("{:?}", all_sections);
    for section in all_sections {
        if let Some((section_key, section_table)) = tables.get_mut(&section) {
                    
            if section_table.position().is_some() {
                println!("section {} {:?} -> {idx}", section_key.get(), section_table.position());
                section_table.set_position(idx);
                idx += 1;
                toml_document.insert(section_key.get(), Item::Table(section_table.clone()));
            }
                       
            if let Some((mut k,v)) = toml_document.get_key_value_mut(section_key.get()) {
                k.decor_mut().set_prefix(section_key.decor().prefix().unwrap().to_string());
                k.decor_mut().set_suffix(section_key.decor().suffix().unwrap().to_string());
            }
         
            if let Some(recursive_table_variants) = recursive_tables.remove(&section) {
                for key in recursive_table_variants {  
                    let has_no_position = section_table.position().is_none();
                    let table = section_table.get_mut(key.get()).unwrap().as_table_mut().unwrap();
                                        
                    if has_no_position {
                        println!("item {} {:?} -> {}", key.get(), table.position(),idx);
                        table.set_position(idx);
                        idx += 1;
                        toml_document.insert(section_key.get(), Item::Table(section_table.clone()));
                    }

                    if let Some((mut k,_)) = toml_document.get_key_value_mut(key.get()) {
                        k.decor_mut().set_prefix(key.decor().prefix().unwrap().to_string());
                        k.decor_mut().set_suffix(key.decor().suffix().unwrap().to_string());
                    }
                }
            }
        }
    }

    println!("=====\n{}", toml_document.to_string());

    assert_eq!(toml_document.to_string(), after);
}

pub fn debug_table(table: &Table) {
    println!("{}", table.to_string());
}
