use std::{collections::HashMap, hash::Hash};

use cargo_fmt::{package_order::TomlSection, toml_config::TomlFormatConfig, cargo_toml::CargoToml};
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
    let paths = std::fs::read_dir("E:\\programming\\ark\\components\\").unwrap();

    for path in paths {
        let toml_path = path.unwrap().path();
        
        let full_path = format!("{}\\Cargo.toml", toml_path.display());
        println!("Path: {}", full_path);

        if let Ok(toml_contents) = std::fs::read_to_string(full_path.clone()) {
            let after = "[workspace.test]";
            
            let mut config = TomlFormatConfig::new();
            config.order_package_section = true;
        
        
            let mut toml = CargoToml::new(toml_contents, config).unwrap();
        
            toml.format();
        
            std::fs::write(full_path, toml.toml_document.to_string());            
        } else {
            println!("Failed to read file: {}", full_path);
        }
    }   
  
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
