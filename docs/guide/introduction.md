# Introduction

WebAssembly Language Tools aims to provide and improve the editing experience of WebAssembly Text Format.
It brings you features like code completion, go to definition, find references, hover information, rename symbol, and so on.

It also provides an out-of-the-box formatter (a.k.a. pretty printer) for WebAssembly Text Format.

While it's mainly used in editors, you can use the [`wat_service`](https://crates.io/crates/wat_service) crate to do semantic analysis with programmatic API.

WebAssembly Language Tools supports some [WebAssembly proposals](https://webassembly.org/features/) like [Garbage Collection](https://github.com/WebAssembly/gc), [Multiple Memories](https://github.com/WebAssembly/multi-memory/blob/master/proposals/multi-memory/Overview.md) and so on.
