use cargo_fmt::{cargo_toml::CargoToml, toml_config::TomlFormatConfig, package_order::TomlSection};
use strum::VariantNames;

#[test]
fn orders_sections() {
    const BEFORE: &str = r#"[bins]
    [cargo-features]
    [target]
    [dev-dependencies]
    [features]
    [package]
    [profile]
    [package.metadata]
    [badges]
    [target.btc]
    [lib]
    [dependencies]
    [workspace]
    [example]
    [test]
    [replace]
    [patch]
    [build-dependencies]
    [bench]
    "#;

    const AFTER: &str = r#"[package]
    [package.metadata]
    [lib]
    [bins]
    [example]
    [test]
    [bench]
    [dependencies]
    [dev-dependencies]
    [build-dependencies]
    [target]
    [badges]
    [features]
    [patch]
    [replace]
    [profile]
    [workspace]
    [cargo-features]
    "#;

    let mut toml = CargoToml::new(BEFORE.to_string()).unwrap();

    let mut config = TomlFormatConfig::new();
    config.order_sections = true;

    println!("{}",toml.toml_document.to_string());

    toml.format(config);

    println!("{}",toml.toml_document.to_string());

    let mut iter = toml.toml_document.iter();

    for variant in TomlSection::VARIANTS.iter() {
        let (key, _) = iter.next().unwrap();
        assert_eq!(key, *variant)
    }
}
