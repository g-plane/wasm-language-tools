The parser for WebAssembly Text Format.

This parser is error-tolerant, which means it can parse even even if the input contains syntax errors.

This parser will produce concrete syntax tree (CST),
but you can build AST from it with a bunch of helpers from `wat_syntax::ast` module.

## Usage

Use the main [`parse`] function:

```rust
use wat_syntax::SyntaxKind;

let input = "(module)";
let (tree, errors) = wat_parser::parse(input);
assert_eq!(tree.kind(), SyntaxKind::ROOT.into());
```

Any syntax errors won't prevent the parser from parsing the rest of the input,
so the [`parse`] function returns a tuple which contains the CST and syntax errors.
You can access syntax errors like this:

```rust
use rowan::TextSize;
use wat_syntax::SyntaxKind;

let input = "(module";
let (tree, errors) = wat_parser::parse(input);
assert_eq!(errors[0].range.start(), TextSize::from(7));
assert!(errors[0].message.to_string().contains("expected `)`"));
```
