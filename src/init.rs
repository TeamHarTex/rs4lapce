use crate::config::PluginConfiguration;
use anyhow::Result;
use lapce_plugin::psp_types::lsp_types::{InitializeParams, MessageType};
use lapce_plugin::PLUGIN_RPC;

pub(crate) fn initialize_plugin(params: InitializeParams) -> Result<()> {
    let config = params
        .initialization_options
        .map(|value| serde_json::from_value::<PluginConfiguration>(value))
        .transpose()?;

    let nightly = if let Some(config) = config
        && let Some(rust_analyzer) = config.rust_analyzer
    {
        rust_analyzer.nightly
    } else {
        false
    };

    PLUGIN_RPC.window_show_message(
        MessageType::INFO,
        format!(
            "using {} rust-analyzer",
            if nightly { "nightly" } else { "stable" }
        ),
    );

    Ok(())
}
