use cargo_toml_fmt::{cargo_toml::CargoToml, toml_config::TomlFormatConfig};

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

    let mut config = TomlFormatConfig::new();
    config.trim_section_key_names = true;

    let mut toml = CargoToml::from_config(BEFORE.to_string(), config).unwrap();

    toml.format().unwrap();

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

    let mut config = TomlFormatConfig::new();
    config.trim_section_item_keys = true;

    let mut toml = CargoToml::from_config(BEFORE.to_string(), config).unwrap();

    toml.format().unwrap();

    assert_eq!(toml.toml_document.to_string(), AFTER);
}

#[test]
fn trims_quotes_from_keys() {
    const BEFORE: &str = r#"[a]
    [b]
    "a"="a"
    [c]
    b={"a"={"a"="b"}} 
    "#;

    const AFTER: &str = r#"[a]
    [b]
    a="a"
    [c]
    b={a={a="b"}} 
    "#;

    let mut config = TomlFormatConfig::new();
    config.trim_quotes_table_keys = true;

    let mut toml = CargoToml::from_config(BEFORE.to_string(), config).unwrap();

    toml.format().unwrap();

    assert_eq!(toml.toml_document.to_string(), AFTER);
}

#[test]
fn trimming_around_comments() {
    const BEFORE: &str = r#"[a] 
    a = { a = "b" } # A description of the package.
    b = { a = "b" }       # A description of the package.        
    c = "b"   #    A description of the package.
    d = []  # A description of the package.
    "#;

    const AFTER: &str = r#"[a]
a = { a = "b" } # A description of the package.
b = { a = "b" } # A description of the package.
c = "b" #    A description of the package.
d = [] # A description of the package.
    "#;

    let mut config = TomlFormatConfig::new();
    config.table_formatting = true;

    let mut toml = CargoToml::from_config(BEFORE.to_string(), config).unwrap();
    println!("{}", toml.toml_document.to_string());
    toml.format().unwrap();
    println!("{}", toml.toml_document.to_string());
    assert_eq!(toml.toml_document.to_string(), AFTER);
}

#[test]
fn comments_stress_test() {
    const BEFORE: &str = r#"[a] 
    # comment 1
    a = { a = "b" }  # comment 2


    # comment 2

    b = false
    # comment 1

    # comment 3
c = [""]

    d = [""] # comment 4
    [b]
    # comment 1
    a = { a = "b" }  # comment 2


    # comment 2

    b = false
    # comment 1

    # comment 3
c = [""]

    d = [""] # comment 4
    "#;

    const AFTER: &str = r#"[a]
# comment 1
a = { a = "b" } # comment 2
# comment 2
b = false
# comment 1
# comment 3
c = [""]
d = [""] # comment 4
[b]
# comment 1
a = { a = "b" } # comment 2
# comment 2
b = false
# comment 1
# comment 3
c = [""]
d = [""] # comment 4
    "#;

    let mut config = TomlFormatConfig::new();
    config.table_formatting = true;

    let mut toml = CargoToml::from_config(BEFORE.to_string(), config).unwrap();

    println!("{}", toml.toml_document.to_string());

    toml.format().unwrap();

    println!("{}", toml.toml_document.to_string());

    assert_eq!(toml.toml_document.to_string(), AFTER);
}
