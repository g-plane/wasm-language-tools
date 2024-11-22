The parser for WebAssembly Text Format.

This parser is error-tolerant, which means it can parse even even if the input contains syntax errors.

This parser will produce concrete syntax tree (CST),
but you can build AST from it with a bunch of helpers from `wat_syntax::ast` module.

## Usage

You need to create a parser instance with source code.
Once created, the source code can't be changed.
So if you want to parse different source code, you need to create a new parser instance.

```rust
use wat_parser::Parser;
use wat_syntax::SyntaxKind;

let input = "(module)";
let mut parser = Parser::new(input);
let tree = parser.parse();
assert_eq!(tree.kind(), SyntaxKind::ROOT);
```

Any syntax errors won't prevent the parser from parsing the rest of the input,
however you can retrieve the errors like this:

```rust
use wat_parser::Parser;
use wat_syntax::SyntaxKind;

let input = "(module";
let mut parser = Parser::new(input);
let tree = parser.parse();
let errors = parser.errors();
assert_eq!(errors[0].start, 7);
assert!(errors[0].message.to_string().contains("expected `)`"));
```
