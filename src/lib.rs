use std::fs;
use zed_extension_api::{self as zed, Result};

static GITHUB_REPO: &'static str = "navi-language/navi";
static NAVI_SERVER_BIN_NAME: &'static str = "navi-lsp-server";

struct NaviExtension {
    cached_binary_path: Option<String>,
}

enum Status {
    None,
    CheckingForUpdate,
    Downloading,
    Failed(String),
}

fn update_status(config: &zed::LanguageServerConfig, status: Status) {
    match status {
        Status::None => zed::set_language_server_installation_status(
            &config.name,
            &zed::LanguageServerInstallationStatus::None,
        ),
        Status::CheckingForUpdate => zed::set_language_server_installation_status(
            &config.name,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        ),
        Status::Downloading => zed::set_language_server_installation_status(
            &config.name,
            &zed::LanguageServerInstallationStatus::Downloading,
        ),
        Status::Failed(msg) => zed::set_language_server_installation_status(
            &config.name,
            &zed::LanguageServerInstallationStatus::Failed(msg),
        ),
    }
}

impl NaviExtension {
    fn language_server_binary_path(
        &mut self,
        config: &zed::LanguageServerConfig,
    ) -> Result<String> {
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                update_status(&config, Status::None);
                return Ok(path.clone());
            }
        }

        update_status(&config, Status::CheckingForUpdate);
        let release = zed::latest_github_release(
            GITHUB_REPO,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: true,
            },
        )?;

        let (platform, arch) = zed::current_platform();
        let asset_name = format!(
            "navi-{os}-{arch}.tar.gz",
            arch = match arch {
                zed::Architecture::Aarch64 => "arm64",
                zed::Architecture::X86 => "amd64",
                zed::Architecture::X8664 => "amd64",
            },
            os = match platform {
                zed::Os::Mac => "darwin",
                zed::Os::Linux => "linux",
                zed::Os::Windows => "windows",
            },
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("navi-{}", release.version);
        let binary_path = format!("{version_dir}/{NAVI_SERVER_BIN_NAME}");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            update_status(&config, Status::Downloading);
            zed::download_file(
                &asset.download_url,
                &version_dir,
                zed::DownloadedFileType::GzipTar,
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(&entry.path()).ok();
                }
            }

            update_status(&config, Status::None);
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for NaviExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        config: zed::LanguageServerConfig,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let command = self.language_server_binary_path(&config).map_err(|err| {
            update_status(&config, Status::Failed(err.to_string()));
            err
        })?;

        Ok(zed::Command {
            command,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(NaviExtension);
