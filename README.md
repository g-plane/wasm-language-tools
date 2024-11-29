# WebAssembly Language Tools

WebAssembly Language Tools aims to provide and improve the editing experience of WebAssembly Text Format.
It also provides an out-of-the-box formatter (a.k.a. pretty printer) for WebAssembly Text Format.

## 📌 Features

### Code Completion

### Go to Definition

### Find References

### Hover

### Rename

### Document Symbols

### Inlay Hint

### Code Action

### Formatting

### Semantic Highlighting

### Call Hierarchy

## 🍵 Usage

We don't provide pre-built binaries at the moment.

If you have installed Rust, you can run Cargo to install:

```shell
cargo install --git https://github.com/g-plane/wasm-language-tools.git wat_server
```

### Editor Support

- Visual Studio Code: Install the [WebAssembly Language Tools](https://marketplace.visualstudio.com/items?itemName=gplane.wasm-language-tools) extension.
- Neovim: You need to configure with `nvim-lspconfig` manually at the moment.
- Zed: Coming soon.
- Helix: Add the following lines to `<config_dir>/helix/languages.toml`:
  ```toml
  [language-server.wasm-language-tools]
  command = "wat_server" # or the absolute path to the binary
  args = []

  [[language]]
  name = "wat"
  language-servers = ["wasm-language-tools"]
  ```

## 📜 License

MIT License

Copyright (c) 2024-present Pig Fang
