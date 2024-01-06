use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct PluginConfiguration {
    #[serde(rename = "rustAnalyzer.nightly")]
    pub nightly: bool,
}
