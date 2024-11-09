# Zed Mojo

Mojo language support for Zed. It provides syntax highlighting and lsp.

## Grammar

- [tree-sitter-mojo](https://github.com/lsh/tree-sitter-mojo/)

## Feedback

Provide your feedback to improve this extension as I don't use mojo language.

## Settings

You can configure the lsp by changing the zed settings

```json
{
  "lsp": {
    "mojo": {
      "settings": {
        "lsp_path": "/name/my-proj/.magic/envs/default/bin/mojo-lsp-server", // path to mojo-lsp-server
        "args": ["--log=verbose"] // additional args for mojo-lsp-server
      }
    }
  },
}
```

## Installation

- Clone this repository.
- From zed extension page click "Install Dev Extension" and select the cloned repository.

> Note: This extension is not published to zed extension store. As this isn't stable and it isn't complete.
