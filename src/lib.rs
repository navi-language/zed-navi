use anyhow::{anyhow, Context, Result};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use futures::executor::block_on;
use serde_derive::Serialize;
use smol::fs;
use smol::io::BufReader;
use smol::stream::StreamExt;
use std::env::consts::ARCH;
use std::path::PathBuf;

use plugin::prelude::*;
use serde::Deserialize;
use util::github::latest_github_release;
use util::{async_maybe, http, ResultExt};

static NAVI_SERVER_BIN_NAME: &'static str = "navi-lsp-server";

#[derive(Debug, Deserialize, Serialize)]
struct VersionInfo {
    version: String,
    url: String,
}

#[import]
fn command(string: &str) -> Option<Vec<u8>>;

#[export]
pub fn name() -> &'static str {
    "navi"
}

#[export]
pub fn initialization_options() -> Option<String> {
    Some("{ \"nightly\": true }".to_string())
}

#[export]
pub fn language_ids() -> Vec<(String, String)> {
    vec![("Navi".into(), "navi".into())]
}

#[export]
pub fn server_args() -> Vec<String> {
    vec![]
}

#[export]
pub fn fetch_latest_server_version() -> Option<String> {
    block_on(async {
        let release = latest_github_release("navi-language/navi", true, true, http::client())
            .await
            .expect("error fetching latest release");

        let arch = match ARCH {
            "x86_64" => "amd64",
            "aarch64" => "arm64",
            _ => "amd64",
        };

        let asset_name = format!("navi-darwin-{}.tar.gz", arch);
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| anyhow!("no asset found matching {:?}", asset_name))
            .expect("error fetching asset");

        let version_info = VersionInfo {
            version: release.tag_name,
            url: asset.browser_download_url.clone(),
        };

        let version_info = serde_json::to_string(&version_info).log_err().unwrap();

        Some(version_info)
    })
}

#[export]
pub fn fetch_server_binary(
    container_dir: PathBuf,
    version_info: String,
) -> Result<PathBuf, String> {
    let version_info = serde_json::from_str::<VersionInfo>(&version_info).unwrap();
    let binary_path = container_dir.join(NAVI_SERVER_BIN_NAME);

    block_on(async {
        if fs::metadata(&binary_path).await.is_err() {
            let mut response = http::client()
                .get(&version_info.url, Default::default(), true)
                .await
                .context("error downloading release")
                .log_err()
                .unwrap();

            let decompressed_bytes = GzipDecoder::new(BufReader::new(response.body_mut()));
            let archive = Archive::new(decompressed_bytes);
            archive.unpack(container_dir).await.log_err();
        }

        fs::set_permissions(
            &binary_path,
            <fs::Permissions as fs::unix::PermissionsExt>::from_mode(0o755),
        )
        .await
        .log_err();
    });

    Ok(binary_path)
}

#[export]
pub fn cached_server_binary(container_dir: PathBuf) -> Option<PathBuf> {
    get_cached_server_binary(container_dir)
}

fn get_cached_server_binary(container_dir: PathBuf) -> Option<PathBuf> {
    block_on(async {
        async_maybe!({
            let mut last_binary_path = None;
            let mut entries = fs::read_dir(&container_dir).await?;
            while let Some(entry) = entries.next().await {
                let entry = entry?;
                if entry.file_type().await?.is_file()
                    && entry
                        .file_name()
                        .to_str()
                        .map_or(false, |name| name == NAVI_SERVER_BIN_NAME)
                {
                    last_binary_path = Some(entry.path());
                }
            }

            if let Some(path) = last_binary_path {
                Ok(path)
            } else {
                Err(anyhow!("no cached binary"))
            }
        })
        .await
        .log_err()
    })
}
