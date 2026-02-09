<div align="center"><img src="./media/logo.svg" width="160"></div>
<h1 align="center">WebAssembly Language Tools</h1>

WebAssembly Language Tools aims to provide and improve the editing experience of WebAssembly Text Format.
It delivers deep and smart static analysis, precise type checking, and full-featured editor integration — plus a configurable formatter — making WebAssembly development fast, safe, and joyful.

<picture>
  <source media="(prefers-color-scheme: light)" srcset="https://github.com/user-attachments/assets/475a0cbb-adaf-47f1-9277-1080c1e9a92e">
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/user-attachments/assets/be706755-9926-41b9-8270-7bc2eff440c0">
  <img src="https://github.com/user-attachments/assets/475a0cbb-adaf-47f1-9277-1080c1e9a92e" />
</picture>

## Why WebAssembly Language Tools?

Smart, zero-config tooling for confident WebAssembly development:

- 🌐 **WebAssembly 3.0 Ready**  \
  Full core spec support (GC, exceptions, etc.) and ongoing Phase 4 & 3 proposals — code with tomorrow's standards today.
- 🔍 **Deep Semantic Checks**  \
  Powered by Control Flow Analysis (CFA) for precise unreachable code detection, along with checks for uninitialized locals, written-but-never-read locals, and undefined and unused items.
- ⚖️ **Precise Type Safety**  \
  Comprehensive type checking with friendly error messages and subtyping relationship validation.
- 🔒 **Mutability Guard**  \
  Catches accidental mutations of immutables and redundant mutable declarations.
- 🎨 **Highly Configurable Formatter**  \
  Works out of the box while offering more than 10 formatting options for flexible and personalized code style.
- 🔧 **Smart Code Actions**  \
  Provides around 20 practical code actions to assist with quick fixes, refactoring, and code improvements.
- ⚡ **Near-instant Feedback**  \
  Deeply tuned and heavily optimized — silky smooth even on huge modules.
- 💡 **Full Editor Experience**  \
  Code completion, hover, go-to-definition, find references, rename and more work out of the box in VS Code, Zed, Neovim, Emacs, and Helix.

*New to WebAssembly tooling? Start coding with confidence — errors become guidance, not frustration.*

## Editor Features

<details>
  <summary>Code Completion</summary>
  <picture>
    <source media="(prefers-color-scheme: light)" srcset="https://github.com/user-attachments/assets/0185b411-a6cf-4372-9232-39e1c211a414">
    <source media="(prefers-color-scheme: dark)" srcset="https://github.com/user-attachments/assets/c91ced71-1f49-41bc-b153-56230236d5bb">
    <img src="https://github.com/user-attachments/assets/0185b411-a6cf-4372-9232-39e1c211a414">
  </picture>
</details>

<details>
  <summary>Go to Definition</summary>
  <picture>
    <source media="(prefers-color-scheme: light)" srcset="https://github.com/user-attachments/assets/cf803292-f17f-46f2-b091-79468e3ed73f">
    <source media="(prefers-color-scheme: dark)" srcset="https://github.com/user-attachments/assets/7006d7cd-6ebc-46dc-804c-0a7c06defbdc">
    <img src="https://github.com/user-attachments/assets/cf803292-f17f-46f2-b091-79468e3ed73f">
  </picture>
</details>

<details>
  <summary>Find References</summary>
  <picture>
    <source media="(prefers-color-scheme: light)" srcset="https://github.com/user-attachments/assets/f680a504-132d-469a-bd02-17f73ecb83f1">
    <source media="(prefers-color-scheme: dark)" srcset="https://github.com/user-attachments/assets/55b1b3be-4f9c-4870-a02d-826f4f18c2ba">
    <img src="https://github.com/user-attachments/assets/f680a504-132d-469a-bd02-17f73ecb83f1">
  </picture>
</details>

<details>
  <summary>Hover</summary>
  <picture>
    <source media="(prefers-color-scheme: light)" srcset="https://github.com/user-attachments/assets/30e9e84b-58c0-44ab-a249-da10f234d705">
    <source media="(prefers-color-scheme: dark)" srcset="https://github.com/user-attachments/assets/59520e97-8b34-4440-ada8-7f16cf2e1c2d">
    <img src="https://github.com/user-attachments/assets/30e9e84b-58c0-44ab-a249-da10f234d705">
  </picture>
</details>

<details>
  <summary>Rename</summary>
  <picture>
    <source media="(prefers-color-scheme: light)" srcset="https://github.com/user-attachments/assets/fc566c85-e99d-4cc6-93dc-a3e49248e745">
    <source media="(prefers-color-scheme: dark)" srcset="https://github.com/user-attachments/assets/60b65b98-55e4-4cee-9ffd-3531466de2fb">
    <img src="https://github.com/user-attachments/assets/fc566c85-e99d-4cc6-93dc-a3e49248e745">
  </picture>
</details>

<details>
  <summary>Document Symbols</summary>
  <picture>
    <source media="(prefers-color-scheme: light)" srcset="https://github.com/user-attachments/assets/91ab73f6-577e-445e-913f-f16c754b9701">
    <source media="(prefers-color-scheme: dark)" srcset="https://github.com/user-attachments/assets/5447a407-fba2-4e65-ab45-9ef423009999">
    <img src="https://github.com/user-attachments/assets/91ab73f6-577e-445e-913f-f16c754b9701">
  </picture>
</details>

<details>
  <summary>Diagnostics</summary>
  <picture>
    <source media="(prefers-color-scheme: light)" srcset="https://github.com/user-attachments/assets/7f90d054-7a0b-4a59-8239-9927a2cec14f">
    <source media="(prefers-color-scheme: dark)" srcset="https://github.com/user-attachments/assets/c2db6cc7-3c4c-428c-a1ef-4e5464095da8">
    <img src="https://github.com/user-attachments/assets/7f90d054-7a0b-4a59-8239-9927a2cec14f">
  </picture>
</details>

<details>
  <summary>Inlay Hint</summary>
  <picture>
    <source media="(prefers-color-scheme: light)" srcset="https://github.com/user-attachments/assets/e63e0ed1-05ab-42bb-9180-4611f008a198">
    <source media="(prefers-color-scheme: dark)" srcset="https://github.com/user-attachments/assets/234ff428-f0e6-49f9-80e9-97417ea99e24">
    <img src="https://github.com/user-attachments/assets/e63e0ed1-05ab-42bb-9180-4611f008a198">
  </picture>
</details>

<details>
  <summary>Code Action</summary>
  <picture>
    <source media="(prefers-color-scheme: light)" srcset="https://github.com/user-attachments/assets/2048edca-f542-4bdf-b2e8-a57c49559ccc">
    <source media="(prefers-color-scheme: dark)" srcset="https://github.com/user-attachments/assets/73052830-26dd-4434-b6ed-84ec742434c9">
    <img src="https://github.com/user-attachments/assets/2048edca-f542-4bdf-b2e8-a57c49559ccc">
  </picture>
</details>

<details>
  <summary>Formatting</summary>
  <picture>
    <source media="(prefers-color-scheme: light)" srcset="https://github.com/user-attachments/assets/3f164aa7-12ec-4377-a510-cb325a8c0a98">
    <source media="(prefers-color-scheme: dark)" srcset="https://github.com/user-attachments/assets/9f56b529-a390-4db1-9468-73e1875146d2">
    <img src="https://github.com/user-attachments/assets/3f164aa7-12ec-4377-a510-cb325a8c0a98">
  </picture>
</details>

<details>
  <summary>Semantic Highlighting</summary>
  <picture>
    <source media="(prefers-color-scheme: light)" srcset="https://github.com/user-attachments/assets/1d4fa62b-48fb-4d12-a2f7-392c8805dc9f">
    <source media="(prefers-color-scheme: dark)" srcset="https://github.com/user-attachments/assets/06c94113-4698-4b4d-8ca2-e2edc890ff02">
    <img src="https://github.com/user-attachments/assets/1d4fa62b-48fb-4d12-a2f7-392c8805dc9f">
  </picture>
</details>

<details>
  <summary>Call Hierarchy</summary>
  <picture>
    <source media="(prefers-color-scheme: light)" srcset="https://github.com/user-attachments/assets/374db3a2-6b0c-4235-9a8c-c37e196ced53">
    <source media="(prefers-color-scheme: dark)" srcset="https://github.com/user-attachments/assets/8bbb9e85-4750-41a7-958f-f55a2ec4c6ca">
    <img src="https://github.com/user-attachments/assets/374db3a2-6b0c-4235-9a8c-c37e196ced53">
  </picture>
  <picture>
    <source media="(prefers-color-scheme: light)" srcset="https://github.com/user-attachments/assets/97797d14-77fb-4505-b97c-70c6d0c80f81">
    <source media="(prefers-color-scheme: dark)" srcset="https://github.com/user-attachments/assets/c993d747-3744-465f-a520-2ec6a09158c9">
    <img src="https://github.com/user-attachments/assets/97797d14-77fb-4505-b97c-70c6d0c80f81">
  </picture>
</details>

<details>
  <summary>Signature Help</summary>
  <picture>
    <source media="(prefers-color-scheme: light)" srcset="https://github.com/user-attachments/assets/3beaeb93-63ca-469a-bded-bbff53e9eca1">
    <source media="(prefers-color-scheme: dark)" srcset="https://github.com/user-attachments/assets/87314ee1-7ffe-4aa7-ac0a-9931108ed430">
    <img src="https://github.com/user-attachments/assets/3beaeb93-63ca-469a-bded-bbff53e9eca1">
  </picture>
</details>

<details>
  <summary>Type Hierarchy</summary>
  <picture>
    <source media="(prefers-color-scheme: light)" srcset="https://github.com/user-attachments/assets/76c734b9-e6c8-49ad-b105-2044d8f1ee09">
    <source media="(prefers-color-scheme: dark)" srcset="https://github.com/user-attachments/assets/ea4723b2-a1ce-4483-8c48-77e9762a9b3c">
    <img src="https://github.com/user-attachments/assets/76c734b9-e6c8-49ad-b105-2044d8f1ee09">
  </picture>
  <picture>
    <source media="(prefers-color-scheme: light)" srcset="https://github.com/user-attachments/assets/6409039c-7def-4082-8e90-0dc929b54188">
    <source media="(prefers-color-scheme: dark)" srcset="https://github.com/user-attachments/assets/a63f49d6-7099-415b-a9ec-1b74c122da9a">
    <img src="https://github.com/user-attachments/assets/6409039c-7def-4082-8e90-0dc929b54188">
  </picture>
</details>

## Usage

### Try it Online

Open [vscode.dev](https://vscode.dev/) or [github.dev](https://github.dev/), then search and install the `gplane.wasm-language-tools` extension.
After installed, open or create a ".wat" file to try it out.

### Editor Support

- Visual Studio Code: Install the [WebAssembly Language Tools](https://marketplace.visualstudio.com/items?itemName=gplane.wasm-language-tools) extension.
- Zed: Install the [WebAssembly Text Format](https://zed.dev/extensions?query=WebAssembly+Text+Format) extension.
- Neovim: Built-in support in [nvim-lspconfig](https://github.com/neovim/nvim-lspconfig/blob/master/doc/configs.md#wasm_language_tools) with name `wasm_language_tools`.
- Emacs: [lsp-mode](https://emacs-lsp.github.io/lsp-mode/page/lsp-wasm-language-tools/), [Eglot](https://elpa.gnu.org/packages/doc/eglot.html), and [lsp-bridge](https://github.com/manateelazycat/lsp-bridge) all have built-in support.
- Helix: Built-in support.

For other editors and advanced configurations, please refer to the [editor guide](https://wasm-language-tools.netlify.app/guide/getting-started/editors.html).

### Binaries

We've provided pre-built binaries on [GitHub Releases](https://github.com/g-plane/wasm-language-tools/releases).
You can download it according to your platform, then extract it from the compressed file.
Or, read the [documentation](https://wasm-language-tools.netlify.app/guide/getting-started/executable.html) for alternative installation methods.

## Documentation

Please visit the [documentation website](https://wasm-language-tools.netlify.app/) for configuration and diagnostics explanation.

## License

MIT License

Copyright (c) 2024-present Pig Fang
