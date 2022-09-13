use toml_edit::{Document, Item};

use crate::{
    AppendLineAfterSection, InlineTableWrap, KeyQuoteTrimmer, KeyTrimmer, OrderDependencies,
    OrderPackageSection, OrderSections, OrderTableKeysAlphabetically, SectionKeyNameTrimmer,
    TableFormatting, TomlFormatter, WrapArray,
};

use crate::toml_config::TomlFormatConfig;

/// Simple attempt to structure formatting rules.
/// Some formatting rules have to happen after other rules.
#[derive(PartialEq, Clone, Copy)]
pub enum FormattingStage {
    /// Before the document is formatted.
    /// Use this if the formatting logic doesn't depend on formatting.
    BeforeFormatting,
    /// At the same time, before or after document formatting.
    WhileFormatting,
    /// After the formatting is performed.
    /// Use this if the formatting logic is dependent upon length of lines for example.
    AfterFormatting,
}

/// The in memory representation of a Cargo.toml file.
/// This is the main entry point for formatting a Cargo.toml file.
pub struct CargoToml {
    pub toml_document: Document,
    rules: Vec<(bool, FormattingStage, Box<dyn TomlFormatter>)>,
    config: TomlFormatConfig,
}

impl CargoToml {
    /// Loads the given toml contents and doesn't initialize default formatting rules.
    fn new(toml_contents: String, config: TomlFormatConfig) -> anyhow::Result<Self> {
        let toml_document = toml_contents
            .parse::<Document>()
            .map_err(|e| anyhow::anyhow!("Failed to parse toml. {e}"))?;

        Ok(Self {
            toml_document,
            rules: vec![],
            config,
        })
    }

    /// Loads the given toml contents in memory and initializes the default formatting rules.
    pub fn default(toml_contents: String) -> anyhow::Result<Self> {
        let config = TomlFormatConfig::default();
        let cargo_toml = CargoToml::from_config(toml_contents, config)?;

        Ok(cargo_toml)
    }

    /// Loads the given toml contents in memory and initializes the formatting rules as configured by the configuration.
    pub fn from_config(toml_contents: String, config: TomlFormatConfig) -> anyhow::Result<Self> {
        let mut toml = Self::new(toml_contents, config.clone())?;

        if config.order_sections {
            toml.add_format_rule(FormattingStage::BeforeFormatting, OrderSections);
        }

        if config.order_package_section {
            toml.add_format_rule(FormattingStage::BeforeFormatting, OrderPackageSection);
        }

        if config.order_table_keys {
            toml.add_format_rule(
                FormattingStage::BeforeFormatting,
                OrderTableKeysAlphabetically,
            );
        }

        if config.order_dependencies.is_some() {
            toml.add_format_rule(FormattingStage::BeforeFormatting, OrderDependencies);
        }

        if config.trim_key_quotes {
            toml.add_format_rule(FormattingStage::BeforeFormatting, KeyQuoteTrimmer);
        }

        if config.trim_all_keys {
            toml.add_format_rule(FormattingStage::BeforeFormatting, KeyTrimmer);
        }

        if config.trim_section_key_names {
            toml.add_format_rule(FormattingStage::BeforeFormatting, SectionKeyNameTrimmer);
        }

        if config.table_formatting {
            toml.add_format_rule(FormattingStage::WhileFormatting, TableFormatting);
        }

        if config.wrap_array.is_some() {
            toml.add_format_rule(FormattingStage::AfterFormatting, WrapArray);
        }

        if config.wrap_table.is_some() {
            toml.add_format_rule(FormattingStage::AfterFormatting, InlineTableWrap::new());
        }

        if config.add_newline_after_section {
            toml.add_format_rule(FormattingStage::AfterFormatting, AppendLineAfterSection);
        }

        Ok(toml)
    }

    /// Adds a custom formatting rule that will be executed in the provided formatting stage.
    pub fn add_format_rule<T: TomlFormatter + 'static>(&mut self, stage: FormattingStage, rule: T) {
        self.rules.push((true, stage, Box::from(rule)));
    }

    /// Formats the toml document in memory.
    /// This iterates all rules and applies rules in order of their stage.
    pub fn format(&mut self) -> anyhow::Result<()> {
        let mut toml_document = self.toml_document.clone();

        let mut iter_stage = |filter_stage: FormattingStage| -> anyhow::Result<()> {
            for (enabled, _, rule) in self
                .rules
                .iter_mut()
                .filter(|(_, stage, _)| *stage == filter_stage)
            {
                if *enabled {
                    match rule.visit_document(&mut toml_document, &self.config) {
                        Ok(_) => {}
                        Err(e) => anyhow::bail!("Error: {:?}", e),
                    }
                }
            }
            Ok(())
        };

        iter_stage(FormattingStage::BeforeFormatting)?;
        iter_stage(FormattingStage::WhileFormatting)?;
        iter_stage(FormattingStage::AfterFormatting)?;

        self.toml_document = toml_document;

        Ok(())
    }

    /// Returns the dependencies section of this document.
    pub fn dependencies(&mut self) -> anyhow::Result<&mut Item> {
        self.toml_document
            .get_mut("dependencies")
            .ok_or_else(|| anyhow::anyhow!("Dependencies tag not found in toml document"))
    }
}
