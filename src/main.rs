use lapce_plugin::psp_types::lsp_types::request::Initialize;
use lapce_plugin::psp_types::lsp_types::{InitializeParams, MessageType};
use lapce_plugin::psp_types::Request;
use lapce_plugin::{register_plugin, LapcePlugin, PLUGIN_RPC};

mod config;
mod init;

#[derive(Default)]
struct Rs4Lapce;

register_plugin!(Rs4Lapce);

impl LapcePlugin for Rs4Lapce {
    fn handle_request(&mut self, _: u64, method: String, parameters: serde_json::value::Value) {
        if method.as_str() == Initialize::METHOD {
            let Ok(parameters) = serde_json::from_value::<InitializeParams>(parameters) else {
                PLUGIN_RPC.window_show_message(
                    MessageType::ERROR,
                    String::from("failed to obtain plugin initialization parameters"),
                );

                return;
            };

            if let Err(error) = init::initialize_plugin(parameters) {
                PLUGIN_RPC.window_show_message(
                    MessageType::ERROR,
                    format!("failed to initialize rs4lapce plugin: {error}"),
                );
            }
        }
    }
}
