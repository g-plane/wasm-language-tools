use crate::{LanguageService, binder::SymbolTable, helpers, refactorings::*, uri::InternUri};
use lspt::{CodeAction, CodeActionKind, CodeActionParams};
use wat_syntax::{SyntaxElement, SyntaxKind};

impl LanguageService {
    /// Handler for `textDocument/codeAction` request.
    pub fn code_action(&self, params: CodeActionParams) -> Option<Vec<CodeAction>> {
        let uri = InternUri::new(self, &params.text_document.uri);
        let document = self.get_document(params.text_document.uri)?;
        let line_index = document.line_index(self);
        let root = document.root_tree(self);
        let symbol_table = SymbolTable::of(self, document);

        let mut quickfix = params.context.only.is_none();
        let mut rewrite = params.context.only.is_none();
        let mut inline = params.context.only.is_none();
        params
            .context
            .only
            .iter()
            .flatten()
            .cloned()
            .for_each(|kind| {
                if kind == CodeActionKind::QuickFix {
                    quickfix = true;
                } else if kind == CodeActionKind::RefactorRewrite {
                    rewrite = true;
                } else if kind == CodeActionKind::RefactorInline {
                    inline = true;
                }
            });

        let mut actions = vec![];
        let range = helpers::lsp_range_to_rowan_range(line_index, params.range)?;
        let mut node = root.clone();
        while let Some(SyntaxElement::Node(it)) = node.child_or_token_at_range(range) {
            match it.kind() {
                SyntaxKind::MODULE_FIELD_FUNC => {
                    if rewrite
                        && let Some(action) = func_header_join::act(
                            self,
                            uri,
                            line_index,
                            &it,
                            SyntaxKind::LOCAL,
                            range,
                        )
                    {
                        actions.push(action);
                    }
                }
                SyntaxKind::PLAIN_INSTR => {
                    if rewrite && let Some(action) = br_if_to_if_br::act(self, uri, line_index, &it)
                    {
                        actions.push(action);
                    }
                }
                SyntaxKind::PARAM => {
                    if rewrite
                        && let Some(action) =
                            func_header_split::act(self, uri, line_index, &it, SyntaxKind::PARAM)
                    {
                        actions.push(action);
                    }
                }
                SyntaxKind::RESULT => {
                    if rewrite
                        && let Some(action) =
                            func_header_split::act(self, uri, line_index, &it, SyntaxKind::RESULT)
                    {
                        actions.push(action);
                    }
                }
                SyntaxKind::LOCAL => {
                    if rewrite
                        && let Some(action) =
                            func_header_split::act(self, uri, line_index, &it, SyntaxKind::LOCAL)
                    {
                        actions.push(action);
                    }
                }
                SyntaxKind::TYPE_USE | SyntaxKind::FUNC_TYPE => {
                    if rewrite {
                        if let Some(action) = func_header_join::act(
                            self,
                            uri,
                            line_index,
                            &it,
                            SyntaxKind::PARAM,
                            range,
                        ) {
                            actions.push(action);
                        }
                        if let Some(action) = func_header_join::act(
                            self,
                            uri,
                            line_index,
                            &it,
                            SyntaxKind::RESULT,
                            range,
                        ) {
                            actions.push(action);
                        }
                    }
                    if inline
                        && let Some(action) =
                            inline_func_type::act(self, uri, line_index, &root, symbol_table, &it)
                    {
                        actions.push(action);
                    }
                }
                SyntaxKind::BLOCK_IF => {
                    if rewrite && let Some(action) = if_br_to_br_if::act(self, uri, line_index, &it)
                    {
                        actions.push(action);
                    }
                }
                SyntaxKind::GLOBAL_TYPE => {
                    if quickfix
                        && let Some(action) =
                            remove_mut::act(self, uri, line_index, &it, &params.context)
                    {
                        actions.push(action);
                    }
                }
                SyntaxKind::IMMEDIATE => {
                    if quickfix
                        && let Some(mut action) =
                            fix_packing::act(self, uri, line_index, &it, &params.context)
                    {
                        actions.append(&mut action);
                    }
                    if rewrite
                        && let Some(action) =
                            idx_conversion::act(self, uri, line_index, symbol_table, &it)
                    {
                        actions.push(action);
                    }
                }
                SyntaxKind::INDEX => {
                    if rewrite
                        && let Some(action) =
                            idx_conversion::act(self, uri, line_index, symbol_table, &it)
                    {
                        actions.push(action);
                    }
                }
                SyntaxKind::MEM_ARG => {
                    if quickfix
                        && let Some(action) =
                            fix_invalid_mem_arg::act(self, uri, line_index, &it, &params.context)
                    {
                        actions.push(action);
                    }
                }
                _ => {}
            }
            node = it;
        }

        if actions.is_empty() {
            None
        } else {
            Some(actions)
        }
    }
}
