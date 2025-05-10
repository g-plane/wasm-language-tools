# Editors

## Visual Studio Code

Visual Studio Code is the recommended editor, since it's the best supported editor.

To use WebAssembly Language Tools in Visual Studio Code, you just need to install
[the extension](https://marketplace.visualstudio.com/items/?itemName=gplane.wasm-language-tools) from the marketplace.
You don't need to install the server binary manually, since the extension bundles it for you.

## Zed

Install the [WebAssembly Text Format](https://zed.dev/extensions?query=WebAssembly+Text+Format) extension.
Once opened a `.wat` file, server binary will be automatically downloaded, so you don't need to install it manually.

## Neovim

Neovim has built-in support for WebAssembly Language Tools via [nvim-lspconfig](https://github.com/neovim/nvim-lspconfig/blob/master/doc/configs.md#wasm_language_tools).

> [!IMPORTANT]
> You need to [install the server binary](./binary.md) manually and make sure it's in your `$PATH` (or specify the binary path manually).

For the minimal setup, add the following lines to your `init.lua`:

```lua
require("lspconfig").wasm_language_tools.setup({})
```

Additionally, you configure the language server like this:

```lua
require("lspconfig").wasm_language_tools.setup({
  settings = {
    format = {},
    lint = {
      unused = "warn",
    },
  },
})
```

## Helix

> [!IMPORTANT]
> You need to [install the server binary](./binary.md) manually and make sure it's in your `$PATH` (or specify the binary path manually).

For the minimal setup, add the following lines to `<config_dir>/helix/languages.toml`:

```toml
[language-server.wasm-language-tools]
command = "wat_server" # or the absolute path to the binary

[[language]]
name = "wat"
language-servers = ["wasm-language-tools"]
```

Additionally, you configure the language server like this:

```toml
[language-server.wasm-language-tools]
command = "wat_server"
config = { format = {}, lint = { unused = "warn" } } # [!code ++]

[[language]]
name = "wat"
language-servers = ["wasm-language-tools"]
```
