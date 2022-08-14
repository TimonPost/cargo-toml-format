use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumProperty, EnumString, EnumVariantNames, VariantNames};

#[derive(
    Clone,
    Debug,
    Display,
    AsRefStr,
    Eq,
    PartialEq,
    PartialOrd,
    Hash,
    EnumString,
    Serialize,
    Deserialize,
)]
pub enum TomlSort {
    Alphabetical,
    Length,
}

#[derive(
    Clone,
    Debug,
    Display,
    AsRefStr,
    Eq,
    PartialEq,
    PartialOrd,
    Hash,
    EnumString,
    Serialize,
    Deserialize,
    EnumProperty,
    EnumVariantNames,
)]
pub enum TomlSection {
    #[strum(serialize = "package", props(order = "a"))]
    Package,
    #[strum(serialize = "lib", props(order = "b"))]
    Lib,
    #[strum(serialize = "bins", props(order = "c"))]
    Bins,
    #[strum(serialize = "example", props(order = "d"))]
    Example,
    #[strum(serialize = "test", props(order = "e"))]
    Test,
    #[strum(serialize = "bench", props(order = "f"))]
    Bench,
    #[strum(serialize = "dependencies", props(order = "g"))]
    Dependencies,
    #[strum(serialize = "dev-dependencies", props(order = "h"))]
    DevDependencies,
    #[strum(serialize = "build-dependencies", props(order = "i"))]
    BuildDependencies,
    #[strum(serialize = "target", props(order = "j"))]
    Target,
    #[strum(serialize = "badges", props(order = "k"))]
    Badges,
    #[strum(serialize = "features", props(order = "l"))]
    Features,
    #[strum(serialize = "patch", props(order = "m"))]
    Patch,
    #[strum(serialize = "replace", props(order = "n"))]
    Replace,
    #[strum(serialize = "profile", props(order = "o"))]
    Profile,
    #[strum(serialize = "workspace", props(order = "p"))]
    Workspace,
    #[strum(serialize = "cargo-features", props(order = "q"))]
    CargoFeatures,
}

impl TomlSection {
    pub fn manifest_spec() -> Vec<String> {
        TomlSection::VARIANTS
            .iter()
            .map(|f| f.to_string())
            .collect()
    }
}

#[derive(
    Clone,
    Debug,
    Display,
    EnumVariantNames,
    AsRefStr,
    Eq,
    PartialEq,
    PartialOrd,
    Hash,
    EnumString,
    Serialize,
    Deserialize,
    EnumProperty,
)]
pub enum PackageOrder {
    #[strum(serialize = "name", props(order = "0"))]
    Name,
    #[strum(serialize = "version", props(order = "1"))]
    Version,
    #[strum(serialize = "authors", props(order = "2"))]
    Authors,
    #[strum(serialize = "edition", props(order = "3"))]
    Edition,
    #[strum(serialize = "rust-version", props(order = "4"))]
    RustVersion,
    #[strum(serialize = "description", props(order = "5"))]
    Description,
    #[strum(serialize = "documentation", props(order = "6"))]
    Documentation,
    #[strum(serialize = "readme", props(order = "7"))]
    Readme,
    #[strum(serialize = "homepage", props(order = "8"))]
    Homepage,
    #[strum(serialize = "repository", props(order = "9"))]
    Repository,
    #[strum(serialize = "license", props(order = "10"))]
    License,
    #[strum(serialize = "license-file", props(order = "11"))]
    LicenseFile,
    #[strum(serialize = "keywords", props(order = "12"))]
    Keywords,
    #[strum(serialize = "categories", props(order = "13"))]
    Categories,
    #[strum(serialize = "workspace", props(order = "14"))]
    Workspace,
    #[strum(serialize = "build", props(order = "15"))]
    Build,
    #[strum(serialize = "links", props(order = "16"))]
    Links,
    #[strum(serialize = "exclude", props(order = "17"))]
    Exclude,
    #[strum(serialize = "include", props(order = "18"))]
    Include,
    #[strum(serialize = "publish", props(order = "19"))]
    Publish,
    #[strum(serialize = "metadata", props(order = "20"))]
    Metadata,
    #[strum(serialize = "default-run", props(order = "21"))]
    DefaultRun,
    #[strum(serialize = "autobins", props(order = "22"))]
    AutoBins,
    #[strum(serialize = "autoexamples", props(order = "23"))]
    AutoExamples,
    #[strum(serialize = "autotests", props(order = "24"))]
    AutoTests,
    #[strum(serialize = "autobenches", props(order = "25"))]
    AutoBenchmarks,
    #[strum(serialize = "resolver", props(order = "26"))]
    Resolver,
}

impl PackageOrder {
    pub fn manifest_spec() -> Vec<String> {
        PackageOrder::VARIANTS
            .iter()
            .map(|f| f.to_string())
            .collect()
    }
}
