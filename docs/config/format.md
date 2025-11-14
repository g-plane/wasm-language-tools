# Format

In editor setup, all formatting options are flatten as shown below.
However, if you're using the formatter as a Rust crate, type structures are different.
For more details, please refer to the [API documentation](https://docs.rs/wat_formatter/latest/wat_formatter/config/index.html).

## `printWidth`

> default: `80`

The line width limitation that formatter should *(but not must)* avoid exceeding.
The formatter will try its best to keep line width less than this value,
but it may exceed for some cases, for example, a very very long single word.

## `indentWidth`

> default: `2`

Size of indentation. When enabled `useTabs`, this option may be disregarded,
since only one tab will be inserted when indented once.

## `lineBreak`

> default: `"lf"`

Specify using `\n` (`"lf"`) or `\r\n` (`"crlf"`) for line break.

## `useTabs`

> default: `false`

Specify using space or tab for indentation.

## `splitClosingParens`

> default: `false`

Control whether closing parentheses should be splitted into different lines.

When disabled, code will be formatted like this:

```wasm
(module
  (func
    (block)))
```

When enabled, code will be formatted like this:

```wasm
(module
  (func
    (block)
  )
)
```

## `formatComments`

> default: `false`

Control whether whitespace should be inserted at the beginning and end of comments.

For example, the following code:

```wasm
;;comment
(;comment;)
```

will be formatted to:

```wasm
;; comment
(; comment ;)
```

Though this option is set to `false`, comments contain leading or trailing whitespace will still be kept as-is.

## `ignoreCommentDirective`

> default: `"fmt-ignore"`

Text directive for ignoring formatting specific module or module field.
