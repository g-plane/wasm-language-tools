use crate::{
    binder::{SymbolKey, SymbolTablesCtx},
    helpers,
    syntax_tree::SyntaxTreeCtx,
    uri::{InternUri, UrisCtx},
    LanguageService,
};
use line_index::LineCol;
use lsp_types::{
    SemanticToken, SemanticTokens, SemanticTokensParams, SemanticTokensRangeParams,
    SemanticTokensRangeResult, SemanticTokensResult,
};
use rowan::ast::support;
use std::mem;
use wat_syntax::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};

impl LanguageService {
    /// Handler for `textDocument/semanticTokens/full` request.
    pub fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Option<SemanticTokensResult> {
        let uri = self.uri(params.text_document.uri);
        let mut delta_line = 0;
        let mut prev_start = 0;
        let tokens = self.build_tokens(
            uri,
            SyntaxNode::new_root(self.root(uri))
                .descendants_with_tokens()
                .filter_map(SyntaxElement::into_token),
            &mut delta_line,
            &mut prev_start,
        );
        Some(SemanticTokensResult::Tokens(SemanticTokens {
            result_id: None,
            data: tokens,
        }))
    }

    /// Handler for `textDocument/semanticTokens/range` request.
    pub fn semantic_tokens_range(
        &self,
        params: SemanticTokensRangeParams,
    ) -> Option<SemanticTokensRangeResult> {
        let uri = self.uri(params.text_document.uri);
        let line_index = self.line_index(uri);
        let start = helpers::lsp_pos_to_rowan_pos(&line_index, params.range.start)?;
        let end = helpers::lsp_pos_to_rowan_pos(&line_index, params.range.end)?;

        let mut delta_line = 0;
        let mut prev_start = 0;
        let mut tokens = SyntaxNode::new_root(self.root(uri))
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
        let tokens = self.build_tokens(uri, tokens, &mut delta_line, &mut prev_start);
        Some(SemanticTokensRangeResult::Tokens(SemanticTokens {
            result_id: None,
            data: tokens,
        }))
    }

    fn build_tokens(
        &self,
        uri: InternUri,
        tokens: impl Iterator<Item = SyntaxToken>,
        delta_line: &mut u32,
        prev_start: &mut u32,
    ) -> Vec<SemanticToken> {
        let line_index = self.line_index(uri);
        tokens
            .filter_map(|token| {
                if token.kind() == SyntaxKind::WHITESPACE {
                    let lines = token.text().chars().filter(|c| *c == '\n').count() as u32;
                    if lines > 0 {
                        *delta_line += lines;
                        *prev_start = 0;
                    }
                    return None;
                }
                let token_type = self.token_type(uri, &token)?;
                let block_comment_lines = if token.kind() == SyntaxKind::BLOCK_COMMENT {
                    Some(token.text().chars().filter(|c| *c == '\n').count() as u32)
                } else {
                    None
                };
                let range = token.text_range();
                let col = line_index.line_col(range.start()).col;
                Some(SemanticToken {
                    delta_line: mem::replace(delta_line, block_comment_lines.unwrap_or_default()),
                    delta_start: col
                        - mem::replace(
                            prev_start,
                            if block_comment_lines.is_some_and(|lines| lines > 0) {
                                0
                            } else {
                                col
                            },
                        ),
                    length: range.len().into(),
                    token_type,
                    token_modifiers_bitset: 0,
                })
            })
            .collect()
    }

    fn token_type(&self, uri: InternUri, token: &SyntaxToken) -> Option<u32> {
        let symbol_table = self.symbol_table(uri);
        let token_kinds = &self.semantic_token_kinds;
        match token.kind() {
            SyntaxKind::NUM_TYPE | SyntaxKind::VEC_TYPE | SyntaxKind::REF_TYPE => {
                token_kinds.get_index_of(&SemanticTokenKind::Type)
            }
            SyntaxKind::KEYWORD => token_kinds.get_index_of(&SemanticTokenKind::Keyword),
            SyntaxKind::INT | SyntaxKind::UNSIGNED_INT => {
                let parent = token.parent();
                let grand = parent.as_ref().and_then(|parent| parent.parent());
                if grand.as_ref().is_some_and(|grand| {
                    helpers::ast::is_call(grand)
                        || matches!(
                            grand.kind(),
                            SyntaxKind::MODULE_FIELD_START | SyntaxKind::EXPORT_DESC_FUNC
                        )
                }) {
                    token_kinds.get_index_of(&SemanticTokenKind::Func)
                } else if let Some(immediate) = grand
                    .filter(|grand| {
                        support::token(grand, SyntaxKind::INSTR_NAME)
                            .is_some_and(|name| name.text().starts_with("local."))
                    })
                    .and(parent)
                {
                    if symbol_table
                        .find_param_def(SymbolKey::new(&immediate))
                        .is_some()
                    {
                        token_kinds.get_index_of(&SemanticTokenKind::Param)
                    } else {
                        token_kinds.get_index_of(&SemanticTokenKind::Var)
                    }
                } else {
                    token_kinds.get_index_of(&SemanticTokenKind::Number)
                }
            }
            SyntaxKind::FLOAT => token_kinds.get_index_of(&SemanticTokenKind::Number),
            SyntaxKind::IDENT => {
                let parent = token.parent();
                if parent
                    .as_ref()
                    .and_then(|parent| parent.parent())
                    .is_some_and(|grand| {
                        helpers::ast::is_call(&grand)
                            || matches!(
                                grand.kind(),
                                SyntaxKind::MODULE_FIELD_START | SyntaxKind::EXPORT_DESC_FUNC
                            )
                    })
                    || parent
                        .as_ref()
                        .is_some_and(|node| node.kind() == SyntaxKind::MODULE_FIELD_FUNC)
                {
                    token_kinds.get_index_of(&SemanticTokenKind::Func)
                } else if parent
                    .as_ref()
                    .is_some_and(|node| node.kind() == SyntaxKind::PARAM)
                    || parent
                        .as_ref()
                        .and_then(|parent| symbol_table.find_param_def(SymbolKey::new(parent)))
                        .is_some()
                {
                    token_kinds.get_index_of(&SemanticTokenKind::Param)
                } else {
                    token_kinds.get_index_of(&SemanticTokenKind::Var)
                }
            }
            SyntaxKind::STRING => token_kinds.get_index_of(&SemanticTokenKind::String),
            SyntaxKind::LINE_COMMENT | SyntaxKind::BLOCK_COMMENT => {
                token_kinds.get_index_of(&SemanticTokenKind::Comment)
            }
            SyntaxKind::INSTR_NAME => token_kinds.get_index_of(&SemanticTokenKind::Op),
            _ => None,
        }
        .map(|v| v as u32)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum SemanticTokenKind {
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
