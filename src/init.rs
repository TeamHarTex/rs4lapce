use crate::config::PluginConfiguration;
use anyhow::{anyhow, Error, Result};
use flate2::read::GzDecoder;
use lapce_plugin::psp_types::lsp_types::{DocumentFilter, InitializeParams, MessageType, Url};
use lapce_plugin::{Http, VoltEnvironment, PLUGIN_RPC};
use std::fs::File;
use std::path::PathBuf;
use std::{fs, io};

pub(crate) fn initialize_plugin(params: InitializeParams) -> Result<()> {
    let config = params
        .initialization_options
        .map(serde_json::from_value::<PluginConfiguration>)
        .transpose()?;

    let nightly = if let Some(config) = &config
        && let Some(rust_analyzer_builds) = &config.rust_analyzer_builds
    {
        rust_analyzer_builds.nightly
    } else {
        false
    };

    PLUGIN_RPC.window_show_message(
        MessageType::LOG,
        format!(
            "using {} rust-analyzer",
            if nightly { "nightly" } else { "stable" }
        ),
    );

    let architecture = match VoltEnvironment::architecture()?.as_str() {
        "aarch64" => "aarch64",
        "x86_64" => "x86_64",
        unsupported => {
            PLUGIN_RPC.window_show_message(
                MessageType::ERROR,
                format!("unsupported system architecture: {unsupported}"),
            );

            return Err(Error::msg(format!(
                "unsupported system architecture: {unsupported}"
            )));
        }
    };

    let (target_triple, executable_name) = match VoltEnvironment::operating_system()?.as_str() {
        "windows" => ("pc-windows-msvc", "rust-analyzer.exe"),
        "macos" => ("apple-darwin", "rust-analyzer"),
        // FIXME: unknown-linux-musl exists
        "linux" => ("unknown-linux-gnu", "rust-analyzer"),
        unsupported => {
            PLUGIN_RPC.window_show_message(
                MessageType::ERROR,
                format!("unsupported operating system: {unsupported}"),
            );

            return Err(Error::msg(format!(
                "unsupported operating system: {unsupported}"
            )));
        }
    };

    let file_path = PathBuf::from(executable_name);
    if !file_path.exists() {
        PLUGIN_RPC.window_show_message(MessageType::LOG, String::from("downloading rust-analyzer"));

        if let Err(error) = download_rust_analyzer(nightly, &file_path, architecture, target_triple)
        {
            PLUGIN_RPC.window_show_message(
                MessageType::ERROR,
                format!("failed to download rust-analyzer: {error}"),
            );

            return Ok(());
        }
    }

    let pwd = VoltEnvironment::uri()?;
    let server_path = Url::parse(&pwd)?.join(&file_path.as_os_str().to_string_lossy())?;

    PLUGIN_RPC.start_lsp(
        server_path,
        Vec::new(),
        vec![DocumentFilter {
            language: Some(String::from("rust")),
            scheme: None,
            pattern: None,
        }],
        config.map(|config| config.rust_analyzer).unwrap_or(None),
    );

    Ok(())
}

fn download_rust_analyzer(
    nightly: bool,
    target: &PathBuf,
    architecture: &str,
    target_triple: &str,
) -> Result<()> {
    let gz_path = PathBuf::from("rust-analyzer.gz");
    {
        let url = if nightly {
            format!("https://github.com/rust-lang/rust-analyzer/releases/download/nightly/rust-analyzer-{architecture}-{target_triple}.gz")
        } else {
            format!("https://github.com/rust-lang/rust-analyzer/releases/latest/download/rust-analyzer-{architecture}-{target_triple}.gz")
        };
        let mut response = Http::get(&url)?;
        let body = response.body_read_all()?;
        fs::write(&gz_path, body)?;

        let mut gz = GzDecoder::new(File::open(&gz_path)?);
        let mut file = File::create(target)?;
        io::copy(&mut gz, &mut file)?;
    }
    fs::remove_file(&gz_path).map_err(|x| anyhow!("remove_file {:?} fail: {:?}", gz_path, x))?;
    Ok(())
}
