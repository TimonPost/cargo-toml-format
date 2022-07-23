use cargo_fmt::{cargo_toml::CargoToml, toml_config::TomlFormatConfig};

#[test]
fn package_order_sort() {
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

    let mut toml = CargoToml::new(BEFORE.to_string()).unwrap();

    let mut config = TomlFormatConfig::new();
    config.order_package_section = true;

    toml.format(config);

    assert_eq!(toml.toml_document.to_string(), AFTER);
}
