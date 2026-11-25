use std::{env, fs, path::Path};
use zed_extension_api::{self as zed, Result};

static GITHUB_REPO: &'static str = "navi-language/navi";
static NAVI_SERVER_BIN_NAME: &'static str = "navi-lsp-server";

const NAVI_LSP_VSIX: &str = "https://raw.githubusercontent.com/longbridge/navi/9fa1c19fc444247f3e893c0d515c653b952fcea6/public/navi-vscode.vsix";
const NAVI_LSP_DIR: &str = "navi-lsp-0.5.0-9fa1c19";
const NAVI_LSP_ENTRYPOINT: &str = "extension/dist/server.node.js";
const NAVI_LSP_MODULES: [&str; 2] = [
    "extension/wasm-lsp/navi_lsp_wasm_bg.wasm",
    "extension/wasm-formatter/navi_formatter_wasm_bg.wasm",
];

struct NaviExtension {
    cached_binary_path: Option<String>,
}

enum Status {
    None,
    CheckingForUpdate,
    Downloading,
    Failed(String),
}

fn update_status(id: &zed::LanguageServerId, status: Status) {
    match status {
        Status::None => zed::set_language_server_installation_status(
            id,
            &zed::LanguageServerInstallationStatus::None,
        ),
        Status::CheckingForUpdate => zed::set_language_server_installation_status(
            id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        ),
        Status::Downloading => zed::set_language_server_installation_status(
            id,
            &zed::LanguageServerInstallationStatus::Downloading,
        ),
        Status::Failed(msg) => zed::set_language_server_installation_status(
            id,
            &zed::LanguageServerInstallationStatus::Failed(msg),
        ),
    }
}

fn navi_05_server_command(
    id: &zed::LanguageServerId,
    worktree: &zed::Worktree,
) -> Result<zed::Command> {
    let native_binary = match zed::current_platform().0 {
        zed::Os::Windows => "navi-lsp.exe",
        _ => "navi-lsp",
    };
    if let Some(command) = worktree.which(native_binary) {
        return Ok(zed::Command {
            command,
            args: vec![],
            env: Default::default(),
        });
    }

    let node = zed::node_binary_path()?;
    let directory = Path::new(NAVI_LSP_DIR);
    let server = directory.join(NAVI_LSP_ENTRYPOINT);
    if !navi_05_installation_is_complete(directory) {
        update_status(id, Status::Downloading);
        let staging = format!("{NAVI_LSP_DIR}-staging");
        let staging_path = Path::new(&staging);
        for path in [staging_path, directory] {
            if path.is_dir() {
                fs::remove_dir_all(path)
                    .map_err(|err| format!("failed to clear Navi LSP download: {err}"))?;
            }
        }
        zed::download_file(NAVI_LSP_VSIX, &staging, zed::DownloadedFileType::Zip).map_err(
            |err| format!("failed to download the official Navi language server: {err}"),
        )?;
        if !navi_05_installation_is_complete(staging_path) {
            return Err(
                "the Navi archive is missing the language server or its WebAssembly modules".into(),
            );
        }
        fs::rename(staging_path, directory)
            .map_err(|err| format!("failed to install the Navi language server: {err}"))?;
    }
    let server = env::current_dir()
        .map_err(|err| format!("failed to locate the Navi extension work directory: {err}"))?
        .join(server);

    Ok(zed::Command {
        command: node,
        args: vec![server.to_string_lossy().into_owned(), "--stdio".into()],
        env: Default::default(),
    })
}

fn navi_05_installation_is_complete(directory: &Path) -> bool {
    directory.join(NAVI_LSP_ENTRYPOINT).is_file()
        && NAVI_LSP_MODULES
            .iter()
            .all(|path| directory.join(path).is_file())
}

impl NaviExtension {
    fn language_server_binary_path(
        &mut self,
        id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        // Check if the binary is already installed by manually checking the path
        if let Some(path) = worktree.which(NAVI_SERVER_BIN_NAME) {
            return Ok(path);
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                update_status(id, Status::None);
                return Ok(path.clone());
            }
        }
        let (platform, arch) = zed::current_platform();

        if fs::metadata(NAVI_SERVER_BIN_NAME).map_or(false, |stat| stat.is_file()) {
            update_status(id, Status::None);
            return Ok(NAVI_SERVER_BIN_NAME.to_string());
        }

        update_status(id, Status::CheckingForUpdate);

        let release = zed::latest_github_release(
            GITHUB_REPO,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: true,
            },
        )?;

        let asset_name = format!(
            "navi-{os}-{arch}.{ext}",
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
            ext = match platform {
                zed::Os::Windows => "zip",
                _ => "tar.gz",
            }
        );

        let file_type = match platform {
            zed::Os::Windows => zed::DownloadedFileType::Zip,
            _ => zed::DownloadedFileType::GzipTar,
        };

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("navi-{}", release.version);
        let binary_path = format!("{version_dir}/{NAVI_SERVER_BIN_NAME}");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            update_status(id, Status::Downloading);
            zed::download_file(&asset.download_url, &version_dir, file_type)
                .map_err(|e| format!("failed to download file: {e}"))?;

            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir)
                    && !entry.file_name().to_string_lossy().starts_with("navi-lsp-")
                {
                    fs::remove_dir_all(&entry.path()).ok();
                }
            }

            update_status(id, Status::None);
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
        id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        if id.as_ref() == "navi_05" {
            let result = navi_05_server_command(id, worktree);
            match &result {
                Ok(_) => update_status(id, Status::None),
                Err(err) => update_status(id, Status::Failed(err.to_string())),
            }
            return result;
        }

        let command = self
            .language_server_binary_path(id, worktree)
            .map_err(|err| {
                update_status(id, Status::Failed(err.to_string()));
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
