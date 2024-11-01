use super::find_meaningful_token;
use crate::{
    binder::{DefIdx, SymbolItem, SymbolItemKind, SymbolTablesCtx},
    data_set,
    files::FilesCtx,
    helpers,
    types_analyzer::TypesAnalyzerCtx,
    InternUri, LanguageService,
};
use lsp_types::{
    Hover, HoverContents, HoverParams, LanguageString, MarkedString, MarkupContent, MarkupKind,
};
use rowan::ast::{support::child, AstNode};
use wat_syntax::{ast::GlobalType, SyntaxKind, SyntaxNode};

impl LanguageService {
    pub fn hover(&self, params: HoverParams) -> Option<Hover> {
        let uri = self.uri(
            params
                .text_document_position_params
                .text_document
                .uri
                .clone(),
        );
        let root = SyntaxNode::new_root(self.root(uri));
        let token = find_meaningful_token(
            self,
            uri,
            &root,
            params.text_document_position_params.position,
        )?;
        let line_index = self.line_index(uri);

        match token.kind() {
            SyntaxKind::IDENT | SyntaxKind::INT | SyntaxKind::UNSIGNED_INT => {
                let symbol_table = self.symbol_table(uri);

                let parent = token.parent()?;
                let key = parent.into();
                symbol_table
                    .find_param_or_local_def(&key)
                    .map(|symbol| Hover {
                        contents: HoverContents::Scalar(create_param_or_local_hover(self, symbol)),
                        range: Some(helpers::rowan_range_to_lsp_range(
                            &line_index,
                            token.text_range(),
                        )),
                    })
                    .or_else(|| {
                        symbol_table.find_global_defs(&key).map(|symbols| Hover {
                            contents: HoverContents::Array(
                                symbols
                                    .map(|symbol| create_global_def_hover(self, symbol, &root))
                                    .collect(),
                            ),
                            range: Some(helpers::rowan_range_to_lsp_range(
                                &line_index,
                                token.text_range(),
                            )),
                        })
                    })
                    .or_else(|| {
                        symbol_table.find_func_defs(&key).map(|symbols| Hover {
                            contents: HoverContents::Array(
                                symbols
                                    .map(|symbol| {
                                        create_marked_string(
                                            self.render_func_header(uri, symbol.clone().into()),
                                        )
                                    })
                                    .collect(),
                            ),
                            range: Some(helpers::rowan_range_to_lsp_range(
                                &line_index,
                                token.text_range(),
                            )),
                        })
                    })
                    .or_else(|| {
                        symbol_table.find_type_use_defs(&key).map(|symbols| Hover {
                            contents: HoverContents::Array(
                                symbols
                                    .map(|symbol| create_type_def_hover(self, symbol))
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
                            .and_then(|symbol| create_def_hover(self, uri, &root, symbol))
                            .map(|contents| Hover {
                                contents,
                                range: Some(helpers::rowan_range_to_lsp_range(
                                    &line_index,
                                    token.text_range(),
                                )),
                            })
                    })
            }
            SyntaxKind::NUM_TYPE | SyntaxKind::VEC_TYPE | SyntaxKind::REF_TYPE => {
                let ty = token.text();
                data_set::get_value_type_description(token.text()).map(|doc| Hover {
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
            SyntaxKind::KEYWORD => {
                let parent = token.parent()?;
                let key = parent.into();

                let symbol_table = self.symbol_table(uri);
                symbol_table
                    .symbols
                    .iter()
                    .find(|symbol| symbol.key == key)
                    .and_then(|symbol| create_def_hover(self, uri, &root, symbol))
                    .map(|contents| Hover {
                        contents,
                        range: Some(helpers::rowan_range_to_lsp_range(
                            &line_index,
                            token.text_range(),
                        )),
                    })
            }
            SyntaxKind::INSTR_NAME => {
                let name = token.text();
                data_set::INSTR_METAS.get(name).map(|instr| Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: format!("```wat\n{name}\n```\nBinary Opcode: {}", instr.bin_op),
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

fn create_def_hover(
    service: &LanguageService,
    uri: InternUri,
    root: &SyntaxNode,
    symbol: &SymbolItem,
) -> Option<HoverContents> {
    let content = match symbol.kind {
        SymbolItemKind::Param(..) | SymbolItemKind::Local(..) => {
            create_param_or_local_hover(service, symbol)
        }
        SymbolItemKind::Func(..) => {
            create_marked_string(service.render_func_header(uri, symbol.clone().into()))
        }
        SymbolItemKind::Type(..) => create_type_def_hover(service, symbol),
        SymbolItemKind::GlobalDef(..) => create_global_def_hover(service, symbol, root),
        _ => return None,
    };
    Some(HoverContents::Scalar(content))
}

fn create_param_or_local_hover(service: &LanguageService, symbol: &SymbolItem) -> MarkedString {
    let mut content_value = '('.to_string();
    match &symbol.kind {
        SymbolItemKind::Param(idx) => {
            content_value.push_str("param");
            if let Some(name) = idx.name {
                content_value.push(' ');
                content_value.push_str(&service.lookup_ident(name));
            }
        }
        SymbolItemKind::Local(idx) => {
            content_value.push_str("local");
            if let Some(name) = idx.name {
                content_value.push(' ');
                content_value.push_str(&service.lookup_ident(name));
            }
        }
        _ => {}
    }
    if let Some(ty) = service.extract_type(symbol.green.clone()) {
        content_value.push(' ');
        content_value.push_str(&ty.to_string());
    }
    content_value.push(')');
    create_marked_string(content_value)
}

fn create_global_def_hover(
    service: &LanguageService,
    symbol: &SymbolItem,
    root: &SyntaxNode,
) -> MarkedString {
    let mut content_value = '('.to_string();
    if let SymbolItemKind::GlobalDef(idx) = &symbol.kind {
        content_value.push_str("global");
        if let Some(name) = idx.name {
            content_value.push(' ');
            content_value.push_str(&service.lookup_ident(name));
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

fn create_type_def_hover(service: &LanguageService, symbol: &SymbolItem) -> MarkedString {
    let mut content_value = "(type".to_string();
    if let SymbolItemKind::Type(DefIdx {
        name: Some(name), ..
    }) = symbol.kind
    {
        content_value.push(' ');
        content_value.push_str(&service.lookup_ident(name));
    }
    if let Some(func_type) = helpers::ast::find_func_type_of_type_def(&symbol.green) {
        let sig = service.extract_sig(func_type.to_owned());
        content_value.push_str(" (func");
        if !sig.params.is_empty() || !sig.results.is_empty() {
            content_value.push(' ');
            content_value.push_str(&sig.to_string());
        }
        content_value.push(')');
    }
    content_value.push(')');
    create_marked_string(content_value)
}

fn create_marked_string(value: String) -> MarkedString {
    MarkedString::LanguageString(LanguageString {
        language: "wat".into(),
        value,
    })
}
