use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct PluginConfiguration {
    #[serde(rename = "rustAnalyzer")]
    pub rust_analyzer: Option<RustAnalyzerConfiguration>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RustAnalyzerConfiguration {
    #[serde(default)]
    #[serde(rename = "nightly")]
    pub nightly: bool,
}
