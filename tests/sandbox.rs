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

    let mut config = TomlFormatConfig::new();
    config.order_section_keys_by_group_alphabetically = true;

    let mut cargo = CargoToml::from_config(toml.to_string(), config).unwrap();

    cargo.format().unwrap();

    println!("{}", cargo.toml_document);
}
