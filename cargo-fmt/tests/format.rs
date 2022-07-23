use cargo_fmt::{cargo_toml::CargoToml, toml_config::TomlFormatConfig};

#[test]
fn adds_new_line_after_section() {
    const BEFORE: &str = r#"[a]
[b]
a="a" # A description of the package.
[c]
b="b" # A description of the package.
c="c" # A description of the package.
[d]
d="d" # A description of the package.
"#;

    const AFTER: &str = r#"[a]

[b]
a="a" # A description of the package.

[c]
b="b" # A description of the package.
c="c" # A description of the package.

[d]
d="d" # A description of the package.
"#;

    let mut toml = CargoToml::new(BEFORE.to_string()).unwrap();

    let mut config = TomlFormatConfig::new();
    config.add_newline_after_section = true;

    toml.format(config);

    assert_eq!(toml.toml_document.to_string(), AFTER);
}

#[test]
fn trims_empty_spaces_section_keys() {
    const BEFORE: &str = r#"[ a]
[b ]
a="a" # A description of the package.
[ c ]
b="b" # A description of the package.
"#;

    const AFTER: &str = r#"[a]
[b]
a="a" # A description of the package.
[c]
b="b" # A description of the package.
"#;

    let mut toml = CargoToml::new(BEFORE.to_string()).unwrap();

    let mut config = TomlFormatConfig::new();
    config.trim_section_keys = true;

    toml.format(config);

    assert_eq!(toml.toml_document.to_string(), AFTER);
}

#[test]
fn trims_empty_spaces_keys() {
    const BEFORE: &str = r#"[a]
[b]

    a="a" # A description of the package.
[c]
    b="b" # A description of the package.
        c="c" # A description of the package.


[d] 
    
    d="d" # A description of the package.
"#;

    const AFTER: &str = r#"[a]
[b]
a="a" # A description of the package.
[c]
b="b" # A description of the package.
c="c" # A description of the package.
[d]
d="d" # A description of the package.
"#;

    let mut toml = CargoToml::new(BEFORE.to_string()).unwrap();

    let mut config = TomlFormatConfig::new();
    config.trim_keys = true;

    toml.format(config);

    assert_eq!(toml.toml_document.to_string(), AFTER);
}

#[test]
fn trims_quotes_from_keys() {
    const BEFORE_3: &str = r#"[a]
    [b]
    "a"="a" # A description of the package.
    [c]
    "b"="b" # A description of the package.
    "#;

    const AFTER_3: &str = r#"[a]
    [b]
    a="a" # A description of the package.
    [c]
    b="b" # A description of the package.
    "#;

    let mut toml = CargoToml::new(BEFORE_3.to_string()).unwrap();

    let mut config = TomlFormatConfig::new();
    config.trim_key_quotes = true;

    toml.format(config);

    assert_eq!(toml.toml_document.to_string(), AFTER_3);
}

#[test]
fn space_between_assignments() {
    const BEFORE_3: &str = r#"[a]
    a="a" # A description of the package.
    a= "b" # A description of the package.
    b ="b" # A description of the package.
    c = "b" # A description of the package.
    d = {a="b"} # A description of the package.
    e = {a= "b"} # A description of the package.
    f = {a ="b"} # A description of the package.
    g = {a = "b"} # A description of the package.
    e = [{a="b"}] # A description of the package.
    e = [{a= "b"}] # A description of the package.
    e = [{a ="b"}] # A description of the package.
    e = [{a = "b"}] # A description of the package.
    "#;

    const AFTER_3: &str = r#"[a]
    a ="a" # A description of the package.
    a = "b" # A description of the package.
    b = "b" # A description of the package.
    c = "b" # A description of the package.
    d = {a = "b"} # A description of the package.
    e = {a = "b"} # A description of the package.
    f = {a = "b"} # A description of the package.
    g = {a = "b"} # A description of the package.
    e = [{a = "b"}] # A description of the package.
    e = [{a = "b"}] # A description of the package.
    e = [{a = "b"}] # A description of the package.
    e = [{a = "b"}] # A description of the package.
    "#;

    let mut toml = CargoToml::new(BEFORE_3.to_string()).unwrap();

    let mut config = TomlFormatConfig::new();
    config.space_between_assignment = true;

    toml.format(config);

    assert_eq!(toml.toml_document.to_string(), AFTER_3);
}
