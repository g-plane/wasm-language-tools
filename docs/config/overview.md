# Configuration

Configuration in Visual Studio Code is a flatten set of key-value entries.
Each entry comes with a key which is a dot-separated string.
For other editors, configuration is a nested structure.

Supposed we want to set [`unused`](./lint.md#unused) as `"deny"` for linting and enable [`formatComments`](./format.md#formatcomments) for formatter,
configuration in Visual Studio Code will be:

```json
{
  "wasmLanguageTools.lint.unused": "deny",
  "wasmLanguageTools.format.formatComments": true
}
```

> [!TIP]
> VS Code allows us override user (global) settings in workspace settings.
> For example, we can set `wasmLanguageTools.lint.unused` to `"deny"` in workspace settings,
> while keeping it as `"warn"` in user (global) settings.

While for the same configuration above in Neovim, it will be:

```lua
vim.lsp.config("wasm_language_tools", {
  settings = {
    lint = {
      unused = "deny",
    },
    format = {
      formatComments = true,
    },
  },
})
```

For other editors, you can read [Editors Setup](../guide/getting-started/editors.md) for examples or refer to your editor's documentation.
