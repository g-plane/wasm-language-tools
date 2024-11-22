The formatter (pretty printer) for the WebAssembly Text Format.

This formatter can format a tree that contains syntax errors.

## Usage

The main [`format()`] function only accept parsed syntax tree,
so you should use `wat_parser::Parser` to parse source code first.

```rust
use rowan::ast::AstNode;
use wat_formatter::format;
use wat_parser::Parser;
use wat_syntax::ast::Root;

let input = "( module )";
let mut parser = Parser::new(input);
let root = Root::cast(parser.parse()).unwrap();
assert_eq!("(module)\n", format(&root, &Default::default()));
```

For customizing the formatting behavior, please refer to [`config`].
