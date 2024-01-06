use anyhow::Result;
use lapce_plugin::psp_types::lsp_types::InitializeParams;

pub(crate) fn initialize_plugin(_: InitializeParams) -> Result<()> {
    Ok(())
}
