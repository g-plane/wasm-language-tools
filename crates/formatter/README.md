The formatter (pretty printer) for the WebAssembly Text Format.

This formatter can format a tree that contains syntax errors.

## Usage

### Format the whole syntax tree

The [`format()`] function only accepts parsed syntax tree,
so you should use the parser to parse source code first.

```rust
use wat_formatter::format;
use wat_syntax::{SyntaxNode, ast::{AstNode, Root}};

let input = "( module )";
let (root, _) = wat_parser::parse(input);
assert_eq!("(module)\n", format(&root, &Default::default()));
```

For customizing the formatting behavior, please refer to [`config`].

### Format a specific syntax node

You can format a specific syntax node by calling [`format_node`] function.

Notes:

- Returned formatted string is related to specific syntax node, not the root syntax tree, so you may replace by yourself.
- You may set `base_indent` to a different value other than `0` when that syntax node is indented in original source code. This can be useful in range formatting.

```rust
use wat_formatter::format_node;
use wat_syntax::AmberNode;

let input = "( module ( func ) )";
let (tree, _) = wat_parser::parse(input);
let root = AmberNode::new_root(&tree);
let func = root.children().next().unwrap().children().next().unwrap();
let formatted = format_node(func, &Default::default(), 0);
assert_eq!("(func)", &formatted);
```
