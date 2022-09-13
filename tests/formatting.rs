use cargo_toml_fmt::{cargo_toml::CargoToml, toml_config::TomlFormatConfig};

#[test]
fn append_new_line_after_section() {
    const BEFORE: &str = r#"[package]
[b]
a="a" # A description of the package.
[c]
b="b" # A description of the package.
c="c" # A description of the package.
[d]
d="d" # A description of the package.
"#;

    const AFTER: &str = r#"[package]

[b]
a="a" # A description of the package.

[c]
b="b" # A description of the package.
c="c" # A description of the package.

[d]
d="d" # A description of the package.
"#;

    let mut config = TomlFormatConfig::new();
    config.add_newline_after_section = true;

    let mut toml = CargoToml::from_config(BEFORE.to_string(), config).unwrap();

    toml.format().unwrap();

    assert_eq!(toml.toml_document.to_string(), AFTER);
}

#[test]
fn add_spaces_between_value_assignments() {
    const BEFORE: &str = r#"[a]
    a="a"
    b= "b"
    c ="b"
    d = "b"
    "#;

    const AFTER: &str = r#"[a]
a = "a"
b = "b"
c = "b"
d = "b"
    "#;

    let mut config = TomlFormatConfig::new();
    config.table_formatting = true;

    let mut toml = CargoToml::from_config(BEFORE.to_string(), config).unwrap();

    toml.format().unwrap();

    assert_eq!(toml.toml_document.to_string(), AFTER);
}

#[test]
fn adds_spaces_between_table_assignments() {
    const BEFORE: &str = r#"[a] 
    e ={a="b"}  
    f={a= "b"}  
    g ={a ="b"} 
    h ={a = "b"}    # test
"#;

    const AFTER: &str = r#"[a]
e = { a = "b" }
f = { a = "b" }
g = { a = "b" }
h = { a = "b" } # test
"#;

    let mut config = TomlFormatConfig::new();
    config.table_formatting = true;

    let mut toml = CargoToml::from_config(BEFORE.to_string(), config).unwrap();

    toml.format().unwrap();

    assert_eq!(toml.toml_document.to_string(), AFTER);
}

#[test]
fn wrap_line_and_format_array() {
    const BEFORE: &str = r#"[a]
[b]
a=["a"]
b=["a","b"]
c=["a","b"]
d=["a","b","c"]
e=["a","b","c","d"]
f=["a","b","c","d","e"]
g=["a","b","c","d","e","f"]
h=["a","b","c","d","e","f","g"]
    "#;

    const AFTER: &str = r#"[a]
[b]
a=["a"]
b=["a", "b"]
c=["a", "b"]
d=[
    "a",
    "b",
    "c"
]
e=[
    "a",
    "b",
    "c",
    "d"
]
f=[
    "a",
    "b",
    "c",
    "d",
    "e"
]
g=[
    "a",
    "b",
    "c",
    "d",
    "e",
    "f"
]
h=[
    "a",
    "b",
    "c",
    "d",
    "e",
    "f",
    "g"
]
    "#;

    let mut config = TomlFormatConfig::new();
    config.wrap_array = Some(15);

    let mut toml = CargoToml::from_config(BEFORE.to_string(), config).unwrap();

    toml.format().unwrap();

    assert_eq!(toml.toml_document.to_string(), AFTER);
}

#[test]
fn wrap_table() {
    const BEFORE: &str = r#"[dependencies]
a={version="0.4.1"}
b={version="0.4.1",path="some_path"}
c={version="0.4.1",path="some_path",features=["a","b"]}
    "#;
    const AFTER: &str = r#"[dependencies]
a={version="0.4.1"}

[dependencies.b]
version = "0.4.1"
path = "some_path"

[dependencies.c]
version = "0.4.1"
path = "some_path"
features = ["a","b"]
    "#;

    let mut config = TomlFormatConfig::new();
    config.wrap_table = Some(20);

    let mut toml = CargoToml::from_config(BEFORE.to_string(), config).unwrap();

    toml.format().unwrap();

    assert_eq!(toml.toml_document.to_string(), AFTER);
}

#[test]
fn comments() {
    const BEFORE: &str = r#"[dependencies]
a={version="0.4.1"}
b={version="0.4.1",path="some_path"}
c={version="0.4.1",path="some_path",features=["a","b"]}
    "#;
    const AFTER: &str = r#"[dependencies]
a={version="0.4.1"}

[dependencies.b]
version = "0.4.1"
path = "some_path"

[dependencies.c]
version = "0.4.1"
path = "some_path"
features = ["a","b"]
    "#;

    let mut config = TomlFormatConfig::new();
    config.wrap_table = Some(20);

    let mut toml = CargoToml::from_config(BEFORE.to_string(), config).unwrap();

    toml.format().unwrap();

    assert_eq!(toml.toml_document.to_string(), AFTER);
}
