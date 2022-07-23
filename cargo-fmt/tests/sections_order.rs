use cargo_fmt::{cargo_toml::CargoToml, toml_config::TomlFormatConfig};

#[test]
fn orders_sections() {
    const BEFORE: &str = r#"
    [package.metadata]
    
    [bins]

    [cargo-features]

    [target]

    [dev-dependencies]

    [features]

    [package]

    [profile]

    [badges]

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

    const AFTER: &str = r#"
    [package]
    
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

    toml.format(config);

    assert_eq!(toml.toml_document.to_string(), AFTER);
}
