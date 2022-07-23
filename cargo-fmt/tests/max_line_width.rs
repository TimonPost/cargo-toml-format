use cargo_fmt::{cargo_toml::CargoToml, toml_config::TomlFormatConfig};

const BEFORE_WHITE_SPACE_STRIP: &str = r#"


[a ]
description="a" # A description of the package.



 description1="b" # b description of the package.
[ b ]

    description="a" # A description of the package.


           [ c]          

description="a" # A description of the package.



[d ]
description="a" # A description of the package.
"#;

const AFTER_WHITE_SPACE_STRIP: &str = r#"[a]
description= ["a"]
description1= ["a", "b"]
description1= ["a", "b", "c"]
description1= ["a", "b", "c", "d"]
description1= ["a", "b", "c", "d", "e"]
description1= ["a", "b", "c", "d", "e", "f"]
description1= ["a", "b", "c", "d", "e", "f", "g"]
"#;

#[test]
fn strip_empty_spaces_and_enforce_new_line() {
    let mut toml = CargoToml::new(BEFORE_WHITE_SPACE_STRIP.to_string()).unwrap();

    let mut config = TomlFormatConfig::new();
    //config.format_toml = Some(true);

    toml.format(config);

    //assert_eq!(toml.toml_document.to_string(), AFTER_WHITE_SPACE_STRIP);
}
