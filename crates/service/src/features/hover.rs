use super::find_meaningful_token;
use crate::{
    binder::{Symbol, SymbolKey, SymbolKind, SymbolTablesCtx},
    data_set, helpers,
    idx::IdentsCtx,
    syntax_tree::SyntaxTreeCtx,
    types_analyzer::{self, CompositeType, DefType, TypesAnalyzerCtx},
    uri::{InternUri, UrisCtx},
    LanguageService,
};
use lspt::{Hover, HoverParams, MarkupContent, MarkupKind, Union3};
use rowan::ast::{support::child, AstNode};
use std::fmt::Write;
use wat_syntax::{
    ast::{GlobalType, Limits, MemoryType, PlainInstr, TableType},
    SyntaxKind, SyntaxNode,
};

impl LanguageService {
    /// Handler for `textDocument/hover` request.
    pub fn hover(&self, params: HoverParams) -> Option<Hover> {
        let uri = self.uri(params.text_document.uri.clone());
        let root = SyntaxNode::new_root(self.root(uri));
        let token = find_meaningful_token(self, uri, &root, params.position)?;
        let line_index = self.line_index(uri);
        let symbol_table = self.symbol_table(uri);

        match token.kind() {
            SyntaxKind::IDENT | SyntaxKind::INT | SyntaxKind::UNSIGNED_INT => {
                let parent = token.parent()?;
                let key = SymbolKey::new(&parent);
                symbol_table
                    .find_param_or_local_def(key)
                    .map(|symbol| Hover {
                        contents: Union3::A(create_param_or_local_hover(self, symbol)),
                        range: Some(helpers::rowan_range_to_lsp_range(
                            &line_index,
                            token.text_range(),
                        )),
                    })
                    .or_else(|| {
                        symbol_table
                            .symbols
                            .iter()
                            .find(|symbol| symbol.key == key)
                            .and_then(|symbol| match symbol.kind {
                                SymbolKind::Call => symbol_table.find_def(key).map(|symbol| {
                                    let contents =
                                        create_func_hover(self, uri, symbol.clone(), &root);
                                    Hover {
                                        contents: Union3::A(MarkupContent {
                                            kind: MarkupKind::Markdown,
                                            value: contents,
                                        }),
                                        range: Some(helpers::rowan_range_to_lsp_range(
                                            &line_index,
                                            token.text_range(),
                                        )),
                                    }
                                }),
                                SymbolKind::TypeUse => {
                                    symbol_table.find_def(key).map(|symbol| Hover {
                                        contents: Union3::A(create_type_def_hover(
                                            self, uri, symbol,
                                        )),
                                        range: Some(helpers::rowan_range_to_lsp_range(
                                            &line_index,
                                            token.text_range(),
                                        )),
                                    })
                                }
                                SymbolKind::GlobalRef => {
                                    symbol_table.find_def(key).map(|symbol| Hover {
                                        contents: Union3::A(create_global_def_hover(
                                            self, symbol, &root,
                                        )),
                                        range: Some(helpers::rowan_range_to_lsp_range(
                                            &line_index,
                                            token.text_range(),
                                        )),
                                    })
                                }
                                SymbolKind::MemoryRef => {
                                    symbol_table.find_def(key).map(|symbol| Hover {
                                        contents: Union3::A(create_memory_def_hover(
                                            self, symbol, &root,
                                        )),
                                        range: Some(helpers::rowan_range_to_lsp_range(
                                            &line_index,
                                            token.text_range(),
                                        )),
                                    })
                                }
                                SymbolKind::TableRef => {
                                    symbol_table.find_def(key).map(|symbol| Hover {
                                        contents: Union3::A(create_table_def_hover(
                                            self, symbol, &root,
                                        )),
                                        range: Some(helpers::rowan_range_to_lsp_range(
                                            &line_index,
                                            token.text_range(),
                                        )),
                                    })
                                }
                                SymbolKind::BlockRef => symbol_table
                                    .find_block_def(key)
                                    .and_then(|def_key| {
                                        symbol_table
                                            .symbols
                                            .iter()
                                            .find(|symbol| symbol.key == def_key)
                                    })
                                    .map(|block| Hover {
                                        contents: Union3::A(create_block_hover(
                                            self, block, uri, &root,
                                        )),
                                        range: Some(helpers::rowan_range_to_lsp_range(
                                            &line_index,
                                            token.text_range(),
                                        )),
                                    }),
                                SymbolKind::FieldRef => {
                                    symbol_table.find_def(key).map(|symbol| Hover {
                                        contents: Union3::A(create_field_def_hover(
                                            self, symbol, uri,
                                        )),
                                        range: Some(helpers::rowan_range_to_lsp_range(
                                            &line_index,
                                            token.text_range(),
                                        )),
                                    })
                                }
                                _ => None,
                            })
                    })
                    .or_else(|| {
                        symbol_table
                            .symbols
                            .iter()
                            .find(|symbol| symbol.key == key)
                            .and_then(|symbol| create_def_hover(self, uri, &root, symbol))
                            .map(|contents| Hover {
                                contents: Union3::A(contents),
                                range: Some(helpers::rowan_range_to_lsp_range(
                                    &line_index,
                                    token.text_range(),
                                )),
                            })
                    })
            }
            SyntaxKind::TYPE_KEYWORD => {
                let ty = token.text();
                data_set::get_value_type_description(token.text()).map(|doc| Hover {
                    contents: Union3::A(MarkupContent {
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
                let node = token.parent()?;
                let node = if node.kind() == SyntaxKind::REF_TYPE {
                    node.parent()?
                } else {
                    node
                };
                let key = SymbolKey::new(&node);

                symbol_table
                    .symbols
                    .iter()
                    .find(|symbol| symbol.key == key)
                    .and_then(|symbol| create_def_hover(self, uri, &root, symbol))
                    .map(|contents| Hover {
                        contents: Union3::A(contents),
                        range: Some(helpers::rowan_range_to_lsp_range(
                            &line_index,
                            if matches!(token.text(), "mut" | "ref") {
                                node.text_range()
                            } else {
                                token.text_range()
                            },
                        )),
                    })
            }
            SyntaxKind::INSTR_NAME => {
                let name = token.text();
                let key = match name {
                    "select" => {
                        let parent = token.parent().and_then(PlainInstr::cast)?;
                        if parent.immediates().count() > 0 {
                            "select."
                        } else {
                            "select"
                        }
                    }
                    "ref.test" => {
                        let parent = token.parent().and_then(PlainInstr::cast)?;
                        if parent
                            .immediates()
                            .next()
                            .and_then(|immediate| immediate.ref_type())
                            .is_some_and(|ref_type| helpers::ast::is_nullable_ref_type(&ref_type))
                        {
                            "ref.test."
                        } else {
                            "ref.test"
                        }
                    }
                    "ref.cast" => {
                        let parent = token.parent().and_then(PlainInstr::cast)?;
                        if parent
                            .immediates()
                            .next()
                            .and_then(|immediate| immediate.ref_type())
                            .is_some_and(|ref_type| helpers::ast::is_nullable_ref_type(&ref_type))
                        {
                            "ref.cast."
                        } else {
                            "ref.cast"
                        }
                    }
                    name => name,
                };
                data_set::INSTR_OP_CODES.get(key).map(|code| Hover {
                    contents: Union3::A(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: format!(
                            "```wat\n{name}\n```\nBinary Opcode: {}",
                            format_op_code(*code)
                        ),
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
    symbol: &Symbol,
) -> Option<MarkupContent> {
    match symbol.kind {
        SymbolKind::Param | SymbolKind::Local => Some(create_param_or_local_hover(service, symbol)),
        SymbolKind::Func => Some(MarkupContent {
            kind: MarkupKind::Markdown,
            value: create_func_hover(service, uri, symbol.clone(), root),
        }),
        SymbolKind::Type => Some(create_type_def_hover(service, uri, symbol)),
        SymbolKind::GlobalDef => Some(create_global_def_hover(service, symbol, root)),
        SymbolKind::MemoryDef => Some(create_memory_def_hover(service, symbol, root)),
        SymbolKind::TableDef => Some(create_table_def_hover(service, symbol, root)),
        SymbolKind::BlockDef => Some(create_block_hover(service, symbol, uri, root)),
        SymbolKind::FieldDef => Some(create_field_def_hover(service, symbol, uri)),
        _ => None,
    }
}

fn create_func_hover(
    service: &LanguageService,
    uri: InternUri,
    symbol: Symbol,
    root: &SyntaxNode,
) -> String {
    let node = symbol.key.to_node(root);
    let doc = helpers::ast::get_doc_comment(&node);
    let mut content = format!(
        "```wat\n{}\n```",
        service.render_func_header(
            symbol.idx.name,
            service.get_func_sig(uri, symbol.key, symbol.green)
        )
    );
    if !doc.is_empty() {
        content.push_str("\n---\n");
        content.push_str(&doc);
    }
    content
}

fn create_param_or_local_hover(service: &LanguageService, symbol: &Symbol) -> MarkupContent {
    let mut content = '('.to_string();
    match symbol.kind {
        SymbolKind::Param => {
            content.push_str("param");
        }
        SymbolKind::Local => {
            content.push_str("local");
        }
        _ => {}
    }
    if let Some(name) = symbol.idx.name {
        content.push(' ');
        content.push_str(&service.lookup_ident(name));
    }
    if let Some(ty) = service.extract_type(symbol.green.clone()) {
        content.push(' ');
        let _ = write!(content, "{}", ty.render(service));
    }
    content.push(')');
    MarkupContent {
        kind: MarkupKind::Markdown,
        value: format!("```wat\n{content}\n```"),
    }
}

fn create_global_def_hover(
    service: &LanguageService,
    symbol: &Symbol,
    root: &SyntaxNode,
) -> MarkupContent {
    let mut content = "(global".to_string();
    if let Some(name) = symbol.idx.name {
        content.push(' ');
        content.push_str(&service.lookup_ident(name));
    }
    let node = symbol.key.to_node(root);
    if let Some(global_type) = child::<GlobalType>(&node) {
        let mutable = global_type.mut_keyword().is_some();
        if mutable {
            content.push_str(" (mut");
        }
        if let Some(val_type) = global_type.val_type() {
            content.push(' ');
            content.push_str(&val_type.syntax().to_string());
        }
        if mutable {
            content.push(')');
        }
    }
    content.push(')');
    MarkupContent {
        kind: MarkupKind::Markdown,
        value: format!("```wat\n{content}\n```"),
    }
}

fn create_memory_def_hover(
    service: &LanguageService,
    symbol: &Symbol,
    root: &SyntaxNode,
) -> MarkupContent {
    let mut content = "(memory".to_string();
    if let Some(name) = symbol.idx.name {
        content.push(' ');
        content.push_str(&service.lookup_ident(name));
    }
    let node = symbol.key.to_node(root);
    if let Some(limits) = child::<MemoryType>(&node)
        .and_then(|memory_type| memory_type.limits())
        .and_then(|limits| render_limits(&limits))
    {
        content.push(' ');
        content.push_str(&limits);
    }
    content.push(')');
    MarkupContent {
        kind: MarkupKind::Markdown,
        value: format!("```wat\n{content}\n```"),
    }
}

fn create_table_def_hover(
    service: &LanguageService,
    symbol: &Symbol,
    root: &SyntaxNode,
) -> MarkupContent {
    use crate::types_analyzer::RefType;

    let mut content = "(table".to_string();
    if let Some(name) = symbol.idx.name {
        content.push(' ');
        content.push_str(&service.lookup_ident(name));
    }
    let node = symbol.key.to_node(root);
    if let Some(table_type) = child::<TableType>(&node) {
        if let Some(limits) = table_type
            .limits()
            .and_then(|limits| render_limits(&limits))
        {
            content.push(' ');
            content.push_str(&limits);
        }
        if let Some(ref_type) = table_type
            .ref_type()
            .and_then(|ref_type| RefType::from_green(&ref_type.syntax().green(), service))
        {
            content.push(' ');
            let _ = write!(content, "{}", ref_type.render(service));
        }
    }
    content.push(')');
    MarkupContent {
        kind: MarkupKind::Markdown,
        value: format!("```wat\n{content}\n```"),
    }
}

fn create_type_def_hover(
    service: &LanguageService,
    uri: InternUri,
    symbol: &Symbol,
) -> MarkupContent {
    let def_types = service.def_types(uri);
    let mut content = "(type".to_string();
    if let Some(name) = symbol.idx.name {
        content.push(' ');
        content.push_str(&service.lookup_ident(name));
    }
    if let Some(DefType { comp, .. }) = def_types.iter().find(|def_type| def_type.key == symbol.key)
    {
        content.push(' ');
        match comp {
            CompositeType::Func(sig) => {
                content.push_str("(func");
                if !sig.params.is_empty() || !sig.results.is_empty() {
                    content.push(' ');
                    content.push_str(&service.render_sig(sig.clone()));
                }
                content.push(')');
            }
            CompositeType::Struct(fields) => {
                content.push_str("(struct");
                if !fields.0.is_empty() {
                    content.push(' ');
                    let _ = write!(content, "{}", fields.render(service));
                }
                content.push(')');
            }
            CompositeType::Array(field_ty) => {
                content.push_str("(array");
                if let Some(field_ty) = field_ty {
                    content.push(' ');
                    let _ = write!(content, "{}", field_ty.render(service));
                }
                content.push(')');
            }
        }
    }
    content.push(')');
    MarkupContent {
        kind: MarkupKind::Markdown,
        value: format!("```wat\n{content}\n```"),
    }
}

fn create_block_hover(
    service: &LanguageService,
    symbol: &Symbol,
    uri: InternUri,
    root: &SyntaxNode,
) -> MarkupContent {
    let content = service.render_block_header(
        symbol.key.kind(),
        symbol.idx.name,
        types_analyzer::get_block_sig(service, uri, &symbol.key.to_node(root)),
    );
    MarkupContent {
        kind: MarkupKind::Markdown,
        value: format!("```wat\n{content}\n```"),
    }
}

fn create_field_def_hover(
    service: &LanguageService,
    symbol: &Symbol,
    uri: InternUri,
) -> MarkupContent {
    let mut content = "(field".to_string();
    if let Some(name) = symbol.idx.name {
        content.push(' ');
        content.push_str(&service.lookup_ident(name));
    }
    if let Some(ty) = service.resolve_field_type(uri, symbol.key, symbol.region) {
        let _ = write!(content, " {}", ty.render(service));
    }
    content.push(')');
    MarkupContent {
        kind: MarkupKind::Markdown,
        value: format!("```wat\n{content}\n```"),
    }
}

fn format_op_code(code: u32) -> String {
    if code >> 16 > 0 {
        format!(
            "0x{:02X} 0x{:02X} 0x{:02X}",
            code >> 16,
            (code >> 8) & 0xFF,
            code & 0xFF
        )
    } else if code >> 8 > 0 {
        format!("0x{:02X} 0x{:02X}", code >> 8, code & 0xFF)
    } else {
        format!("0x{code:02X}")
    }
}

fn render_limits(limits: &Limits) -> Option<String> {
    let mut content = String::with_capacity(2);
    content.push_str(limits.min()?.text());
    if let Some(max) = limits.max() {
        content.push(' ');
        content.push_str(max.text());
    }
    Some(content)
}
