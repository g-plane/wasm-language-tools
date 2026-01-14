# Editors

## Visual Studio Code

Visual Studio Code is the recommended editor, since it's the best supported editor.

To use WebAssembly Language Tools in Visual Studio Code, you just need to install
[the extension](https://marketplace.visualstudio.com/items/?itemName=gplane.wasm-language-tools) from the marketplace.
You don't need to install the server executable manually, since the extension bundles it for you.

## Zed

Install the [WebAssembly Text Format](https://zed.dev/extensions?query=WebAssembly+Text+Format) extension.
Once opened a `.wat` file, server executable will be automatically downloaded, so you don't need to install it manually.

## Neovim

Neovim has built-in support for WebAssembly Language Tools via [nvim-lspconfig](https://github.com/neovim/nvim-lspconfig/blob/master/doc/configs.md#wasm_language_tools).

> [!IMPORTANT]
> You need to [install the server executable](./executable.md) manually and make sure it's in your `$PATH` (or specify the executable path manually).

For the minimal setup, add the following lines to your `init.lua`:

```lua
vim.lsp.enable("wasm_language_tools")
```

Additionally, you can configure the language server like this:

```lua
vim.lsp.config("wasm_language_tools", {
  settings = {
    format = {},
    lint = {
      unused = "warn",
    },
  },
})
vim.lsp.enable("wasm_language_tools")
```

If you're using Neovim 0.10 or older:

```lua
require("lspconfig").wasm_language_tools.setup({})
```

or with configuration:

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

## coc.nvim

> [!IMPORTANT]
> You need to [install the server executable](./executable.md) manually and make sure it's in your `$PATH` (or specify the executable path manually).

For the minimal setup, add the following lines to your `coc-settings.json`:

```json
{
  "languageserver": {
    "wasm-language-tools": {
      "command": "wat_server", // or the absolute path to the executable
      "filetypes": ["wat"]
    }
  }
}
```

## Emacs

> [!IMPORTANT]
> You need to [install the server executable](./executable.md) manually and make sure it's in your `$PATH` (or specify the executable path manually).

Emacs can't recognize `.wat` files by default. You need to create a major mode like this:

```emacs-lisp
(define-derived-mode wat-mode prog-mode "WAT")
(add-to-list 'auto-mode-alist '("\\.wat\\'" . wat-mode))
```

or use similar packages.

### lsp-mode

[lsp-mode](https://github.com/emacs-lsp/lsp-mode/) has built-in support for WebAssembly Language Tools.

For the minimal setup, you may need to add a hook:

```emacs-lisp
(use-package lsp-mode
  ; ... other configurations ...
  :hook
  (wat-mode . lsp-deferred))
```

Additionally, you can configure the language server like this:

```emacs-lisp
(use-package lsp-mode
  ; ... other configurations ...
  :hook
  (wat-mode . lsp-deferred)
  :config
  (setq lsp-wat-server-command '("/path/to/wat_server")) ; optional
  (setq lsp-wat-lint-unused "deny"))
```

To view all available configurations, please refer to [the documentation of lsp-mode](https://emacs-lsp.github.io/lsp-mode/page/lsp-wasm-language-tools/#available-configurations).

### lsp-bridge

[lsp-bridge](https://github.com/manateelazycat/lsp-bridge) has built-in support for WebAssembly Language Tools.

## Helix

Helix has built-in support for WebAssembly Language Tools.

> [!IMPORTANT]
> You need to [install the server executable](./executable.md) manually and make sure it's in your `$PATH` (or specify the executable path manually).

Additionally, you can configure the language server like this:

```toml
[language-server.wasm-language-tools]
command = "wat_server"
config = { format = {}, lint = { unused = "warn" } } # [!code ++]

[[language]]
name = "wat"
language-servers = ["wasm-language-tools"]
```

## Fresh

> [!IMPORTANT]
> You need to [install the server executable](./executable.md) manually and make sure it's in your `$PATH` (or specify the executable path manually).

For the minimal setup, add the following lines to your [Fresh Editor](https://github.com/sinelaw/fresh) config `~/.config/fresh/config.json`:

```json
{
  "languages": {
    "wat": {
      "extensions": ["wat"],
      "comment_prefix": ";;",
      "auto_indent": true
    }
  },
  "lsp": {
    "wat": {
      "command": "wat_server",
      "args": [],
      "enabled": true
    }
  }
}
```

or with configuration:

```json
{
  "languages": {
    "wat": {
      "extensions": ["wat"],
      "comment_prefix": ";;",
      "auto_indent": true
    }
  },
  "lsp": {
    "wat": {
      "command": "wat_server",
      "args": [],
      "enabled": true,
      "initialization_options": { // [!code ++]
        "lint": {}, // [!code ++]
        "format": {} // [!code ++]
      } // [!code ++]
    }
  }
}
```
