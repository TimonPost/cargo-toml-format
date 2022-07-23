use cargo_fmt::{cargo_toml::CargoToml, package_order::TomlSort, toml_config::TomlFormatConfig};

#[test]
fn sort_alphabetically() {
    const BEFORE: &str = r#"
    [dependencies]
    a = "0.1"
    d = "0.4"
    e = "0.5"
    b = "0.2"
    c = "0.3"
    f = "0.6"
    g = "0.7"
    "#;

    const AFTER: &str = r#"
    [dependencies]
    a = "0.1"
    b = "0.2"
    c = "0.3"
    d = "0.4"
    e = "0.5"
    f = "0.6"
    g = "0.7"
    "#;

    let mut toml = CargoToml::new(BEFORE.to_string()).unwrap();

    let mut config = TomlFormatConfig::new();
    config.dependency_sorts = Some(vec![TomlSort::Alphabetical]);

    toml.format(config);

    assert_eq!(toml.toml_document.to_string(), AFTER);
}

#[test]
fn sort_length() {
    const BEFORE: &str = r#"
    [dependencies]
    a = "a"
    g = { test = 5.0 }
    d = { version = "aa"}
    f = { test = true}
    b = { version = "aaaa", default_features=false}
    c = { test = 1 }
    e = { version = "aaa", features = ["a"] }
    "#;

    const AFTER: &str = r#"
    [dependencies]
    a = "a"
    c = { test = 1 }
    g = { test = 5.0 }
    f = { test = true}
    d = { version = "aa"}
    e = { version = "aaa", features = ["a"] }
    b = { version = "aaaa", default_features=false}
    "#;

    let mut toml = CargoToml::new(BEFORE.to_string()).unwrap();

    let mut config = TomlFormatConfig::new();
    config.dependency_sorts = Some(vec![TomlSort::Length]);

    toml.format(config);

    assert_eq!(toml.toml_document.to_string(), AFTER);
}
