# Zed Mojo

A Zed extension to provide Mojo language support with syntax highlighting and LSP integration.

## Features

- Syntax highlighting
- Outlines
- Language Server Protocol (LSP) support

## Installation

1. Install the [Pixi](https://pixi.sh)
2. Clone this repository
3. Open Zed and click "Install Dev Extension" from the extensions page
4. Select the cloned repository

### Formatter Settings

Enable formatting by adding following in zed setting :

```json
{
  "languages": {
    "mojo": {
      "formatter": {
        "external": {
          "command": "pixi",
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
