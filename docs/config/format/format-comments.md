# `formatComments`

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

If this option is set to `false`, comments contain leading or trailing whitespace will still be kept as-is.
