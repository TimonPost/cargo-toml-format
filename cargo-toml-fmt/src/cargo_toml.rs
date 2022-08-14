use toml_edit::{Document, Item};

use crate::formatting::{
    InlineTableWrap, KeyQuoteTrimmer, KeyTrimmer, OrderDependencies, OrderSections,
    OrderTableKeysAlphabetically, TableFormatting, TomlFormatter, WrapArray,
};

use crate::{
    formatting::{AppendLineAfterSection, OrderPackageSection, SectionKeyNameTrimmer},
    toml_config::TomlFormatConfig,
};

pub struct CargoToml {
    pub toml_document: Document,
    pub rules: Vec<(bool, Box<dyn TomlFormatter>)>,
    config: TomlFormatConfig,
}

impl CargoToml {
    /// Crates a in-memory toml definition from the given toml contents.
    pub fn new(toml_contents: String, config: TomlFormatConfig) -> anyhow::Result<Self> {
        let toml_document = toml_contents
            .parse::<Document>()
            .map_err(|e| anyhow::anyhow!("Failed to parse toml. {e}"))?;

        let mut rules: Vec<(bool, Box<dyn TomlFormatter>)> = Vec::new();
        rules.push((config.order_package_section, Box::from(OrderPackageSection)));
        rules.push((config.order_sections, Box::from(OrderSections)));
        rules.push((
            config.order_table_keys,
            Box::from(OrderTableKeysAlphabetically),
        ));

        rules.push((config.trim_keys, Box::from(KeyTrimmer)));
        rules.push((config.trim_section_keys, Box::from(SectionKeyNameTrimmer)));
        rules.push((config.trim_key_quotes, Box::from(KeyQuoteTrimmer)));

        rules.push((
            config.add_newline_after_section,
            Box::from(AppendLineAfterSection),
        ));
        rules.push((config.table_formatting, Box::from(TableFormatting)));

        rules.push((config.wrap_array.is_some(), Box::from(WrapArray)));
        rules.push((
            config.wrap_table.is_some(),
            Box::from(InlineTableWrap::new()),
        ));
        rules.push((
            config.dependency_sorts.is_some(),
            Box::from(OrderDependencies),
        ));

        Ok(Self {
            toml_document,
            rules,
            config,
        })
    }

    pub fn format(&mut self) -> anyhow::Result<()> {
        let mut toml_document = self.toml_document.clone();

        for (enabled, rule) in self.rules.iter_mut() {
            if *enabled {
                rule.visit_document(&mut toml_document, &self.config)
                    .unwrap();
            }
        }

        self.toml_document = toml_document;

        Ok(())
    }

    pub fn dependencies(&mut self) -> anyhow::Result<&mut Item> {
        self.toml_document
            .get_mut("dependencies")
            .ok_or_else(|| anyhow::anyhow!("Dependencies tag not found in toml document"))
    }
}
