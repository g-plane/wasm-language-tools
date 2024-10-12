use super::find_meaningful_token;
use crate::{
    binder::{DefIdx, SymbolItem, SymbolItemKind, SymbolTable, SymbolTablesCtx},
    dataset,
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
                    .map(|symbol| Hover {
                        contents: HoverContents::Scalar(create_param_or_local_hover(
                            &self.ctx, symbol,
                        )),
                        range: Some(helpers::rowan_range_to_lsp_range(
                            &line_index,
                            token.text_range(),
                        )),
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
                            .find(|symbol| symbol.key == key)
                            .and_then(|symbol| {
                                let content = match symbol.kind {
                                    SymbolItemKind::Param(..) | SymbolItemKind::Local(..) => {
                                        create_param_or_local_hover(&self.ctx, symbol)
                                    }
                                    SymbolItemKind::Func(..) => create_func_hover(
                                        &self.ctx,
                                        symbol,
                                        &symbol_table,
                                        &self.ctx.root(uri),
                                    ),
                                    SymbolItemKind::Type(..) => {
                                        create_type_def_hover(&self.ctx, symbol)
                                    }
                                    SymbolItemKind::GlobalDef(..) => {
                                        create_global_def_hover(symbol, &self.ctx.root(uri))
                                    }
                                    _ => return None,
                                };
                                Some(Hover {
                                    contents: HoverContents::Scalar(content),
                                    range: Some(helpers::rowan_range_to_lsp_range(
                                        &line_index,
                                        token.text_range(),
                                    )),
                                })
                            })
                    })
            }
            SyntaxKind::NUM_TYPE | SyntaxKind::VEC_TYPE | SyntaxKind::REF_TYPE => {
                let ty = token.text();
                dataset::get_value_type_description(token.text()).map(|doc| Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: format!("```wat\n{ty}\n```\n\n{doc}"),
                    }),
                    range: Some(helpers::rowan_range_to_lsp_range(
                        &line_index,
                        token.text_range(),
                    )),
                })
            }
            _ => None,
        }
    }
}

fn create_param_or_local_hover(ctx: &LanguageServiceCtx, symbol: &SymbolItem) -> MarkedString {
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
    ctx.extract_types(symbol.key.green.clone())
        .into_iter()
        .for_each(|ty| {
            content_value.push(' ');
            content_value.push_str(&ty.to_string());
        });
    content_value.push(')');
    create_marked_string(content_value)
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
        if type_use.children().any(|child| {
            let kind = child.kind();
            kind == SyntaxKind::PARAM.into() || kind == SyntaxKind::RESULT.into()
        }) {
            let sig = ctx.extract_func_sig(type_use.to_owned());
            content_value.push_str(&sig.to_string());
        } else {
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
