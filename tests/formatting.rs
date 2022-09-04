use cargo_fmt::{cargo_toml::CargoToml, package_order::TomlSort, toml_config::TomlFormatConfig};

#[test]
fn adds_new_line_after_section() {
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

    let mut toml = CargoToml::new(BEFORE.to_string(), config).unwrap();

    println!("{}", toml.toml_document.to_string());
    toml.format();
    println!("{}", toml.toml_document.to_string());

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

    let mut toml = CargoToml::new(BEFORE.to_string(), config).unwrap();

    toml.format();

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

    let mut toml = CargoToml::new(BEFORE.to_string(), config).unwrap();

    toml.format();

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

    let mut toml = CargoToml::new(BEFORE.to_string(), config).unwrap();

    toml.format();

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

    let mut toml = CargoToml::new(BEFORE.to_string(), config).unwrap();

    println!("{}", toml.toml_document.to_string());

    toml.format();

    println!("{}", toml.toml_document.to_string());

    assert_eq!(toml.toml_document.to_string(), AFTER);
}

// #[test]
// fn format_cargo_toml() {
//     const BEFORE: &str = r#"
//     [package]
//     name = "tokio"
//     # When releasing to crates.io:
//     # - Remove path dependencies
//     # - Update doc url
//     #   - README.md
//     # - Update CHANGELOG.md.
//     # - Create "v1.0.x" git tag.
//     version = "1.20.1"
//     edition = "2018"
//     rust-version = "1.49"
//     authors = ["Tokio Contributors <team@tokio.rs>"]
//     license = "MIT"
//     readme = "README.md"
//     repository = "https://github.com/tokio-rs/tokio"
//     homepage = "https://tokio.rs"
//     description = """
//     An event-driven, non-blocking I/O platform for writing asynchronous I/O
//     backed applications.
//     """
//     categories = ["asynchronous", "network-programming"]
//     keywords = ["io", "async", "non-blocking", "futures"]

//     [features]
//     # Include nothing by default
//     default = []

//     # enable everything
//     full = [
//     "fs",
//     "io-util",
//     "io-std",
//     "macros",
//     "net",
//     "parking_lot",
//     "process",
//     "rt",
//     "rt-multi-thread",
//     "signal",
//     "sync",
//     "time",
//     ]

//     fs = []
//     io-util = ["memchr", "bytes"]
//     # stdin, stdout, stderr
//     io-std = []
//     macros = ["tokio-macros"]
//     net = [
//     "libc",
//     "mio/os-poll",
//     "mio/os-ext",
//     "mio/net",
//     "winapi/fileapi",
//     "winapi/handleapi",
//     "winapi/namedpipeapi",
//     "winapi/winbase",
//     "winapi/winnt",
//     "winapi/minwindef",
//     "winapi/accctrl",
//     "winapi/aclapi"
//     ]
//     process = [
//     "bytes",
//     "once_cell",
//     "libc",
//     "mio/os-poll",
//     "mio/os-ext",
//     "mio/net",
//     "signal-hook-registry",
//     "winapi/handleapi",
//     "winapi/minwindef",
//     "winapi/processthreadsapi",
//     "winapi/threadpoollegacyapiset",
//     "winapi/winbase",
//     "winapi/winnt",
//     ]
//     # Includes basic task execution capabilities
//     rt = ["once_cell"]
//     rt-multi-thread = [
//     "num_cpus",
//     "rt",
//     ]
//     signal = [
//     "once_cell",
//     "libc",
//     "mio/os-poll",
//     "mio/net",
//     "mio/os-ext",
//     "signal-hook-registry",
//     "winapi/consoleapi",
//     "winapi/wincon",
//     "winapi/minwindef",
//     ]
//     sync = []
//     test-util = ["rt", "sync", "time"]
//     time = []

//     # Technically, removing this is a breaking change even though it only ever did
//     # anything with the unstable flag on. It is probably safe to get rid of it after
//     # a few releases.
//     stats = []
//     "#;

//     const AFTER: &str = r#"
//     [package]
//     name = "tokio"
//     # When releasing to crates.io:
//     # - Remove path dependencies
//     # - Update doc url
//     #   - README.md
//     # - Update CHANGELOG.md.
//     # - Create "v1.0.x" git tag.
//     version = "1.20.1"
//     authors = ["Tokio Contributors <team@tokio.rs>"]
//     edition = "2018"
//     rust-version = "1.49"
//     description = """
//     An event-driven, non-blocking I/O platform for writing asynchronous I/O
//     backed applications.
//     """
//     readme = "README.md"
//     homepage = "https://tokio.rs"
//     repository = "https://github.com/tokio-rs/tokio"
//     license = "MIT"
//     keywords = ["io", "async", "non-blocking", "futures"]
//     categories = ["asynchronous", "network-programming"]

//     [features]
//     # Include nothing by default
//     default = []

//     # enable everything
//     full = [
//     "fs",
//     "io-util",
//     "io-std",
//     "macros",
//     "net",
//     "parking_lot",
//     "process",
//     "rt",
//     "rt-multi-thread",
//     "signal",
//     "sync",
//     "time",
//     ]

//     fs = []
//     io-util = ["memchr", "bytes"]
//     # stdin, stdout, stderr
//     io-std = []
//     macros = ["tokio-macros"]
//     net = [
//     "libc",
//     "mio/os-poll",
//     "mio/os-ext",
//     "mio/net",
//     "winapi/fileapi",
//     "winapi/handleapi",
//     "winapi/namedpipeapi",
//     "winapi/winbase",
//     "winapi/winnt",
//     "winapi/minwindef",
//     "winapi/accctrl",
//     "winapi/aclapi"
//     ]
//     process = [
//     "bytes",
//     "once_cell",
//     "libc",
//     "mio/os-poll",
//     "mio/os-ext",
//     "mio/net",
//     "signal-hook-registry",
//     "winapi/handleapi",
//     "winapi/minwindef",
//     "winapi/processthreadsapi",
//     "winapi/threadpoollegacyapiset",
//     "winapi/winbase",
//     "winapi/winnt",
//     ]
//     # Includes basic task execution capabilities
//     rt = ["once_cell"]
//     rt-multi-thread = [
//     "num_cpus",
//     "rt",
//     ]
//     signal = [
//     "once_cell",
//     "libc",
//     "mio/os-poll",
//     "mio/net",
//     "mio/os-ext",
//     "signal-hook-registry",
//     "winapi/consoleapi",
//     "winapi/wincon",
//     "winapi/minwindef",
//     ]
//     # Technically, removing this is a breaking change even though it only ever did
//     # anything with the unstable flag on. It is probably safe to get rid of it after
//     # a few releases.
//     stats = []
//     sync = []
//     test-util = ["rt", "sync", "time"]
//     time = []
//     "#;
//     let mut config = TomlFormatConfig::new();
//     config.order_package_section = true;

//     let mut toml = CargoToml::new(BEFORE.to_string(), config).unwrap();

//     //config.so = Some(vec![TomlSort::Alphabetical]);

//     // config.space_between_assignment = true;
//     // config.trim_section_keys= true;
//     // config.trim_keys= true;
//     // config.add_newline_after_section= true;
//     // config.trim_key_quotes= true;

//     println!("{}", toml.toml_document.to_string());
//     toml.format();
//     println!("{}", toml.toml_document.to_string());

//     assert_eq!(toml.toml_document.to_string(), AFTER);
// }
