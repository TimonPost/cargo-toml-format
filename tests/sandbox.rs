use std::io::Lines;

use cargo_toml_fmt::{cargo_toml::CargoToml, toml_config::TomlFormatConfig};
use toml_edit::Document;

// iterate prefix lines. 
// find comments in prefix Lines
// preserve enters within comments.
// preserve enters upto comments. 

// Key/value pairs within a table only have prefixes
// Sections can have prefixes and suffixes.

//- Items have no spaces in between, but comments are allowed.

#[test]
fn mytest() {

    let toml_1 = r#"
    # test 1
    [a]
    adep = "0.4.0"#;

    let toml_2 = r#"    
    [a]
    adep = "0.4.0
    # test 1"#;

    let toml_3 = r#"      
[b]
name = "b"


# test 1
[a]
# test 2
adep = "0.4.0"

# test 3
    "#;



    // let mut config = TomlFormatConfig::new();
    // config.order_sections = true;

    // let mut cargo = CargoToml::from_config(toml.to_string(), config).unwrap();

    // cargo.format().unwrap();

    // println!("{}", cargo.toml_document);

    let mut document = toml_3.parse::<Document>();
    let mut document = document.unwrap();
    let mut table = document.as_table_mut();

    for (key, val) in &mut table.iter_mut() {
        match val {
            toml_edit::Item::None => todo!(),
            toml_edit::Item::Value(_) => todo!(),
            toml_edit::Item::Table(subtable) => {
                println!("table {key} {:?} {:?}", subtable.decor().prefix(), subtable.decor().suffix());

                for (key, val) in &mut subtable.iter_mut() {
                    match val {
                        toml_edit::Item::None => todo!(),
                        toml_edit::Item::Value(value) => {
                            println!("sub {key} {:?} {:?}", value.decor().prefix(), value.decor().suffix());
                        },
                        toml_edit::Item::Table(subtable) => {
                           
                        },
                        toml_edit::Item::ArrayOfTables(_) => todo!(),
                    }
                }

            },
            toml_edit::Item::ArrayOfTables(_) => todo!(),
        }
    }

    println!("{:?}", table);
}
