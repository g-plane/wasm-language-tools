use crate::{
    binder::{SymbolItemKind, SymbolTable, SymbolTablesCtx},
    data_set,
    files::FilesCtx,
    helpers,
    idx::{IdentsCtx, Idx},
    types_analyzer::TypesAnalyzerCtx,
    InternUri, LanguageService,
};
use lsp_types::{
    CompletionItem, CompletionItemKind, CompletionItemLabelDetails, CompletionParams,
    CompletionResponse, Documentation, MarkupContent, MarkupKind, Position,
};
use rowan::{ast::support, Direction, TokenAtOffset};
use smallvec::SmallVec;
use wat_syntax::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};

impl LanguageService {
    /// Handler for `textDocument/completion` request.
    pub fn completion(&self, params: CompletionParams) -> Option<CompletionResponse> {
        let uri = self.uri(params.text_document_position.text_document.uri);
        let token = find_token(self, uri, params.text_document_position.position)?;

        let cmp_ctx = get_cmp_ctx(&token)?;
        let items = get_cmp_list(self, cmp_ctx, &token, uri, &self.symbol_table(uri));
        Some(CompletionResponse::Array(items))
    }
}

fn find_token(
    service: &LanguageService,
    uri: InternUri,
    position: Position,
) -> Option<SyntaxToken> {
    let offset = helpers::lsp_pos_to_rowan_pos(&service.line_index(uri), position)?;
    match SyntaxNode::new_root(service.root(uri)).token_at_offset(offset) {
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
                ctx.push(CmpCtx::KeywordType);
                ctx.push(CmpCtx::KeywordParamResult);
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
        SyntaxKind::MODULE_FIELD_TYPE => {
            if find_leading_l_paren(token).is_some() {
                ctx.push(CmpCtx::KeywordFunc);
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
                        ctx.push(CmpCtx::KeywordType);
                        ctx.push(CmpCtx::KeywordParamResult);
                        ctx.push(CmpCtx::KeywordLocal);
                    }
                }
            } else {
                let instr_name = support::token(&parent, SyntaxKind::INSTR_NAME)?;
                add_cmp_ctx_for_operands(instr_name.text(), &parent, &mut ctx);
            }
        }
        SyntaxKind::OPERAND => {
            let instr = parent
                .ancestors()
                .find(|node| node.kind() == SyntaxKind::PLAIN_INSTR)?;
            let instr_name = support::token(&instr, SyntaxKind::INSTR_NAME)?;
            add_cmp_ctx_for_operands(instr_name.text(), &parent, &mut ctx);
        }
        SyntaxKind::PARAM | SyntaxKind::RESULT | SyntaxKind::LOCAL | SyntaxKind::GLOBAL_TYPE => {
            if !token.text().starts_with('$') {
                ctx.push(CmpCtx::ValType);
            }
        }
        SyntaxKind::TYPE_USE => ctx.push(CmpCtx::FuncType),
        SyntaxKind::FUNC_TYPE => {
            if find_leading_l_paren(token).is_some() {
                ctx.push(CmpCtx::KeywordParamResult);
            }
        }
        SyntaxKind::INDEX => {
            let grand = parent.parent()?;
            match grand.kind() {
                SyntaxKind::MODULE_FIELD_START | SyntaxKind::EXPORT_DESC_FUNC => {
                    ctx.push(CmpCtx::Func);
                }
                SyntaxKind::TYPE_USE => ctx.push(CmpCtx::FuncType),
                SyntaxKind::EXPORT_DESC_GLOBAL => ctx.push(CmpCtx::Global),
                SyntaxKind::EXPORT_DESC_MEMORY => ctx.push(CmpCtx::Memory),
                SyntaxKind::EXPORT_DESC_TABLE => ctx.push(CmpCtx::Table),
                _ => {}
            }
        }
        SyntaxKind::MODULE_FIELD_GLOBAL => {
            if parent
                .children()
                .any(|node| node.kind() == SyntaxKind::GLOBAL_TYPE)
            {
                ctx.push(CmpCtx::Instr);
            } else if find_leading_l_paren(token).is_some() {
                ctx.push(CmpCtx::KeywordMut);
                ctx.push(CmpCtx::KeywordImExport);
            } else {
                ctx.push(CmpCtx::ValType);
            }
        }
        SyntaxKind::MODULE_FIELD_EXPORT | SyntaxKind::MODULE_FIELD_IMPORT => {
            if find_leading_l_paren(token).is_some() {
                ctx.push(CmpCtx::KeywordPortDesc);
            }
        }
        SyntaxKind::MODULE_FIELD_TABLE => {
            if find_leading_l_paren(token).is_some() {
                ctx.push(CmpCtx::KeywordImExport);
                ctx.push(CmpCtx::KeywordElem);
            } else {
                ctx.push(CmpCtx::RefType);
            }
        }
        SyntaxKind::MODULE_FIELD_START | SyntaxKind::EXPORT_DESC_FUNC => ctx.push(CmpCtx::Func),
        SyntaxKind::EXPORT_DESC_GLOBAL => ctx.push(CmpCtx::Global),
        SyntaxKind::MODULE_FIELD_MEMORY => {
            if find_leading_l_paren(token).is_some() {
                ctx.push(CmpCtx::KeywordImExport);
                ctx.push(CmpCtx::KeywordData);
            }
        }
        SyntaxKind::EXPORT_DESC_MEMORY => ctx.push(CmpCtx::Memory),
        SyntaxKind::EXPORT_DESC_TABLE => ctx.push(CmpCtx::Table),
        SyntaxKind::TABLE_TYPE | SyntaxKind::IMPORT_DESC_TABLE_TYPE => ctx.push(CmpCtx::RefType),
        SyntaxKind::IMPORT_DESC_TYPE_USE => {
            if find_leading_l_paren(token).is_some() {
                ctx.push(CmpCtx::KeywordParamResult);
                ctx.push(CmpCtx::KeywordType);
            }
        }
        SyntaxKind::IMPORT_DESC_GLOBAL_TYPE => {
            if find_leading_l_paren(token).is_some() {
                ctx.push(CmpCtx::KeywordMut);
            } else {
                ctx.push(CmpCtx::ValType);
            }
        }
        SyntaxKind::ELEM => {
            if find_leading_l_paren(token).is_some() {
                ctx.push(CmpCtx::Instr);
                ctx.push(CmpCtx::KeywordItem);
            } else {
                ctx.push(CmpCtx::Func);
            }
        }
        SyntaxKind::ELEM_EXPR => ctx.push(CmpCtx::Instr),
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
fn add_cmp_ctx_for_operands(instr_name: &str, node: &SyntaxNode, ctx: &mut SmallVec<[CmpCtx; 4]>) {
    match instr_name.split_once('.') {
        Some(("local", _)) => ctx.push(CmpCtx::Local),
        Some(("global", _)) => ctx.push(CmpCtx::Global),
        Some(("ref", "func")) => ctx.push(CmpCtx::Func),
        Some(("table", snd)) => {
            if snd == "init"
                && node.kind() == SyntaxKind::OPERAND
                && node
                    .prev_sibling()
                    .is_some_and(|prev| prev.kind() == SyntaxKind::OPERAND)
            {
                // elem id
            } else {
                ctx.push(CmpCtx::Table);
            }
        }
        Some((_, snd)) if snd.starts_with("load") || snd.starts_with("store") => {
            ctx.push(CmpCtx::MemArg);
        }
        None => match instr_name {
            "call" | "return_call" => ctx.push(CmpCtx::Func),
            "br" | "br_if" | "br_table" => ctx.push(CmpCtx::Block),
            _ => {}
        },
        _ => {}
    }
}

enum CmpCtx {
    Instr,
    ValType,
    RefType,
    Local,
    Func,
    FuncType,
    Global,
    MemArg,
    Memory,
    Table,
    Block,
    KeywordModule,
    KeywordModuleField,
    KeywordImExport,
    KeywordType,
    KeywordParamResult,
    KeywordLocal,
    KeywordMut,
    KeywordPortDesc,
    KeywordData,
    KeywordFunc,
    KeywordElem,
    KeywordItem,
}

fn get_cmp_list(
    service: &LanguageService,
    ctx: SmallVec<[CmpCtx; 4]>,
    token: &SyntaxToken,
    uri: InternUri,
    symbol_table: &SymbolTable,
) -> Vec<CompletionItem> {
    ctx.into_iter()
        .fold(Vec::with_capacity(2), |mut items, ctx| {
            match ctx {
                CmpCtx::Instr => {
                    items.extend(data_set::INSTR_NAMES.iter().map(|ty| CompletionItem {
                        label: ty.to_string(),
                        kind: Some(CompletionItemKind::OPERATOR),
                        ..Default::default()
                    }));
                }
                CmpCtx::ValType => {
                    items.extend(data_set::VALUE_TYPES.iter().map(|ty| CompletionItem {
                        label: ty.to_string(),
                        kind: Some(CompletionItemKind::CLASS),
                        documentation: data_set::get_value_type_description(ty).map(|desc| {
                            Documentation::MarkupContent(lsp_types::MarkupContent {
                                kind: MarkupKind::Markdown,
                                value: desc.into(),
                            })
                        }),
                        ..Default::default()
                    }));
                }
                CmpCtx::RefType => {
                    items.extend(["funcref", "externref"].iter().map(|ty| CompletionItem {
                        label: ty.to_string(),
                        kind: Some(CompletionItemKind::CLASS),
                        documentation: data_set::get_value_type_description(ty).map(|desc| {
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
                            .filter_map(|symbol| {
                                let (label, insert_text) =
                                    get_idx_cmp_text(service, &symbol.idx, has_dollar)?;
                                Some(CompletionItem {
                                    label,
                                    insert_text,
                                    kind: Some(CompletionItemKind::VARIABLE),
                                    label_details: service.extract_type(symbol.green.clone()).map(
                                        |ty| CompletionItemLabelDetails {
                                            description: Some(ty.to_string()),
                                            ..Default::default()
                                        },
                                    ),
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
                    items.extend(
                        symbol_table
                            .get_declared_fields(module, SymbolItemKind::Func)
                            .filter_map(|symbol| {
                                let (label, insert_text) =
                                    get_idx_cmp_text(service, &symbol.idx, has_dollar)?;
                                Some(CompletionItem {
                                    label,
                                    insert_text,
                                    kind: Some(CompletionItemKind::FUNCTION),
                                    documentation: Some(Documentation::MarkupContent(
                                        MarkupContent {
                                            kind: MarkupKind::Markdown,
                                            value: format!(
                                                "```wat\n{}\n```",
                                                service
                                                    .render_func_header(uri, symbol.clone().into())
                                            ),
                                        },
                                    )),
                                    ..Default::default()
                                })
                            }),
                    );
                }
                CmpCtx::FuncType => {
                    let Some(module) = token
                        .parent_ancestors()
                        .find(|node| node.kind() == SyntaxKind::MODULE)
                    else {
                        return items;
                    };
                    let has_dollar = token.text().starts_with('$');
                    items.extend(
                        symbol_table
                            .get_declared_fields(module, SymbolItemKind::Type)
                            .filter_map(|symbol| {
                                let (label, insert_text) =
                                    get_idx_cmp_text(service, &symbol.idx, has_dollar)?;
                                Some(CompletionItem {
                                    label,
                                    insert_text,
                                    kind: Some(CompletionItemKind::INTERFACE),
                                    ..Default::default()
                                })
                            }),
                    );
                }
                CmpCtx::Global => {
                    let Some(module) = token
                        .parent_ancestors()
                        .find(|node| node.kind() == SyntaxKind::MODULE)
                    else {
                        return items;
                    };
                    let has_dollar = token.text().starts_with('$');
                    items.extend(
                        symbol_table
                            .get_declared_fields(module, SymbolItemKind::GlobalDef)
                            .filter_map(|symbol| {
                                let (label, insert_text) =
                                    get_idx_cmp_text(service, &symbol.idx, has_dollar)?;
                                Some(CompletionItem {
                                    label,
                                    insert_text,
                                    kind: Some(CompletionItemKind::VARIABLE),
                                    label_details: service
                                        .extract_global_type(symbol.green.clone())
                                        .map(|ty| CompletionItemLabelDetails {
                                            description: Some(ty.to_string()),
                                            ..Default::default()
                                        }),
                                    ..Default::default()
                                })
                            }),
                    );
                }
                CmpCtx::MemArg => {
                    items.extend(["offset=", "align="].iter().map(|label| CompletionItem {
                        label: label.to_string(),
                        kind: Some(CompletionItemKind::SNIPPET),
                        ..Default::default()
                    }));
                }
                CmpCtx::Memory => {
                    let Some(module) = token
                        .parent_ancestors()
                        .find(|node| node.kind() == SyntaxKind::MODULE)
                    else {
                        return items;
                    };
                    let has_dollar = token.text().starts_with('$');
                    items.extend(
                        symbol_table
                            .get_declared_fields(module, SymbolItemKind::MemoryDef)
                            .filter_map(|symbol| {
                                let (label, insert_text) =
                                    get_idx_cmp_text(service, &symbol.idx, has_dollar)?;
                                Some(CompletionItem {
                                    label,
                                    insert_text,
                                    kind: Some(CompletionItemKind::VARIABLE),
                                    ..Default::default()
                                })
                            }),
                    );
                }
                CmpCtx::Table => {
                    let Some(module) = token
                        .parent_ancestors()
                        .find(|node| node.kind() == SyntaxKind::MODULE)
                        .map(|module| module.into())
                    else {
                        return items;
                    };
                    let has_dollar = token.text().starts_with('$');
                    items.extend(
                        symbol_table
                            .symbols
                            .iter()
                            .filter(|symbol| {
                                symbol.kind == SymbolItemKind::TableDef && symbol.region == module
                            })
                            .filter_map(|symbol| {
                                let (label, insert_text) =
                                    get_idx_cmp_text(service, &symbol.idx, has_dollar)?;
                                Some(CompletionItem {
                                    label,
                                    insert_text,
                                    kind: Some(CompletionItemKind::VARIABLE),
                                    ..Default::default()
                                })
                            }),
                    );
                }
                CmpCtx::Block => {
                    let has_dollar = token.text().starts_with('$');
                    items.extend(
                        symbol_table
                            .symbols
                            .iter()
                            .filter(|symbol| {
                                symbol.kind == SymbolItemKind::BlockDef
                                    && symbol
                                        .key
                                        .ptr
                                        .text_range()
                                        .contains_range(token.text_range())
                            })
                            .rev()
                            .enumerate()
                            .filter_map(|(num, symbol)| {
                                let idx = Idx {
                                    num: Some(num as u32),
                                    name: symbol.idx.name,
                                };
                                let (label, insert_text) =
                                    get_idx_cmp_text(service, &idx, has_dollar)?;
                                Some(CompletionItem {
                                    label,
                                    insert_text,
                                    kind: Some(CompletionItemKind::VARIABLE),
                                    ..Default::default()
                                })
                            }),
                    );
                }
                CmpCtx::KeywordModule => items.push(CompletionItem {
                    label: "module".to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    ..Default::default()
                }),
                CmpCtx::KeywordModuleField => {
                    items.extend(data_set::MODULE_FIELDS.iter().map(|ty| CompletionItem {
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
                CmpCtx::KeywordType => items.push(CompletionItem {
                    label: "type".to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    ..Default::default()
                }),
                CmpCtx::KeywordParamResult => {
                    items.extend(["param", "result"].iter().map(|keyword| CompletionItem {
                        label: keyword.to_string(),
                        kind: Some(CompletionItemKind::KEYWORD),
                        ..Default::default()
                    }));
                }
                CmpCtx::KeywordLocal => items.push(CompletionItem {
                    label: "local".to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    ..Default::default()
                }),
                CmpCtx::KeywordMut => items.push(CompletionItem {
                    label: "mut".to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    ..Default::default()
                }),
                CmpCtx::KeywordPortDesc => {
                    items.extend(data_set::PORT_DESC.iter().map(|desc| CompletionItem {
                        label: desc.to_string(),
                        kind: Some(CompletionItemKind::KEYWORD),
                        ..Default::default()
                    }));
                }
                CmpCtx::KeywordData => items.push(CompletionItem {
                    label: "data".to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    ..Default::default()
                }),
                CmpCtx::KeywordFunc => items.push(CompletionItem {
                    label: "func".to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    ..Default::default()
                }),
                CmpCtx::KeywordElem => items.push(CompletionItem {
                    label: "elem".to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    ..Default::default()
                }),
                CmpCtx::KeywordItem => items.push(CompletionItem {
                    label: "item".to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    ..Default::default()
                }),
            }
            items
        })
}

fn get_idx_cmp_text(
    service: &LanguageService,
    idx: &Idx,
    has_dollar: bool,
) -> Option<(String, Option<String>)> {
    if has_dollar {
        let name = service.lookup_ident(idx.name?);
        Some((name.to_owned(), Some(name.strip_prefix('$')?.to_string())))
    } else {
        Some((
            idx.name
                .map(|name| service.lookup_ident(name))
                .or_else(|| idx.num.map(|num| num.to_string()))
                .unwrap_or_default(),
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
