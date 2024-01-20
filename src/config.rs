use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct PluginConfiguration {
    #[serde(default)]
    #[serde(rename = "rustAnalyzer")]
    pub rust_analyzer: Option<Value>,
    #[serde(default)]
    #[serde(rename = "rustAnalyzerBuilds")]
    pub rust_analyzer_builds: Option<RustAnalyzerBuildsConfiguration>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RustAnalyzerBuildsConfiguration {
    #[serde(default)]
    pub nightly: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RustAnalyzerConfiguration {
    #[serde(default)]
    pub cargo: Option<RustAnalyzerCargoConfiguration>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RustAnalyzerCargoConfiguration {
    #[serde(default)]
    #[serde(rename = "buildScripts")]
    pub build_scripts: Option<RustAnalyzerCargoBuildScriptsConfiguration>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RustAnalyzerCargoBuildScriptsConfiguration {
    #[serde(default)]
    pub enable: bool,
}
