use cargo_toml_fmt::{cargo_toml::CargoToml, toml_config::TomlFormatConfig};
use toml_edit::Document;

#[test]
fn mytest() {
    let toml = r#"
    [target.'cfg(windows)'.dependencies]
    winhttp = "0.4.0"

    [package]
    name = "some-crate"
    "#;

    let mut config = TomlFormatConfig::new();
    config.order_sections = true;

    let mut cargo = CargoToml::from_config(toml.to_string(), config).unwrap();

    cargo.format().unwrap();

    println!("{}", cargo.toml_document);

    let document = toml.parse::<Document>();
    let document = document.unwrap();
    let table = document.as_table();
}
