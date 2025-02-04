The language service of WebAssembly Language Tools.
This is the core functionality behind the language server.

The language service does nothing about IO, which means it doesn't read source files.
Actually, it doesn't even require the source code to be a file on disk.

Besides, the language service doesn't communicate with language clients and
process Language Server Protocol messages (by JSON-RPC).
When using the language service,
you need to handle JSON-RPC communication and server loop by yourself,
and call corresponding methods of the language service when receiving requests
then send responses back to the client.
You're free to use [`lsp-server`](https://docs.rs/lsp-server) or [`tower-lsp`](https://docs.rs/tower-lsp)
with the language service, or do it completely by yourself as you like.
