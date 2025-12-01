# Server Executable

If you're using other editors, manually installing the server executable is required.

## GitHub Releases

You can download pre-built binaries from [GitHub Releases](https://github.com/g-plane/wasm-language-tools/releases).
Select the corresponding file according to your platform.

## Cargo

If you've installed Rust, you can run Cargo to install it globally:

```bash
cargo install wat_server
```

## cargo-binstall

If you have [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) installed,
you can install it without building from source:

```bash
cargo binstall wat_server
```

## AUR

Install with your favorite AUR helper, for example:

```bash
paru -S wasm-language-tools
```

or for pre-built executable:

```bash
paru -S wasm-language-tools-bin
```

## Nix

On NixOS:

```bash
nix-env -iA nixos.wasm-language-tools
```

On non-NixOS:

```bash
nix-env -iA nixpkgs.wasm-language-tools
```

Or modifying configuration:

```nix
environment.systemPackages = [
  pkgs.wasm-language-tools
];
```

View detail on [Nixpkgs](https://search.nixos.org/packages?show=wasm-language-tools).
