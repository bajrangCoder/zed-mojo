use zed_extension_api::{self as zed, Result};

struct MojoExtension {}


impl zed::Extension for MojoExtension {
    fn new() -> Self {
         Self {}
     }

     fn language_server_command(
         &mut self,
         _: &zed::LanguageServerId,
         worktree: &zed::Worktree,
     ) -> Result<zed::Command> {
         let path = worktree
         .which("mojo-lsp-server")
         .ok_or_else(|| "Must install `mojo-lsp-server` [via Modular](https://developer.modular.com/download), its comes with mojo package".to_string())?;

         Ok(zed::Command {
             command: path,
             args: vec![],
             env: Default::default(),
         })
     }
}

zed::register_extension!(MojoExtension);
