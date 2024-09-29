use crate::{files::FilesCtx, LanguageService};
use lsp_types::{SemanticToken, SemanticTokens, SemanticTokensParams, SemanticTokensResult};
use std::mem;
use wat_syntax::{SyntaxElement, SyntaxKind, SyntaxToken};

impl LanguageService {
    pub fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Option<SemanticTokensResult> {
        let uri = self.ctx.uri(params.text_document.uri);
        let line_index = self.ctx.line_index(uri);
        let mut delta_line = 0;
        let mut prev_start = 0;
        let tokens = self
            .ctx
            .root(uri)
            .descendants_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .filter_map(|token| {
                if token.kind() == SyntaxKind::WHITESPACE {
                    let lines = token.text().chars().filter(|c| *c == '\n').count() as u32;
                    if lines > 0 {
                        delta_line += lines;
                        prev_start = 0;
                    }
                    return None;
                }
                let token_type = self.token_type(&token)?;
                let block_comment_lines = if token.kind() == SyntaxKind::BLOCK_COMMENT {
                    Some(token.text().chars().filter(|c| *c == '\n').count() as u32)
                } else {
                    None
                };
                let range = token.text_range();
                let col = line_index.line_col(range.start()).col;
                Some(SemanticToken {
                    delta_line: mem::replace(
                        &mut delta_line,
                        if let Some(lines) = block_comment_lines {
                            lines
                        } else {
                            0
                        },
                    ),
                    delta_start: col
                        - mem::replace(
                            &mut prev_start,
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
            .collect();
        Some(SemanticTokensResult::Tokens(SemanticTokens {
            result_id: None,
            data: tokens,
        }))
    }

    fn token_type(&self, token: &SyntaxToken) -> Option<u32> {
        match token.kind() {
            SyntaxKind::NUM_TYPE | SyntaxKind::VEC_TYPE | SyntaxKind::REF_TYPE => self
                .semantic_token_kinds
                .get_index_of(&SemanticTokenKind::Type),
            SyntaxKind::KEYWORD | SyntaxKind::INSTR_NAME => self
                .semantic_token_kinds
                .get_index_of(&SemanticTokenKind::Keyword),
            SyntaxKind::INT | SyntaxKind::UNSIGNED_INT => {
                // TODO: detect `local.get` and `local.set`
                if token
                    .parent()
                    .and_then(|parent| parent.parent())
                    .is_some_and(|node| super::is_call(&node))
                {
                    self.semantic_token_kinds
                        .get_index_of(&SemanticTokenKind::Func)
                } else {
                    self.semantic_token_kinds
                        .get_index_of(&SemanticTokenKind::Number)
                }
            }
            SyntaxKind::FLOAT => self
                .semantic_token_kinds
                .get_index_of(&SemanticTokenKind::Number),
            SyntaxKind::IDENT => {
                // TODO: detect `local.get` and `local.set`
                let parent = token.parent();
                if parent
                    .as_ref()
                    .and_then(|parent| parent.parent())
                    .is_some_and(|node| super::is_call(&node))
                    || parent
                        .as_ref()
                        .is_some_and(|node| node.kind() == SyntaxKind::MODULE_FIELD_FUNC)
                {
                    self.semantic_token_kinds
                        .get_index_of(&SemanticTokenKind::Func)
                } else if parent
                    .as_ref()
                    .is_some_and(|node| node.kind() == SyntaxKind::PARAM)
                {
                    self.semantic_token_kinds
                        .get_index_of(&SemanticTokenKind::Param)
                } else {
                    self.semantic_token_kinds
                        .get_index_of(&SemanticTokenKind::Var)
                }
            }
            SyntaxKind::STRING => self
                .semantic_token_kinds
                .get_index_of(&SemanticTokenKind::String),
            SyntaxKind::LINE_COMMENT | SyntaxKind::BLOCK_COMMENT => self
                .semantic_token_kinds
                .get_index_of(&SemanticTokenKind::Comment),
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
}
