use crate::{
    LanguageService,
    binder::{Symbol, SymbolKey, SymbolKind, SymbolTable},
    data_set,
    document::Document,
    helpers::{self, LineIndexExt},
    mutability,
    types_analyzer::{self, CompositeType, DefType, RefType},
};
use lspt::{Hover, HoverParams, MarkupContent, MarkupKind, Union3};
use std::fmt::Write;
use wat_syntax::{
    NodeOrToken, SyntaxKind, SyntaxNode,
    ast::{AstNode, Limits, MemType, PlainInstr, TableType, support},
};

impl LanguageService {
    /// Handler for `textDocument/hover` request.
    pub fn hover(&self, params: HoverParams) -> Option<Hover> {
        let document = self.get_document(params.text_document.uri)?;
        let root = document.root_tree(self);
        let token = super::find_meaningful_token(self, document, &root, params.position)?;
        let line_index = document.line_index(self);
        let symbol_table = SymbolTable::of(self, document);

        match token.kind() {
            SyntaxKind::IDENT | SyntaxKind::UNSIGNED_INT => {
                create_symbol_hover(self, document, &root, symbol_table, &token.parent()).map(|contents| Hover {
                    contents: Union3::A(contents),
                    range: Some(line_index.convert(token.text_range())),
                })
            }
            SyntaxKind::INT => {
                let parent = token.parent();
                create_symbol_hover(self, document, &root, symbol_table, &parent)
                    .or_else(|| {
                        if let Some(is_i32) =
                            parent
                                .prev_siblings_with_tokens()
                                .find_map(|node_or_token| match node_or_token {
                                    NodeOrToken::Token(token) if token.kind() == SyntaxKind::INSTR_NAME => {
                                        token.text().split_once('.').and_then(|(left, right)| {
                                            if right == "const" {
                                                match left {
                                                    "i32" => Some(true),
                                                    "i64" => Some(false),
                                                    _ => None,
                                                }
                                            } else {
                                                None
                                            }
                                        })
                                    }
                                    _ => None,
                                })
                        {
                            let text = token.text();
                            let (sign, rest) = if let Some(rest) = text.strip_prefix('-') {
                                ("-", rest)
                            } else if let Some(rest) = text.strip_prefix('+') {
                                ("", rest)
                            } else {
                                ("", text)
                            };
                            let value = helpers::parse_u32(rest).ok()?;
                            Some(MarkupContent {
                                kind: MarkupKind::Markdown,
                                value: format!(
                                    "```wat\n{}\n```\n---\n- Dec: `{sign}{value}`\n- Hex: `{sign}0x{value:X}`\n- Bin: `{sign}0b{value:b}`",
                                    if is_i32 { "i32" } else { "i64" },
                                ),
                            })
                        } else {
                            None
                        }
                    })
                    .map(|contents| Hover {
                        contents: Union3::A(contents),
                        range: Some(line_index.convert(token.text_range())),
                    })
            }
            SyntaxKind::TYPE_KEYWORD => {
                let ty = token.text();
                data_set::get_value_type_description(token.text()).map(|doc| Hover {
                    contents: Union3::A(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: format!("```wat\n{ty}\n```\n\n{doc}"),
                    }),
                    range: Some(line_index.convert(token.text_range())),
                })
            }
            SyntaxKind::KEYWORD => {
                let node = token.parent();
                let node = if node.kind() == SyntaxKind::REF_TYPE {
                    node.parent()?
                } else {
                    node
                };
                create_symbol_hover(self, document, &root, symbol_table, &node).map(|contents| Hover {
                    contents: Union3::A(contents),
                    range: Some(line_index.convert(if matches!(token.text(), "mut" | "ref") {
                        node.text_range()
                    } else {
                        token.text_range()
                    })),
                })
            }
            SyntaxKind::INSTR_NAME => {
                let name = token.text();
                match name {
                    "select" => {
                        let parent = PlainInstr::cast(token.parent())?;
                        if parent.immediates().count() > 0 {
                            Some(0x1C)
                        } else {
                            data_set::INSTR_OP_CODES.get("select").copied()
                        }
                    }
                    "ref.test" => {
                        let parent = PlainInstr::cast(token.parent())?;
                        if parent
                            .immediates()
                            .next()
                            .and_then(|immediate| immediate.ref_type())
                            .and_then(|ref_type| RefType::from_green(ref_type.syntax().green(), self))
                            .is_some_and(|ty| ty.nullable)
                        {
                            Some(0xFB15)
                        } else {
                            data_set::INSTR_OP_CODES.get("ref.test").copied()
                        }
                    }
                    "ref.cast" => {
                        let parent = PlainInstr::cast(token.parent())?;
                        if parent
                            .immediates()
                            .next()
                            .and_then(|immediate| immediate.ref_type())
                            .and_then(|ref_type| RefType::from_green(ref_type.syntax().green(), self))
                            .is_some_and(|ty| ty.nullable)
                        {
                            Some(0xFB17)
                        } else {
                            data_set::INSTR_OP_CODES.get("ref.cast").copied()
                        }
                    }
                    name => data_set::INSTR_OP_CODES.get(name).copied(),
                }
                .map(|code| Hover {
                    contents: Union3::A(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: format!("```wat\n{name}\n```\nBinary Opcode: {}", format_op_code(code)),
                    }),
                    range: Some(line_index.convert(token.text_range())),
                })
            }
            _ => None,
        }
    }
}

fn create_symbol_hover(
    db: &dyn salsa::Database,
    document: Document,
    root: &SyntaxNode,
    symbol_table: &SymbolTable,
    node: &SyntaxNode,
) -> Option<MarkupContent> {
    symbol_table
        .symbols
        .get(&SymbolKey::new(node))
        .and_then(|symbol| match symbol.kind {
            SymbolKind::Param | SymbolKind::Local => Some(create_param_or_local_hover(db, symbol)),
            SymbolKind::LocalRef => symbol_table
                .find_def(symbol.key)
                .map(|symbol| create_param_or_local_hover(db, symbol)),
            SymbolKind::Func => Some(create_func_hover(db, document, symbol_table, symbol)),
            SymbolKind::Call => symbol_table
                .find_def(symbol.key)
                .map(|symbol| create_func_hover(db, document, symbol_table, symbol)),
            SymbolKind::Type => Some(create_type_def_hover(db, document, symbol)),
            SymbolKind::TypeUse => symbol_table
                .find_def(symbol.key)
                .map(|symbol| create_type_def_hover(db, document, symbol)),
            SymbolKind::GlobalDef => Some(create_global_def_hover(db, document, symbol)),
            SymbolKind::GlobalRef => symbol_table
                .find_def(symbol.key)
                .map(|symbol| create_global_def_hover(db, document, symbol)),
            SymbolKind::MemoryDef => Some(create_memory_def_hover(db, symbol, root)),
            SymbolKind::MemoryRef => symbol_table
                .find_def(symbol.key)
                .map(|symbol| create_memory_def_hover(db, symbol, root)),
            SymbolKind::TableDef => Some(create_table_def_hover(db, symbol, root)),
            SymbolKind::TableRef => symbol_table
                .find_def(symbol.key)
                .map(|symbol| create_table_def_hover(db, symbol, root)),
            SymbolKind::BlockDef => Some(create_block_hover(db, symbol, document)),
            SymbolKind::BlockRef => symbol_table
                .find_def(symbol.key)
                .map(|symbol| create_block_hover(db, symbol, document)),
            SymbolKind::FieldDef => Some(create_field_def_hover(db, symbol, document)),
            SymbolKind::FieldRef => symbol_table
                .find_def(symbol.key)
                .map(|symbol| create_field_def_hover(db, symbol, document)),
            SymbolKind::TagDef => Some(create_tag_def_hover(db, symbol, document)),
            SymbolKind::TagRef => symbol_table
                .find_def(symbol.key)
                .map(|symbol| create_tag_def_hover(db, symbol, document)),
            SymbolKind::DataDef => Some(create_data_def_hover(db, symbol)),
            SymbolKind::DataRef => symbol_table
                .find_def(symbol.key)
                .map(|symbol| create_data_def_hover(db, symbol)),
            SymbolKind::ElemDef => Some(create_elem_def_hover(db, symbol)),
            SymbolKind::ElemRef => symbol_table
                .find_def(symbol.key)
                .map(|symbol| create_elem_def_hover(db, symbol)),
            SymbolKind::Module => None,
        })
}

fn create_func_hover(
    db: &dyn salsa::Database,
    document: Document,
    symbol_table: &SymbolTable,
    symbol: &Symbol,
) -> MarkupContent {
    let doc = helpers::get_doc_comment(symbol, symbol_table).filter(|doc| !doc.is_empty());
    let mut content = format!(
        "```wat\n{}\n```",
        types_analyzer::render_func_header(
            db,
            symbol.idx.name,
            types_analyzer::get_func_sig(db, document, symbol.key, &symbol.green)
        )
    );
    if let Some(doc) = doc {
        content.push_str("\n---\n");
        content.push_str(&doc);
    }
    MarkupContent {
        kind: MarkupKind::Markdown,
        value: content,
    }
}

fn create_param_or_local_hover(db: &dyn salsa::Database, symbol: &Symbol) -> MarkupContent {
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
        content.push_str(name.ident(db));
    }
    if let Some(ty) = types_analyzer::extract_type(db, &symbol.green) {
        content.push(' ');
        let _ = write!(content, "{}", ty.render(db));
    }
    content.push(')');
    MarkupContent {
        kind: MarkupKind::Markdown,
        value: format!("```wat\n{content}\n```"),
    }
}

fn create_global_def_hover(db: &dyn salsa::Database, document: Document, symbol: &Symbol) -> MarkupContent {
    let mut content = "(global".to_string();
    if let Some(name) = symbol.idx.name {
        content.push(' ');
        content.push_str(name.ident(db));
    }
    let mutable = mutability::get_mutabilities(db, document)
        .get(&symbol.key)
        .and_then(|mutability| mutability.mut_keyword)
        .is_some();
    if mutable {
        content.push_str(" (mut");
    }
    if let Some(ty) = types_analyzer::extract_global_type(db, &symbol.green) {
        content.push(' ');
        let _ = write!(&mut content, "{}", ty.render(db));
    }
    if mutable {
        content.push(')');
    }
    content.push(')');
    MarkupContent {
        kind: MarkupKind::Markdown,
        value: format!("```wat\n{content}\n```"),
    }
}

fn create_memory_def_hover(db: &dyn salsa::Database, symbol: &Symbol, root: &SyntaxNode) -> MarkupContent {
    let mut content = "(memory".to_string();
    if let Some(name) = symbol.idx.name {
        content.push(' ');
        content.push_str(name.ident(db));
    }
    if let Some(limits) = symbol
        .key
        .to_node(root)
        .and_then(|node| support::child::<MemType>(&node))
        .and_then(|mem_type| mem_type.limits())
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

fn create_table_def_hover(db: &dyn salsa::Database, symbol: &Symbol, root: &SyntaxNode) -> MarkupContent {
    use crate::types_analyzer::RefType;

    let mut content = "(table".to_string();
    if let Some(name) = symbol.idx.name {
        content.push(' ');
        content.push_str(name.ident(db));
    }
    if let Some(table_type) = symbol
        .key
        .to_node(root)
        .and_then(|node| support::child::<TableType>(&node))
    {
        if let Some(limits) = table_type.limits().and_then(|limits| render_limits(&limits)) {
            content.push(' ');
            content.push_str(&limits);
        }
        if let Some(ref_type) = table_type
            .ref_type()
            .and_then(|ref_type| RefType::from_green(ref_type.syntax().green(), db))
        {
            content.push(' ');
            let _ = write!(content, "{}", ref_type.render(db));
        }
    }
    content.push(')');
    MarkupContent {
        kind: MarkupKind::Markdown,
        value: format!("```wat\n{content}\n```"),
    }
}

fn create_type_def_hover(db: &dyn salsa::Database, document: Document, symbol: &Symbol) -> MarkupContent {
    let def_types = types_analyzer::get_def_types(db, document);
    let mut content = "(type".to_string();
    if let Some(name) = symbol.idx.name {
        content.push(' ');
        content.push_str(name.ident(db));
    }
    if let Some(DefType { comp, .. }) = def_types.get(&symbol.key) {
        content.push(' ');
        match comp {
            CompositeType::Func(sig) => {
                content.push_str("(func");
                if !sig.params.is_empty() || !sig.results.is_empty() {
                    content.push(' ');
                    let _ = write!(content, "{}", sig.render(db));
                }
                content.push(')');
            }
            CompositeType::Struct(fields) => {
                content.push_str("(struct");
                if !fields.0.is_empty() {
                    content.push(' ');
                    let _ = write!(content, "{}", fields.render(db));
                }
                content.push(')');
            }
            CompositeType::Array(field_ty) => {
                content.push_str("(array");
                if let Some(field_ty) = field_ty {
                    content.push(' ');
                    let _ = write!(content, "{}", field_ty.render(db));
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

fn create_block_hover(db: &dyn salsa::Database, symbol: &Symbol, document: Document) -> MarkupContent {
    let content = types_analyzer::render_block_header(
        db,
        symbol.key.kind(),
        symbol.idx.name,
        types_analyzer::get_func_sig(db, document, symbol.key, &symbol.green),
    );
    MarkupContent {
        kind: MarkupKind::Markdown,
        value: format!("```wat\n{content}\n```"),
    }
}

fn create_field_def_hover(db: &dyn salsa::Database, symbol: &Symbol, document: Document) -> MarkupContent {
    let mut content = "(field".to_string();
    if let Some(name) = symbol.idx.name {
        content.push(' ');
        content.push_str(name.ident(db));
    }
    if let Some(ty) = types_analyzer::resolve_field_type(db, document, symbol.key, symbol.region) {
        let _ = write!(content, " {}", ty.render(db));
    }
    content.push(')');
    MarkupContent {
        kind: MarkupKind::Markdown,
        value: format!("```wat\n{content}\n```"),
    }
}

fn create_tag_def_hover(db: &dyn salsa::Database, symbol: &Symbol, document: Document) -> MarkupContent {
    let content = types_analyzer::render_header(
        db,
        "tag",
        symbol.idx.name,
        types_analyzer::get_func_sig(db, document, symbol.key, &symbol.green),
    );
    MarkupContent {
        kind: MarkupKind::Markdown,
        value: format!("```wat\n{content}\n```"),
    }
}

fn create_data_def_hover(db: &dyn salsa::Database, symbol: &Symbol) -> MarkupContent {
    let mut content = "(data".to_string();
    if let Some(name) = symbol.idx.name {
        let _ = write!(&mut content, " {}", name.ident(db));
    }
    content.push(')');
    MarkupContent {
        kind: MarkupKind::Markdown,
        value: format!("```wat\n{content}\n```"),
    }
}

fn create_elem_def_hover(db: &dyn salsa::Database, symbol: &Symbol) -> MarkupContent {
    let mut content = "(elem".to_string();
    if let Some(name) = symbol.idx.name {
        let _ = write!(&mut content, " {}", name.ident(db));
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
