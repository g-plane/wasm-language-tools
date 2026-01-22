use crate::{
    LanguageService,
    binder::{SymbolKey, SymbolKind, SymbolTable},
    document::Document,
    helpers, mutability,
};
use indexmap::IndexSet;
use line_index::LineCol;
use lspt::{SemanticTokens, SemanticTokensParams, SemanticTokensRangeParams};
use rowan::ast::support;
use rustc_hash::FxBuildHasher;
use std::mem;
use wat_syntax::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};

impl LanguageService {
    /// Handler for `textDocument/semanticTokens/full` request.
    pub fn semantic_tokens_full(&self, params: SemanticTokensParams) -> Option<SemanticTokens> {
        let token_types = &self.semantic_token_types;
        let document = self.get_document(params.text_document.uri)?;
        self.with_db(|db| {
            let mut delta_line = 0;
            let mut prev_start = 0;
            let tokens = build_tokens(
                db,
                token_types,
                document,
                document
                    .root_tree(db)
                    .descendants_with_tokens()
                    .filter_map(SyntaxElement::into_token),
                &mut delta_line,
                &mut prev_start,
            );
            SemanticTokens {
                result_id: None,
                data: tokens,
            }
        })
    }

    /// Handler for `textDocument/semanticTokens/range` request.
    pub fn semantic_tokens_range(&self, params: SemanticTokensRangeParams) -> Option<SemanticTokens> {
        let token_types = &self.semantic_token_types;
        let document = self.get_document(params.text_document.uri)?;
        self.with_db(|db| {
            let line_index = document.line_index(db);
            let start = helpers::lsp_pos_to_rowan_pos(line_index, params.range.start)?;
            let end = helpers::lsp_pos_to_rowan_pos(line_index, params.range.end)?;

            let mut delta_line = 0;
            let mut prev_start = 0;
            let mut tokens = document
                .root_tree(db)
                .descendants_with_tokens()
                .filter_map(SyntaxElement::into_token)
                .skip_while(|token| token.text_range().end() <= start)
                .take_while(|token| token.text_range().start() < end)
                .peekable();
            if let Some(token) = tokens.peek() {
                LineCol {
                    line: delta_line,
                    col: prev_start,
                } = line_index.line_col(token.text_range().start());
            }
            let tokens = build_tokens(db, token_types, document, tokens, &mut delta_line, &mut prev_start);
            Some(SemanticTokens {
                result_id: None,
                data: tokens,
            })
        })
        .flatten()
    }
}

fn build_tokens(
    db: &dyn salsa::Database,
    token_types: &SemanticTokenTypes,
    document: Document,
    tokens: impl Iterator<Item = SyntaxToken>,
    delta_line: &mut u32,
    prev_start: &mut u32,
) -> Vec<u32> {
    let line_index = document.line_index(db);
    let symbol_table = SymbolTable::of(db, document);
    tokens
        .filter_map(|token| {
            match token.kind() {
                SyntaxKind::WHITESPACE => {
                    let lines = token.text().chars().filter(|c| *c == '\n').count() as u32;
                    if lines > 0 {
                        *delta_line += lines;
                        *prev_start = 0;
                    }
                    None
                }
                SyntaxKind::ERROR => {
                    *prev_start = line_index.line_col(token.text_range().start()).col;
                    None
                }
                _ => {
                    let token_type = compute_token_type(token_types, &token, symbol_table)?;
                    let block_comment_lines = if token.kind() == SyntaxKind::BLOCK_COMMENT {
                        Some(token.text().chars().filter(|c| *c == '\n').count() as u32)
                    } else {
                        None
                    };
                    let range = token.text_range();
                    let col = line_index.line_col(range.start()).col;
                    Some([
                        // delta line
                        mem::replace(delta_line, block_comment_lines.unwrap_or_default()),
                        // delta start
                        col - mem::replace(
                            prev_start,
                            if block_comment_lines.is_some_and(|lines| lines > 0) {
                                0
                            } else {
                                col
                            },
                        ),
                        // length
                        range.len().into(),
                        // token type
                        token_type,
                        // token modifiers bitset
                        compute_token_modifier(db, document, symbol_table, &token),
                    ])
                }
            }
        })
        .flatten()
        .collect()
}

fn compute_token_type(
    token_types: &SemanticTokenTypes,
    token: &SyntaxToken,
    symbol_table: &SymbolTable,
) -> Option<u32> {
    match token.kind() {
        SyntaxKind::TYPE_KEYWORD => token_types.get_index_of(&SemanticTokenType::Type),
        SyntaxKind::KEYWORD => token_types.get_index_of(&SemanticTokenType::Keyword),
        SyntaxKind::INT | SyntaxKind::UNSIGNED_INT => {
            let parent = token.parent();
            let grand = parent.as_ref().and_then(|parent| parent.parent());
            if grand.as_ref().is_some_and(|grand| {
                helpers::ast::is_call(grand)
                    || matches!(
                        grand.kind(),
                        SyntaxKind::MODULE_FIELD_START | SyntaxKind::EXTERN_IDX_FUNC | SyntaxKind::ELEM_LIST
                    )
            }) {
                token_types.get_index_of(&SemanticTokenType::Func)
            } else if let Some(immediate) = grand
                .filter(|grand| {
                    support::token(grand, SyntaxKind::INSTR_NAME).is_some_and(|name| name.text().starts_with("local."))
                })
                .and(parent)
            {
                if is_ref_of_param(&immediate, symbol_table) {
                    token_types.get_index_of(&SemanticTokenType::Param)
                } else {
                    token_types.get_index_of(&SemanticTokenType::Var)
                }
            } else {
                token_types.get_index_of(&SemanticTokenType::Number)
            }
        }
        SyntaxKind::FLOAT => token_types.get_index_of(&SemanticTokenType::Number),
        SyntaxKind::IDENT => {
            let parent = token.parent();
            if parent.as_ref().and_then(|parent| parent.parent()).is_some_and(|grand| {
                helpers::ast::is_call(&grand)
                    || matches!(
                        grand.kind(),
                        SyntaxKind::MODULE_FIELD_START | SyntaxKind::EXTERN_IDX_FUNC | SyntaxKind::ELEM_LIST
                    )
            }) || parent
                .as_ref()
                .is_some_and(|node| node.kind() == SyntaxKind::MODULE_FIELD_FUNC)
            {
                token_types.get_index_of(&SemanticTokenType::Func)
            } else if parent.as_ref().is_some_and(|node| node.kind() == SyntaxKind::PARAM)
                || parent.as_ref().is_some_and(|node| is_ref_of_param(node, symbol_table))
            {
                token_types.get_index_of(&SemanticTokenType::Param)
            } else {
                token_types.get_index_of(&SemanticTokenType::Var)
            }
        }
        SyntaxKind::STRING => token_types.get_index_of(&SemanticTokenType::String),
        SyntaxKind::LINE_COMMENT | SyntaxKind::BLOCK_COMMENT => token_types.get_index_of(&SemanticTokenType::Comment),
        SyntaxKind::INSTR_NAME => token_types.get_index_of(&SemanticTokenType::Op),
        SyntaxKind::ANNOT_ELEM => {
            if token.text().starts_with('"') && token.text().ends_with('"') {
                token_types.get_index_of(&SemanticTokenType::String)
            } else {
                None
            }
        }
        _ => None,
    }
    .map(|v| v as u32)
}

fn compute_token_modifier(
    db: &dyn salsa::Database,
    document: Document,
    symbol_table: &SymbolTable,
    token: &SyntaxToken,
) -> u32 {
    if matches!(
        token.kind(),
        SyntaxKind::IDENT | SyntaxKind::INT | SyntaxKind::UNSIGNED_INT
    ) && let Some(symbol) = token
        .parent()
        .and_then(|node| symbol_table.symbols.get(&SymbolKey::new(&node)))
    {
        match symbol.kind {
            SymbolKind::GlobalDef | SymbolKind::Type | SymbolKind::FieldDef => {
                mutability::get_mutabilities(db, document)
                    .get(&symbol.key)
                    .and_then(|mutability| mutability.mut_keyword)
                    .map_or(0, |_| 1)
            }
            SymbolKind::GlobalRef | SymbolKind::TypeUse | SymbolKind::FieldRef => symbol_table
                .resolved
                .get(&symbol.key)
                .and_then(|def_key| mutability::get_mutabilities(db, document).get(def_key))
                .and_then(|mutability| mutability.mut_keyword)
                .map_or(0, |_| 1),
            _ => 0,
        }
    } else {
        0
    }
}

pub(crate) type SemanticTokenTypes = IndexSet<SemanticTokenType, FxBuildHasher>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum SemanticTokenType {
    Type,
    Param,
    Var,
    Func,
    Keyword,
    Comment,
    String,
    Number,
    Op,
}

fn is_ref_of_param(node: &SyntaxNode, symbol_table: &SymbolTable) -> bool {
    symbol_table
        .find_def(SymbolKey::new(node))
        .is_some_and(|symbol| symbol.kind == SymbolKind::Param)
}
