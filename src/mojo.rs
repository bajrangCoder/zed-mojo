use zed_extension_api::{self as zed, settings::LspSettings, Result};

struct MojoExtension {}

impl zed::Extension for MojoExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

        let error_message = "Must install `magic (mojo-lsp-server)` package manager of mojo language or provide the path to the mojo-lsp-server binary in the settings";

        let settings_clone = settings.settings.clone();
        let mojo_binary_path = settings_clone
            .and_then(|s| {
                s.get("lsp_path")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
            })
            .unwrap_or_else(|| {
                worktree
                    .which("magic")
                    .ok_or_else(|| error_message.to_string())
                    .unwrap()
            });

        let args = settings
            .settings
            .and_then(|s| s.get("args").cloned())
            .and_then(|v| {
                v.as_array().map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
            })
            .unwrap_or_else(|| vec![String::from("run"), String::from("mojo-lsp-server")]);

        Ok(zed::Command {
            command: mojo_binary_path.to_string(),
            args,
            env: Default::default(),
        })
    }
}

zed::register_extension!(MojoExtension);
