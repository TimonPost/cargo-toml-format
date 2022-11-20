use cargo_toml_fmt::{
    cargo_toml::CargoToml,
    package_order::{TomlSection, TomlSort},
    toml_config::TomlFormatConfig,
};
use strum::VariantNames;

#[test]
fn order_sections_to_manifest_spec() {
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
    [bin]
    "#;

    let mut config = TomlFormatConfig::new();
    config.order_sections = true;

    let mut toml = CargoToml::from_config(BEFORE.to_string(), config).unwrap();

    toml.format().unwrap();

    let mut iter = toml.toml_document.iter();

    for variant in TomlSection::VARIANTS.iter() {
        let (key, _) = iter.next().unwrap();
        assert_eq!(key, *variant)
    }
}

#[test]
fn order_package_section_to_manifest_spec() {
    const BEFORE: &str = r#"
    [package]
    authors = ["a"] # The authors of the package.
    exclude = "a" # Files to exclude when publishing.
    name = "a" # The name of the package.
    readme = "a" # Path to the package's README file.
    autobenches = "a" # Disables bench auto discovery.
    license = "a" # The package license.
    categories = ["a"] # Categories of the package.
    build = "a" # Path to the package build script.
    autoexamples = "a" # Disables example auto discovery.
    version = "a" # The version of the package.
    homepage = "a" # URL of the package homepage.
    autotests = "a" # Disables test auto discovery.
    rust-version = "a" # The minimal supported Rust version.
    workspace = "a" # Path to the workspace for the package.
    links = "a" # Name of the native library the package links with.
    autobins = "a" # Disables binary auto discovery.
    metadata = "a" # Extra settings for external tools.
    edition = "a" # The Rust edition.
    keywords = ["a"] # Keywords for the package.
    include = "a" # Files to include when publishing.
    documentation = "a" # URL of the package documentation.
    description = "a" # A description of the package.
    default-run = "a" # The default binary to run by cargo run.
    repository = "a" # URL of the package source repository.
    resolver = "a" # Sets the dependency resolver to use.
    license-file = "a" # Path to the text of the license.
    publish = "a" # Can be used to prevent publishing the package.
    "#;

    const AFTER: &str = r#"
    [package]
    name = "a" # The name of the package.
    version = "a" # The version of the package.
    authors = ["a"] # The authors of the package.
    edition = "a" # The Rust edition.
    rust-version = "a" # The minimal supported Rust version.
    description = "a" # A description of the package.
    documentation = "a" # URL of the package documentation.
    readme = "a" # Path to the package's README file.
    homepage = "a" # URL of the package homepage.
    repository = "a" # URL of the package source repository.
    license = "a" # The package license.
    license-file = "a" # Path to the text of the license.
    keywords = ["a"] # Keywords for the package.
    categories = ["a"] # Categories of the package.
    workspace = "a" # Path to the workspace for the package.
    build = "a" # Path to the package build script.
    links = "a" # Name of the native library the package links with.
    exclude = "a" # Files to exclude when publishing.
    include = "a" # Files to include when publishing.
    publish = "a" # Can be used to prevent publishing the package.
    metadata = "a" # Extra settings for external tools.
    default-run = "a" # The default binary to run by cargo run.
    autobins = "a" # Disables binary auto discovery.
    autoexamples = "a" # Disables example auto discovery.
    autotests = "a" # Disables test auto discovery.
    autobenches = "a" # Disables bench auto discovery.
    resolver = "a" # Sets the dependency resolver to use.
    "#;

    let mut config = TomlFormatConfig::new();
    config.order_package_section = true;

    let mut toml = CargoToml::from_config(BEFORE.to_string(), config).unwrap();

    toml.format().unwrap();

    assert_eq!(toml.toml_document.to_string(), AFTER);
}

#[test]
fn sort_dependencies_alphabetically() {
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

    let mut config = TomlFormatConfig::new();
    config.order_dependencies_alphabetically = true;

    let mut toml = CargoToml::from_config(BEFORE.to_string(), config).unwrap();

    toml.format().unwrap();

    assert_eq!(toml.toml_document.to_string(), AFTER);
}

// #[test]
// fn sort_dependencies_by_length() {
//     const BEFORE: &str = r#"
//     [dependencies]
//     a = "a"
//     g = { test = 5.0 }
//     d = { version = "aa"}
//     f = { test = true}
//     b = { version = "aaaa", default_features=false}
//     c = { test = 1 }
//     e = { version = "aaa", features = ["a"] }
//     "#;

//     const AFTER: &str = r#"
//     [dependencies]
//     a = "a"
//     c = { test = 1 }
//     g = { test = 5.0 }
//     f = { test = true}
//     d = { version = "aa"}
//     e = { version = "aaa", features = ["a"] }
//     b = { version = "aaaa", default_features=false}
//     "#;

//     let mut config = TomlFormatConfig::new();
//     config.order_dependencies_alphabetically =true;

//     let mut toml = CargoToml::from_config(BEFORE.to_string(), config).unwrap();

//     toml.format().unwrap();

//     assert_eq!(toml.toml_document.to_string(), AFTER);
// }

#[test]
fn order_table_keys_alphabetically() {
    const BEFORE: &str = r#"
    [a]
    b={a=1}
    a="a"
    b1a={b=[{b=1,a=1}],a=[{a=1,b=1}]}
    c={b={b=1,a=1}}
    b1={b=1,a=2}
    "#;

    const AFTER: &str = r#"
    [a]
    a="a"
    b={a=1}
    b1={a=2,b=1}
    b1a={a=[{a=1,b=1}],b=[{a=1,b=1}]}
    c={b={a=1,b=1}}
    "#;

    let mut config = TomlFormatConfig::new();
    config.order_table_keys_alphabetically = true;

    let mut toml = CargoToml::from_config(BEFORE.to_string(), config).unwrap();

    toml.format().unwrap();

    assert_eq!(toml.toml_document.to_string(), AFTER);
}

#[test]
fn order_table_keys_alphabetically_and_grouped() {
    const BEFORE: &str = r#"
[a]
e={b=[{b=1,a=1}],a=[{a=1,b=1}]}
c={b={b=1,a=1}}
a={b=1,a=2}

d={b=[{b=1,a=1}],a=[{a=1,b=1}]}
b={b={b=1,a=1}}
f={b=1,a=2}
    "#;

    const AFTER: &str = r#"
[a]
a={b=1,a=2}
c={b={b=1,a=1}}
e={b=[{b=1,a=1}],a=[{a=1,b=1}]}

b={b={b=1,a=1}}
d={b=[{b=1,a=1}],a=[{a=1,b=1}]}
f={b=1,a=2}
    "#;

    let mut config = TomlFormatConfig::new();
    config.order_section_keys_by_group_alphabetically = true;

    let mut toml = CargoToml::from_config(BEFORE.to_string(), config).unwrap();
    println!("{}", toml.toml_document.to_string());
    toml.format().unwrap();
    println!("{}", toml.toml_document.to_string());
    assert_eq!(toml.toml_document.to_string(), AFTER);
}

#[test]
fn order_table_keys_alphabetically_and_grouped_by_comment() {
    const BEFORE: &str = r#"
[a]
e={b=[{b=1,a=1}],a=[{a=1,b=1}]}
c={b={b=1,a=1}}
a={b=1,a=2}

# section 2
d={b=[{b=1,a=1}],a=[{a=1,b=1}]}
b={b={b=1,a=1}}
f={b=1,a=2}

# section 3

k={b=1,a=2}
i={b=[{b=1,a=1}],a=[{a=1,b=1}]}
g={b={b=1,a=1}}
    "#;

    const AFTER: &str = r#"
[a]
a={b=1,a=2}
c={b={b=1,a=1}}
e={b=[{b=1,a=1}],a=[{a=1,b=1}]}

# section 2
b={b={b=1,a=1}}
d={b=[{b=1,a=1}],a=[{a=1,b=1}]}
f={b=1,a=2}

# section 3

g={b={b=1,a=1}}
i={b=[{b=1,a=1}],a=[{a=1,b=1}]}
k={b=1,a=2}
    "#;

    let mut config = TomlFormatConfig::new();
    config.order_section_keys_by_group_alphabetically = true;

    let mut toml = CargoToml::from_config(BEFORE.to_string(), config).unwrap();
    println!("{}", toml.toml_document.to_string());
    toml.format().unwrap();
    println!("{}", toml.toml_document.to_string());
    assert_eq!(toml.toml_document.to_string(), AFTER);
}
