use cargo_toml_fmt::{cargo_toml::CargoToml, toml_config::TomlFormatConfig};
use toml_edit::Table;
use walkdir::WalkDir;

/*
    [workspace] // pos 1

    [package.abc] // pos 2
    a="b"

    [workspace.test] // pos 3
    a="b"

    [package] // pos 4
    b="c"

    [bench.def] // pos 5
    d="1"
*/
fn main() {
    test()
}

fn test() {
    let mut toml_paths = Vec::new();

    for entry in WalkDir::new("E:\\programming\\ark\\")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();

        if f_name.contains("Cargo.toml") {
            println!("{}", entry.path().parent().unwrap().display());
            toml_paths.push(entry.path().display().to_string());
        }
    }

    for toml_path in toml_paths {
        if let Ok(toml_contents) = std::fs::read_to_string(toml_path.clone()) {
            let mut config = TomlFormatConfig::new();
            config.order_section_keys_by_group_alphabetically = true;
            //config.wrap_array = Some(80);

            config.exclude_tables_from_ordering =
                vec!["package".to_string(), "workspace".to_string()];
            config.exclude_keys_from_ordering =
                vec!["ark".to_string(), "ark-".to_string(), "macaw".to_string()];

            let mut toml = CargoToml::from_config(toml_contents, config).unwrap();

            toml.format();

            std::fs::write(
                toml_path,
                toml.toml_document.to_string().trim_end_matches("\r"),
            );
        } else {
            println!("Failed to read file: {}", toml_path);
        }
    }
}

pub fn debug_table(table: &Table) {
    println!("{}", table.to_string());
}
