use crate::{
    binder::{SymbolTable, SymbolTablesCtx},
    dataset,
    files::FilesCtx,
    InternUri, LanguageService, LanguageServiceCtx,
};
use line_index::LineCol;
use lsp_types::{CompletionItem, CompletionParams, CompletionResponse, Position};
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
            let next_node = token
                .siblings_with_tokens(Direction::Next)
                .skip(1)
                .find(|element| matches!(element, SyntaxElement::Node(..)))
                .map(|element| element.kind());
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
            } else {
                let instr_name = support::token(&parent, SyntaxKind::INSTR_NAME)?;
                let instr_name = instr_name.text();
                if instr_name.starts_with("local.") {
                    ctx.push(CmpCtx::Local);
                }
            }
        }
        SyntaxKind::OPERAND => {
            let instr = parent
                .ancestors()
                .find(|node| node.kind() == SyntaxKind::PLAIN_INSTR)?;
            let instr_name = support::token(&instr, SyntaxKind::INSTR_NAME)?;
            let instr_name = instr_name.text();
            if instr_name.starts_with("local.") {
                ctx.push(CmpCtx::Local);
            }
        }
        SyntaxKind::PARAM | SyntaxKind::RESULT | SyntaxKind::LOCAL | SyntaxKind::GLOBAL_TYPE => {
            if !token.text().starts_with('$') {
                ctx.push(CmpCtx::ValType);
            }
        }
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

enum CmpCtx {
    Instr,
    ValType,
    Local,
    KeywordModule,
    KeywordModuleField,
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
                        kind: Some(lsp_types::CompletionItemKind::OPERATOR),
                        ..Default::default()
                    }))
                }
                CmpCtx::ValType => {
                    items.extend(dataset::VALUE_TYPES.iter().map(|ty| CompletionItem {
                        label: ty.to_string(),
                        kind: Some(lsp_types::CompletionItemKind::CLASS),
                        ..Default::default()
                    }))
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
                                if has_dollar {
                                    let name = idx.name.as_ref()?;
                                    Some(CompletionItem {
                                        label: name.to_owned(),
                                        insert_text: Some(name.strip_prefix('$')?.to_string()),
                                        kind: Some(lsp_types::CompletionItemKind::VARIABLE),
                                        ..Default::default()
                                    })
                                } else {
                                    Some(CompletionItem {
                                        label: idx
                                            .name
                                            .as_ref()
                                            .map(|name| name.to_string())
                                            .unwrap_or_else(|| idx.num.to_string()),
                                        kind: Some(lsp_types::CompletionItemKind::VARIABLE),
                                        ..Default::default()
                                    })
                                }
                            }),
                    );
                }
                CmpCtx::KeywordModule => items.push(CompletionItem {
                    label: "module".to_string(),
                    kind: Some(lsp_types::CompletionItemKind::KEYWORD),
                    ..Default::default()
                }),
                CmpCtx::KeywordModuleField => {
                    items.extend(dataset::MODULE_FIELDS.iter().map(|ty| CompletionItem {
                        label: ty.to_string(),
                        kind: Some(lsp_types::CompletionItemKind::KEYWORD),
                        ..Default::default()
                    }))
                }
            }
            items
        })
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
