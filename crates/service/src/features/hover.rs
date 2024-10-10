use super::find_meaningful_token;
use crate::{
    binder::{DefIdx, SymbolItem, SymbolItemKind, SymbolTable, SymbolTablesCtx},
    files::FilesCtx,
    helpers,
    types_analyzer::TypesAnalyzerCtx,
    LanguageService, LanguageServiceCtx,
};
use lsp_types::{
    Hover, HoverContents, HoverParams, LanguageString, MarkedString, MarkupContent, MarkupKind,
};
use rowan::{
    ast::{support::child, AstNode},
    GreenNode, NodeOrToken,
};
use wat_syntax::{
    ast::{GlobalType, TypeUse},
    SyntaxKind, SyntaxNode,
};

impl LanguageService {
    pub fn hover(&self, params: HoverParams) -> Option<Hover> {
        let uri = self.ctx.uri(
            params
                .text_document_position_params
                .text_document
                .uri
                .clone(),
        );
        let token = find_meaningful_token(
            &self.ctx,
            uri,
            params.text_document_position_params.position,
        )?;
        let line_index = self.ctx.line_index(uri);

        match token.kind() {
            SyntaxKind::IDENT | SyntaxKind::INT | SyntaxKind::UNSIGNED_INT => {
                let symbol_table = self.ctx.symbol_table(uri);

                let parent = token.parent()?;
                let key = parent.into();
                symbol_table
                    .find_param_def(&key)
                    .or_else(|| symbol_table.find_local_def(&key))
                    .or_else(|| {
                        symbol_table.symbols.iter().find(|symbol| {
                            symbol.key == key
                                && matches!(
                                    symbol.kind,
                                    SymbolItemKind::Param(..) | SymbolItemKind::Local(..)
                                )
                        })
                    })
                    .map(|symbol| {
                        let mut content_value = '('.to_string();
                        match &symbol.kind {
                            SymbolItemKind::Param(idx) => {
                                content_value.push_str("param");
                                if let Some(name) = &idx.name {
                                    content_value.push(' ');
                                    content_value.push_str(name);
                                }
                            }
                            SymbolItemKind::Local(idx) => {
                                content_value.push_str("local");
                                if let Some(name) = &idx.name {
                                    content_value.push(' ');
                                    content_value.push_str(name);
                                }
                            }
                            _ => {}
                        }
                        self.ctx
                            .extract_types(symbol.key.green.clone())
                            .into_iter()
                            .for_each(|ty| {
                                content_value.push(' ');
                                content_value.push_str(&ty.to_string());
                            });
                        content_value.push(')');
                        Hover {
                            contents: HoverContents::Scalar(create_marked_string(content_value)),
                            range: Some(helpers::rowan_range_to_lsp_range(
                                &line_index,
                                token.text_range(),
                            )),
                        }
                    })
                    .or_else(|| {
                        symbol_table.find_global_defs(&key).map(|symbols| {
                            let root = self.ctx.root(uri);
                            Hover {
                                contents: HoverContents::Array(
                                    symbols
                                        .map(|symbol| create_global_def_hover(symbol, &root))
                                        .collect(),
                                ),
                                range: Some(helpers::rowan_range_to_lsp_range(
                                    &line_index,
                                    token.text_range(),
                                )),
                            }
                        })
                    })
                    .or_else(|| {
                        symbol_table
                            .symbols
                            .iter()
                            .find(|symbol| {
                                symbol.key == key
                                    && matches!(symbol.kind, SymbolItemKind::GlobalDef(..))
                            })
                            .map(|symbol| {
                                let root = self.ctx.root(uri);
                                Hover {
                                    contents: HoverContents::Scalar(create_global_def_hover(
                                        symbol, &root,
                                    )),
                                    range: Some(helpers::rowan_range_to_lsp_range(
                                        &line_index,
                                        token.text_range(),
                                    )),
                                }
                            })
                    })
                    .or_else(|| {
                        symbol_table.find_func_defs(&key).map(|symbols| {
                            let root = self.ctx.root(uri);
                            Hover {
                                contents: HoverContents::Array(
                                    symbols
                                        .map(|symbol| {
                                            create_func_hover(
                                                &self.ctx,
                                                symbol,
                                                &symbol_table,
                                                &root,
                                            )
                                        })
                                        .collect(),
                                ),
                                range: Some(helpers::rowan_range_to_lsp_range(
                                    &line_index,
                                    token.text_range(),
                                )),
                            }
                        })
                    })
                    .or_else(|| {
                        symbol_table
                            .symbols
                            .iter()
                            .find(|symbol| {
                                symbol.key == key && matches!(symbol.kind, SymbolItemKind::Func(..))
                            })
                            .map(|symbol| {
                                let root = self.ctx.root(uri);
                                Hover {
                                    contents: HoverContents::Scalar(create_func_hover(
                                        &self.ctx,
                                        symbol,
                                        &symbol_table,
                                        &root,
                                    )),
                                    range: Some(helpers::rowan_range_to_lsp_range(
                                        &line_index,
                                        token.text_range(),
                                    )),
                                }
                            })
                    })
                    .or_else(|| {
                        symbol_table.find_type_use_defs(&key).map(|symbols| Hover {
                            contents: HoverContents::Array(
                                symbols
                                    .map(|symbol| create_type_def_hover(&self.ctx, symbol))
                                    .collect(),
                            ),
                            range: Some(helpers::rowan_range_to_lsp_range(
                                &line_index,
                                token.text_range(),
                            )),
                        })
                    })
                    .or_else(|| {
                        symbol_table
                            .symbols
                            .iter()
                            .find(|symbol| {
                                symbol.key == key && matches!(symbol.kind, SymbolItemKind::Type(..))
                            })
                            .map(|symbol| Hover {
                                contents: HoverContents::Scalar(create_type_def_hover(
                                    &self.ctx, symbol,
                                )),
                                range: Some(helpers::rowan_range_to_lsp_range(
                                    &line_index,
                                    token.text_range(),
                                )),
                            })
                    })
            }
            SyntaxKind::NUM_TYPE => match token.text() {
                "i32" => Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: I32_DOC.into(),
                    }),
                    range: Some(helpers::rowan_range_to_lsp_range(
                        &line_index,
                        token.text_range(),
                    )),
                }),
                "i64" => Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: I64_DOC.into(),
                    }),
                    range: Some(helpers::rowan_range_to_lsp_range(
                        &line_index,
                        token.text_range(),
                    )),
                }),
                "f32" => Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: F32_DOC.into(),
                    }),
                    range: Some(helpers::rowan_range_to_lsp_range(
                        &line_index,
                        token.text_range(),
                    )),
                }),
                "f64" => Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: F64_DOC.into(),
                    }),
                    range: Some(helpers::rowan_range_to_lsp_range(
                        &line_index,
                        token.text_range(),
                    )),
                }),
                _ => None,
            },
            SyntaxKind::VEC_TYPE => Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: V128_DOC.into(),
                }),
                range: Some(helpers::rowan_range_to_lsp_range(
                    &line_index,
                    token.text_range(),
                )),
            }),
            SyntaxKind::REF_TYPE => match token.text() {
                "funcref" => Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: FUNC_REF_DOC.into(),
                    }),
                    range: Some(helpers::rowan_range_to_lsp_range(
                        &line_index,
                        token.text_range(),
                    )),
                }),
                "externref" => Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: EXTERN_REF_DOC.into(),
                    }),
                    range: Some(helpers::rowan_range_to_lsp_range(
                        &line_index,
                        token.text_range(),
                    )),
                }),
                _ => None,
            },
            _ => None,
        }
    }
}

fn create_global_def_hover(symbol: &SymbolItem, root: &SyntaxNode) -> MarkedString {
    let mut content_value = '('.to_string();
    if let SymbolItemKind::GlobalDef(idx) = &symbol.kind {
        content_value.push_str("global");
        if let Some(name) = &idx.name {
            content_value.push(' ');
            content_value.push_str(name);
        }
    }
    let node = symbol.key.ptr.to_node(root);
    if let Some(global_type) = child::<GlobalType>(&node) {
        let mutable = global_type.mut_keyword().is_some();
        if mutable {
            content_value.push_str(" (mut");
        }
        if let Some(val_type) = global_type.val_type() {
            content_value.push(' ');
            content_value.push_str(&val_type.syntax().to_string());
        }
        if mutable {
            content_value.push(')');
        }
    }
    content_value.push(')');
    create_marked_string(content_value)
}

fn create_func_hover(
    ctx: &LanguageServiceCtx,
    symbol: &SymbolItem,
    symbol_table: &SymbolTable,
    root: &SyntaxNode,
) -> MarkedString {
    let mut content_value = "(func".to_string();
    if let SymbolItemKind::Func(DefIdx {
        name: Some(name), ..
    }) = &symbol.kind
    {
        content_value.push(' ');
        content_value.push_str(name);
    }
    if let Some(type_use) = symbol.key.green.children().find_map(|child| match child {
        NodeOrToken::Node(node) if node.kind() == SyntaxKind::TYPE_USE.into() => Some(node),
        _ => None,
    }) {
        content_value.push(' ');
        if type_use
            .children()
            .filter(|child| {
                let kind = child.kind();
                kind == SyntaxKind::PARAM.into() || kind == SyntaxKind::RESULT.into()
            })
            .next()
            .is_none()
        {
            let node = symbol.key.ptr.to_node(root);
            if let Some(func_type) = child::<TypeUse>(&node)
                .and_then(|type_use| type_use.index())
                .and_then(|idx| symbol_table.find_type_use_defs(&idx.syntax().clone().into()))
                .and_then(|mut symbols| symbols.next())
                .and_then(|symbol| find_func_type_of_type_def(&symbol.key.green))
            {
                let sig = ctx.extract_func_sig(func_type);
                content_value.push_str(&sig.to_string());
            }
        } else {
            let sig = ctx.extract_func_sig(type_use.to_owned());
            content_value.push_str(&sig.to_string());
        }
    }
    content_value.push(')');
    create_marked_string(content_value)
}

fn create_type_def_hover(ctx: &LanguageServiceCtx, symbol: &SymbolItem) -> MarkedString {
    let mut content_value = "(type".to_string();
    if let SymbolItemKind::Type(DefIdx {
        name: Some(name), ..
    }) = &symbol.kind
    {
        content_value.push(' ');
        content_value.push_str(name);
    }
    if let Some(func_type) = find_func_type_of_type_def(&symbol.key.green) {
        let sig = ctx.extract_func_sig(func_type.to_owned());
        content_value.push_str(" (func ");
        content_value.push_str(&sig.to_string());
        content_value.push(')');
    }
    content_value.push(')');
    create_marked_string(content_value)
}

fn find_func_type_of_type_def(green: &GreenNode) -> Option<GreenNode> {
    green.children().find_map(|child| match child {
        NodeOrToken::Node(node) if node.kind() == SyntaxKind::FUNC_TYPE.into() => {
            Some(node.to_owned())
        }
        _ => None,
    })
}

fn create_marked_string(value: String) -> MarkedString {
    MarkedString::LanguageString(LanguageString {
        language: "wat".into(),
        value,
    })
}

const I32_DOC: &str = "```wat
i32
```

The types `i32` classify 32 bit integers.";

const I64_DOC: &str = "```wat
i64
```

The types `i64` classify 64 bit integers.";

const F32_DOC: &str = "```wat
f32
```

The types `f32` classify 32 bit floating-point data.";

const F64_DOC: &str = "```wat
f64
```

The types `f64` classify 64 bit floating-point data.";

const V128_DOC: &str = "```wat
v128
```

The type `v128` corresponds to a 128 bit vector of packed integer or floating-point data.";

const FUNC_REF_DOC: &str =  "```wat
funcref
```

The type [`funcref`](https://webassembly.github.io/spec/core/syntax/types.html#syntax-reftype)
denotes the infinite union of all references to [functions](https://webassembly.github.io/spec/core/syntax/modules.html#syntax-func),
regardless of their [function types](https://webassembly.github.io/spec/core/syntax/types.html#syntax-functype).";

const EXTERN_REF_DOC: &str =  "```wat
externref
```

The type [`externref`](https://webassembly.github.io/spec/core/syntax/types.html#syntax-reftype)
denotes the infinite union of all references to objects owned by the [embedder](https://webassembly.github.io/spec/core/intro/overview.html#embedder)
and that can be passed into WebAssembly under this type.";
