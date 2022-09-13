use cargo_toml_fmt::{cargo_toml::CargoToml, toml_config::TomlFormatConfig};

const TOML_BEFORE: &str = r#"

[ package ] 


"readme"=         "test"
authors = [""]
exclude     =        [""]
edition  = "1"
categories =            [""]


links= "linker"
default-run=    "cargo run"
include = [         ""  ]

autobins=      false

license-file    = "path"
        resolver = "resolver x"

description = "some description"


autobenches=    false
version  =        "0.0.0"       
publish = false
        documentation = "docs"              

rust-version = "1.6.3.0"
repository = "repo"
metadata=           "some metadata"
keywords= ["" ]


license=         "license"
workspace = "workspace"
autoexamples = false                
name = "some-crate"

homepage = "homepage"
    autotests =         false
build    = "path"

[build-dependencies]
cc = "1.0.3"

[[bin]]
name =  "cool-tool"
test = false
bench = false

[patch.crates-io]
foo = {         git = "https://github.com/example/foo" }
bar = { path    =      "my/local/bar" }

[features]
"default"  = [     "fancy-feature" ]
    fancy-feature   = [         "foo",   "bar"]

[dev-dependencies   ]
tempdir = "0.3"

[   dependencies]
time =      "0.1.12"
some-crate       = {     version =   "1.0", registry =    "crates.io", git = "http://github.com",          branch = "next",  features =  ["foo",         "bar"],  path = "hello_utils" }

[profile.dev]
"opt-level" = 1                      # Use slightly better optimizations.
overflow-checks = false                 # Disable integer overflow checks.          
[[example]]         
        

name = "foo"



crate-type = [          "staticlib"               ]


[[bin]]

name = "cool-tool-2"
test = false
bench = false

[workspace]
members = ["member1", "path/to/member2", "crates/*"]
exclude = ["crates/foo", "path/to/other"]

[replace]
"foo:0.1.0" = { git = 'https://github.com/example/foo' }
"bar:1.0.2" = { path = 'my/local/bar' }

[lib] 
crate-type = ["cdylib"]
bench = false
"#;

const TOML_AFTER: &str = r#"[package]
name = "some-crate"
version = "0.0.0"
authors = [""]
edition = "1"
rust-version = "1.6.3.0"
description = "some description"
documentation = "docs"
readme = "test"
homepage = "homepage"
repository = "repo"
license = "license"
license-file = "path"
keywords = [""]
categories = [""]
workspace = "workspace"
build = "path"
links = "linker"
exclude = [""]
include = [""]
publish = false
metadata = "some metadata"
default-run = "cargo run"
autobins = false
autoexamples = false
autotests = false
autobenches = false
resolver = "resolver x"

[lib]
bench = false
crate-type = ["cdylib"]

[[bin]]
bench = false
name = "cool-tool"
test = false

[[bin]]
bench = false
name = "cool-tool-2"
test = false

[[example]]
crate-type = ["staticlib"]
name = "foo"

[dependencies]
time = "0.1.12"

[dependencies.some-crate]
branch = "next"
features = ["foo", "bar"]
git = "http://github.com"
path = "hello_utils"
registry = "crates.io"
version = "1.0"

[dev-dependencies]
tempdir = "0.3"

[build-dependencies]
cc = "1.0.3"

[features]
default = ["fancy-feature"]
fancy-feature = ["foo", "bar"]

[patch.crates-io]
bar = { path = "my/local/bar" }
foo = { git = "https://github.com/example/foo" }

[replace]
"bar:1.0.2" = { path = 'my/local/bar' }

[replace."foo:0.1.0"]
git = 'https://github.com/example/foo'

[profile.dev]
opt-level = 1 # Use slightly better optimizations.
overflow-checks = false # Disable integer overflow checks.

[workspace]
exclude = ["crates/foo", "path/to/other"]
members = [
    "member1",
    "path/to/member2",
    "crates/*"
]
"#;

// [target.'cfg(windows)'.dependencies]
// winhttp = "0.4.0"

// [target.'cfg(unix)'.dependencies]
// openssl = "1.0.1"

// [target.'cfg(target_arch = "x86")'.dependencies]
// native-i686 = { path = "native/i686" }

// [target.'cfg(target_arch = "x86_64")'.dependencies]
// native-x86_64 = { path = "native/x86_64" }

// [target.x86_64-pc-windows-gnu.dependencies]
// winhttp = "0.4.0"

// [target.i686-unknown-linux-gnu.dependencies]
// openssl = "1.0.1"

// [target.bar.dependencies]
// winhttp = "0.4.0"

// [target.my-special-i686-platform.dependencies]
// openssl = "1.0.1"
// native = { path = "native/i686" }

// [target.'cfg(unix)'.dev-dependencies]
// mio = "0.0.1"

// [target.'cfg(unix)'.build-dependencies]
// cc = "1.0.3"

#[test]
fn format_toml() {
    let mut toml = CargoToml::default(TOML_BEFORE.to_string()).unwrap();

    println!("{}", toml.toml_document.to_string());
    toml.format().unwrap();
    println!("{}", toml.toml_document.to_string());
    assert_eq!(toml.toml_document.to_string(), TOML_AFTER);
}
