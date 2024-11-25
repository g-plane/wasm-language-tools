The formatter (pretty printer) for the WebAssembly Text Format.

This formatter can format a tree that contains syntax errors.

## Usage

### Full

The [`format()`] function only accepts parsed syntax tree,
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

### Range

You can format only a specific range of code by calling [`format_range`] function.

Beside the root syntax tree and format options,
this function also accepts requested range and `LineIndex`.

Notes:

- Returned formatted string is corresponding to specific syntax node.
  It isn't full text, so you may replace by yourself.
- Affected range will equal or be wider than the range you give,
  so you should use returned range when replacing, not original range.

```rust
use line_index::LineIndex;
use rowan::{ast::AstNode, TextRange, TextSize};
use wat_formatter::format_range;
use wat_parser::Parser;
use wat_syntax::ast::Root;

let input = "( module ( func ) )";
let line_index = LineIndex::new(input);
let mut parser = Parser::new(input);
let root = Root::cast(parser.parse()).unwrap();
let (formatted, range) = format_range(
    &root,
    &Default::default(),
    TextRange::new(TextSize::new(13), TextSize::new(17)),
    &line_index,
).unwrap();
assert_eq!("(func)", &formatted);
assert_eq!(TextRange::new(TextSize::new(9), TextSize::new(17)), range);
```
