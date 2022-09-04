use cargo_fmt::{cargo_toml::CargoToml, toml_config::TomlFormatConfig};

const CROSSTERM_TOML_BEFORE: &str = r#"[ package ]
name = "crossterm"
version = "0.24.0"
authors = ["T. Post"]
documentation = "https://docs.rs/crossterm/"
description = "A crossplatform terminal library for manipulating terminals."
repository = "https://github.com/crossterm-rs/crossterm"
license = "MIT"
keywords = ["event", "color", "cli", "input", "terminal"]
edition = "2021"
exclude = ["target", "Cargo.lock"]
readme = "README.md"
categories = ["command-line-interface", "command-line-utilities"]"#;

const CROSSTERM_TOML_AFTER: &str = r#"[package]
name = "crossterm"
version = "0.24.0"
authors = ["T. Post"]
edition = "2021"
description = "A crossplatform terminal library for manipulating terminals."
documentation = "https://docs.rs/crossterm/"
readme = "README.md"
repository = "https://github.com/crossterm-rs/crossterm"
license = "MIT"
keywords = ["event", "color", "cli", "input", "terminal"]
categories = ["command-line-interface", "command-line-utilities"]
exclude = ["target", "Cargo.lock"]
"#;

#[test]
fn format_crossterm_toml() {
    let mut config = TomlFormatConfig::new();
    config.order_package_section = true;
    config.trim_keys = true;
    config.table_formatting = true;
    config.trim_section_keys = true;

    let mut toml = CargoToml::new(CROSSTERM_TOML_BEFORE.to_string(), config).unwrap();

    println!("{}", toml.toml_document.to_string());
    toml.format();
    println!("{}", toml.toml_document.to_string());
    assert_eq!(toml.toml_document.to_string(), CROSSTERM_TOML_AFTER);
}
