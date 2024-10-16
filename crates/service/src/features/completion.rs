use crate::{
    binder::{DefIdx, SymbolTable, SymbolTablesCtx},
    dataset,
    files::FilesCtx,
    InternUri, LanguageService, LanguageServiceCtx,
};
use line_index::LineCol;
use lsp_types::{
    CompletionItem, CompletionItemKind, CompletionParams, CompletionResponse, Documentation,
    MarkupKind, Position,
};
use rowan::{ast::support, Direction, TokenAtOffset};
use smallvec::SmallVec;
use wat_syntax::{SyntaxElement, SyntaxKind, SyntaxToken};

impl LanguageService {
    pub fn completion(&self, params: CompletionParams) -> Option<CompletionResponse> {
        let uri = self
            .ctx
            .uri(params.text_document_position.text_document.uri);
        let token = find_token(&self.ctx, uri, params.text_document_position.position)?;

        let cmp_ctx = get_cmp_ctx(&token)?;

        let symbol_table = self.ctx.symbol_table(uri);
        let items = get_cmp_list(cmp_ctx, &token, &symbol_table);
        Some(CompletionResponse::Array(items))
    }
}

fn find_token(
    service: &LanguageServiceCtx,
    uri: InternUri,
    position: Position,
) -> Option<SyntaxToken> {
    let offset = service
        .line_index(uri)
        .offset(LineCol {
            line: position.line,
            col: position.character,
        })
        .map(|text_size| rowan::TextSize::new(text_size.into()))?;
    match service.root(uri).token_at_offset(offset) {
        TokenAtOffset::None => None,
        TokenAtOffset::Single(token) => Some(token),
        TokenAtOffset::Between(left, _) => Some(left),
    }
}

fn get_cmp_ctx(token: &SyntaxToken) -> Option<SmallVec<[CmpCtx; 4]>> {
    let mut ctx = SmallVec::with_capacity(2);
    let parent = token.parent()?;
    match parent.kind() {
        SyntaxKind::MODULE_FIELD_FUNC => {
            let prev_node = token
                .siblings_with_tokens(Direction::Prev)
                .skip(1)
                .find(|element| matches!(element, SyntaxElement::Node(..)))
                .map(|element| element.kind());
            let next_node = token
                .siblings_with_tokens(Direction::Next)
                .skip(1)
                .find(|element| matches!(element, SyntaxElement::Node(..)))
                .map(|element| element.kind());
            if !matches!(
                prev_node,
                Some(
                    SyntaxKind::PLAIN_INSTR
                        | SyntaxKind::BLOCK_BLOCK
                        | SyntaxKind::BLOCK_IF
                        | SyntaxKind::BLOCK_LOOP
                )
            ) && find_leading_l_paren(token).is_some()
            {
                ctx.reserve(3);
                ctx.push(CmpCtx::KeywordImExport);
                ctx.push(CmpCtx::KeywordTypeUse);
                ctx.push(CmpCtx::KeywordLocal);
            }
            if matches!(
                next_node,
                Some(
                    SyntaxKind::PLAIN_INSTR
                        | SyntaxKind::BLOCK_BLOCK
                        | SyntaxKind::BLOCK_IF
                        | SyntaxKind::BLOCK_LOOP
                ) | None
            ) {
                ctx.push(CmpCtx::Instr);
            }
        }
        SyntaxKind::PLAIN_INSTR => {
            if token.kind() == SyntaxKind::INSTR_NAME {
                ctx.push(CmpCtx::Instr);
                if parent
                    .parent()
                    .is_some_and(|grand| grand.kind() == SyntaxKind::MODULE_FIELD_FUNC)
                {
                    // Given the code below:
                    // (func (export "foo") (par))
                    //                          ^ cursor
                    // User is probably going to type "param",
                    // but parser treat it as a plain instruction,
                    // so we catch this case here, though these keywords aren't instruction names.
                    let prev_node = parent
                        .siblings_with_tokens(Direction::Prev)
                        .skip(1)
                        .find(|element| matches!(element, SyntaxElement::Node(..)))
                        .map(|element| element.kind());
                    if !matches!(
                        prev_node,
                        Some(
                            SyntaxKind::PLAIN_INSTR
                                | SyntaxKind::BLOCK_BLOCK
                                | SyntaxKind::BLOCK_IF
                                | SyntaxKind::BLOCK_LOOP
                        )
                    ) && find_leading_l_paren(token).is_some()
                    {
                        ctx.reserve(3);
                        ctx.push(CmpCtx::KeywordImExport);
                        ctx.push(CmpCtx::KeywordTypeUse);
                        ctx.push(CmpCtx::KeywordLocal);
                    }
                }
            } else {
                let instr_name = support::token(&parent, SyntaxKind::INSTR_NAME)?;
                add_cmp_ctx_for_operands(instr_name.text(), &mut ctx);
            }
        }
        SyntaxKind::OPERAND => {
            let instr = parent
                .ancestors()
                .find(|node| node.kind() == SyntaxKind::PLAIN_INSTR)?;
            let instr_name = support::token(&instr, SyntaxKind::INSTR_NAME)?;
            add_cmp_ctx_for_operands(instr_name.text(), &mut ctx);
        }
        SyntaxKind::PARAM | SyntaxKind::RESULT | SyntaxKind::LOCAL | SyntaxKind::GLOBAL_TYPE => {
            if !token.text().starts_with('$') {
                ctx.push(CmpCtx::ValType);
            }
        }
        SyntaxKind::TYPE_USE => ctx.push(CmpCtx::FuncType),
        SyntaxKind::INDEX => {
            let grand = parent.parent()?;
            match grand.kind() {
                SyntaxKind::MODULE_FIELD_START => ctx.push(CmpCtx::Func),
                SyntaxKind::TYPE_USE => ctx.push(CmpCtx::FuncType),
                _ => {}
            }
        }
        SyntaxKind::MODULE_FIELD_START => ctx.push(CmpCtx::Func),
        SyntaxKind::MODULE => {
            if find_leading_l_paren(token).is_some() {
                ctx.push(CmpCtx::KeywordModuleField);
            }
        }
        SyntaxKind::ROOT => {
            if find_leading_l_paren(token).is_some() {
                ctx.push(CmpCtx::KeywordModule);
            }
        }
        _ => {}
    }
    if ctx.is_empty() {
        None
    } else {
        Some(ctx)
    }
}
fn add_cmp_ctx_for_operands(instr_name: &str, ctx: &mut SmallVec<[CmpCtx; 4]>) {
    match instr_name.split_once('.') {
        Some(("local", _)) => ctx.push(CmpCtx::Local),
        Some(("global", _)) => ctx.push(CmpCtx::Global),
        Some(("ref", "func")) => ctx.push(CmpCtx::Func),
        Some((_, snd)) if snd.starts_with("load") || snd.starts_with("store") => {
            ctx.push(CmpCtx::MemArg);
        }
        None => {
            if instr_name == "call" {
                ctx.push(CmpCtx::Func);
            }
        }
        _ => {}
    }
}

enum CmpCtx {
    Instr,
    ValType,
    Local,
    Func,
    FuncType,
    Global,
    MemArg,
    KeywordModule,
    KeywordModuleField,
    KeywordImExport,
    KeywordTypeUse,
    KeywordLocal,
}

fn get_cmp_list(
    ctx: SmallVec<[CmpCtx; 4]>,
    token: &SyntaxToken,
    symbol_table: &SymbolTable,
) -> Vec<CompletionItem> {
    ctx.into_iter()
        .fold(Vec::with_capacity(2), |mut items, ctx| {
            match ctx {
                CmpCtx::Instr => {
                    items.extend(dataset::INSTR_NAMES.iter().map(|ty| CompletionItem {
                        label: ty.to_string(),
                        kind: Some(CompletionItemKind::OPERATOR),
                        ..Default::default()
                    }));
                }
                CmpCtx::ValType => {
                    items.extend(dataset::VALUE_TYPES.iter().map(|ty| CompletionItem {
                        label: ty.to_string(),
                        kind: Some(CompletionItemKind::CLASS),
                        documentation: dataset::get_value_type_description(ty).map(|desc| {
                            Documentation::MarkupContent(lsp_types::MarkupContent {
                                kind: MarkupKind::Markdown,
                                value: desc.into(),
                            })
                        }),
                        ..Default::default()
                    }));
                }
                CmpCtx::Local => {
                    let Some(func) = token
                        .parent_ancestors()
                        .find(|node| node.kind() == SyntaxKind::MODULE_FIELD_FUNC)
                    else {
                        return items;
                    };
                    let has_dollar = token.text().starts_with('$');
                    items.extend(
                        symbol_table
                            .get_declared_params_and_locals(func)
                            .filter_map(|(_, idx)| {
                                let (label, insert_text) = get_def_idx_cmp_text(idx, has_dollar)?;
                                Some(CompletionItem {
                                    label,
                                    insert_text,
                                    kind: Some(CompletionItemKind::VARIABLE),
                                    ..Default::default()
                                })
                            }),
                    );
                }
                CmpCtx::Func => {
                    let Some(module) = token
                        .parent_ancestors()
                        .find(|node| node.kind() == SyntaxKind::MODULE)
                    else {
                        return items;
                    };
                    let has_dollar = token.text().starts_with('$');
                    items.extend(symbol_table.get_declared_functions(module).filter_map(
                        |(_, idx)| {
                            let (label, insert_text) = get_def_idx_cmp_text(idx, has_dollar)?;
                            Some(CompletionItem {
                                label,
                                insert_text,
                                kind: Some(CompletionItemKind::FUNCTION),
                                ..Default::default()
                            })
                        },
                    ));
                }
                CmpCtx::FuncType => {
                    let Some(module) = token
                        .parent_ancestors()
                        .find(|node| node.kind() == SyntaxKind::MODULE)
                    else {
                        return items;
                    };
                    let has_dollar = token.text().starts_with('$');
                    items.extend(symbol_table.get_declared_func_types(module).filter_map(
                        |(_, idx)| {
                            let (label, insert_text) = get_def_idx_cmp_text(idx, has_dollar)?;
                            Some(CompletionItem {
                                label,
                                insert_text,
                                kind: Some(CompletionItemKind::INTERFACE),
                                ..Default::default()
                            })
                        },
                    ));
                }
                CmpCtx::Global => {
                    let Some(module) = token
                        .parent_ancestors()
                        .find(|node| node.kind() == SyntaxKind::MODULE)
                    else {
                        return items;
                    };
                    let has_dollar = token.text().starts_with('$');
                    items.extend(symbol_table.get_declared_globals(module).filter_map(
                        |(_, idx)| {
                            let (label, insert_text) = get_def_idx_cmp_text(idx, has_dollar)?;
                            Some(CompletionItem {
                                label,
                                insert_text,
                                kind: Some(CompletionItemKind::VARIABLE),
                                ..Default::default()
                            })
                        },
                    ));
                }
                CmpCtx::MemArg => {
                    items.extend(["offset=", "align="].iter().map(|label| CompletionItem {
                        label: label.to_string(),
                        kind: Some(CompletionItemKind::SNIPPET),
                        ..Default::default()
                    }));
                }
                CmpCtx::KeywordModule => items.push(CompletionItem {
                    label: "module".to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    ..Default::default()
                }),
                CmpCtx::KeywordModuleField => {
                    items.extend(dataset::MODULE_FIELDS.iter().map(|ty| CompletionItem {
                        label: ty.to_string(),
                        kind: Some(CompletionItemKind::KEYWORD),
                        ..Default::default()
                    }));
                }
                CmpCtx::KeywordImExport => {
                    items.extend(["import", "export"].iter().map(|keyword| CompletionItem {
                        label: keyword.to_string(),
                        kind: Some(CompletionItemKind::KEYWORD),
                        ..Default::default()
                    }));
                }
                CmpCtx::KeywordTypeUse => {
                    items.extend(["type", "param", "result"].iter().map(|keyword| {
                        CompletionItem {
                            label: keyword.to_string(),
                            kind: Some(CompletionItemKind::KEYWORD),
                            ..Default::default()
                        }
                    }));
                }
                CmpCtx::KeywordLocal => items.push(CompletionItem {
                    label: "local".to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    ..Default::default()
                }),
            }
            items
        })
}

fn get_def_idx_cmp_text(idx: &DefIdx, has_dollar: bool) -> Option<(String, Option<String>)> {
    if has_dollar {
        let name = idx.name.as_ref()?;
        Some((name.to_owned(), Some(name.strip_prefix('$')?.to_string())))
    } else {
        Some((
            idx.name
                .as_ref()
                .map(|name| name.to_string())
                .unwrap_or_else(|| idx.num.to_string()),
            None,
        ))
    }
}

fn find_leading_l_paren(token: &SyntaxToken) -> Option<SyntaxToken> {
    if is_l_paren(token) {
        Some(token.clone())
    } else {
        token
            .siblings_with_tokens(Direction::Prev)
            .skip(1)
            .skip_while(|element| {
                matches!(
                    element.kind(),
                    SyntaxKind::WHITESPACE | SyntaxKind::LINE_COMMENT | SyntaxKind::BLOCK_COMMENT
                )
            })
            .find_map(SyntaxElement::into_token)
            .filter(is_l_paren)
    }
}
fn is_l_paren(token: &SyntaxToken) -> bool {
    let kind = token.kind();
    kind == SyntaxKind::L_PAREN || kind == SyntaxKind::ERROR && token.text() == "("
}
