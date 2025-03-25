# Zed Mojo

A Zed extension to provide Mojo language support with syntax highlighting and LSP integration.

## Features

- Syntax highlighting
- Outlines
- Language Server Protocol (LSP) support

## Installation

1. Install the [Magic Package Manager](https://developer.modular.com/download)
2. Clone this repository
3. Open Zed and click "Install Dev Extension" from the extensions page
4. Select the cloned repository

## Configuration

### LSP Settings

Configure the Language Server Protocol in your Zed settings:

```json
{
  "lsp": {
    "mojo": {
      "settings": {
        "lsp_path": "/name/my-proj/.magic/envs/default/bin/mojo-lsp-server", // path to mojo-lsp-server
        "args": ["--log=verbose"] // additional args
      }
    }
  },
}
```

### Formatter Settings

Enable formatting by adding following in zed setting :

```json
{
  "languages": {
    "mojo": {
      "formatter": {
        "external": {
          "command": "magic",
          "arguments": ["run", "mojo", "format", "-q", "-"]
        }
      }
    }
  }
}
```

## Grammar

The extension uses [tree-sitter-mojo](https://github.com/lsh/tree-sitter-mojo/) for syntax highlighting.

## Contributing

Feedback and contributions are welcome! Please share your suggestions to help improve this extension.

> [!NOTE]
> The [Magic](https://docs.modular.com/magic/) Package Manager is required for the LSP functionality to work properly.
