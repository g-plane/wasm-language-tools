# Introduction

WebAssembly Language Tools aims to provide and improve the editing experience of WebAssembly Text Format.
It brings you features like code completion, go to definition, find references, hover information, rename symbol, and so on.

It also provides an out-of-the-box formatter (a.k.a. pretty printer) for WebAssembly Text Format.

While it's mainly used in editors, you can use the [`wat_service`](https://crates.io/crates/wat_service) crate to do semantic analysis with programmatic API.

WebAssembly Language Tools supports [Wasm 3.0](https://webassembly.org/news/2025-09-17-wasm-3.0/).

Except `wat_server`, other crates provide APIs as libraries. You can use them in your own projects.

|Crate|Version|
|---|---|
|wat_formatter|![Crates.io Version](https://img.shields.io/crates/v/wat_formatter?link=https%3A%2F%2Fdocs.rs%2Fwat_formatter)|
|wat_parser|![Crates.io Version](https://img.shields.io/crates/v/wat_parser?link=https%3A%2F%2Fdocs.rs%2Fwat_parser)|
|wat_service|![Crates.io Version](https://img.shields.io/crates/v/wat_service?link=https%3A%2F%2Fdocs.rs%2Fwat_service)|
|wat_syntax|![Crates.io Version](https://img.shields.io/crates/v/wat_syntax?link=https%3A%2F%2Fdocs.rs%2Fwat_syntax)|

## Supported WebAssembly Features

- Bulk Memory Operations
- Compact Import Section
- Compilation Hints
- Custom Page Sizes
- Custom Text Format Annotations
- Exception
- Extended Constant Expressions
- Garbage Collection
- Memory64
- Multiple Memories
- Multi-value
- Import/Export of Mutable Globals
- Reference Types
- Relaxed SIMD
- Non-trapping float-to-int Conversions
- Sign-extension Operators
- Stack Switching
- Fixed-width SIMD
- Tail Call
- Threads
- Typed Function References
- Wide Arithmetic
