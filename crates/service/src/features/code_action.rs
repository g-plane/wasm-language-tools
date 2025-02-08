use crate::{
    binder::SymbolTablesCtx, helpers, refactorings::*, syntax_tree::SyntaxTreeCtx, uri::UrisCtx,
    LanguageService,
};
use lsp_types::{CodeActionKind, CodeActionOrCommand, CodeActionParams};
use wat_syntax::{SyntaxElement, SyntaxKind, SyntaxNode};

impl LanguageService {
    /// Handler for `textDocument/codeAction` request.
    pub fn code_action(&self, params: CodeActionParams) -> Option<Vec<CodeActionOrCommand>> {
        let uri = self.uri(params.text_document.uri.clone());
        let line_index = self.line_index(uri);
        let root = SyntaxNode::new_root(self.root(uri));
        let symbol_table = self.symbol_table(uri);

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
                if kind == CodeActionKind::QUICKFIX {
                    quickfix = true;
                } else if kind == CodeActionKind::REFACTOR_REWRITE {
                    rewrite = true;
                } else if kind == CodeActionKind::REFACTOR_INLINE {
                    inline = true;
                }
            });

        let mut actions = vec![];
        let range = helpers::lsp_range_to_rowan_range(&line_index, params.range)?;
        let mut node = root.clone();
        while let Some(SyntaxElement::Node(it)) = node.child_or_token_at_range(range) {
            match it.kind() {
                SyntaxKind::MODULE_FIELD_FUNC => {
                    if rewrite {
                        if let Some(action) = func_header_join::act(
                            self,
                            uri,
                            &line_index,
                            &it,
                            SyntaxKind::LOCAL,
                            range,
                        ) {
                            actions.push(CodeActionOrCommand::CodeAction(action));
                        }
                    }
                }
                SyntaxKind::PLAIN_INSTR => {
                    if quickfix {
                        if let Some(action) =
                            fix_invalid_mem_arg::act(self, uri, &line_index, &it, &params.context)
                        {
                            actions.push(CodeActionOrCommand::CodeAction(action));
                        }
                    }
                    if rewrite {
                        if let Some(action) = br_if_to_if_br::act(self, uri, &line_index, &it) {
                            actions.push(CodeActionOrCommand::CodeAction(action));
                        }
                    }
                }
                SyntaxKind::PARAM => {
                    if rewrite {
                        if let Some(action) =
                            func_header_split::act(self, uri, &line_index, &it, SyntaxKind::PARAM)
                        {
                            actions.push(CodeActionOrCommand::CodeAction(action));
                        }
                    }
                }
                SyntaxKind::RESULT => {
                    if rewrite {
                        if let Some(action) =
                            func_header_split::act(self, uri, &line_index, &it, SyntaxKind::RESULT)
                        {
                            actions.push(CodeActionOrCommand::CodeAction(action));
                        }
                    }
                }
                SyntaxKind::LOCAL => {
                    if rewrite {
                        if let Some(action) =
                            func_header_split::act(self, uri, &line_index, &it, SyntaxKind::LOCAL)
                        {
                            actions.push(CodeActionOrCommand::CodeAction(action));
                        }
                    }
                }
                SyntaxKind::TYPE_USE | SyntaxKind::FUNC_TYPE => {
                    if rewrite {
                        if let Some(action) = func_header_join::act(
                            self,
                            uri,
                            &line_index,
                            &it,
                            SyntaxKind::PARAM,
                            range,
                        ) {
                            actions.push(CodeActionOrCommand::CodeAction(action));
                        }
                        if let Some(action) = func_header_join::act(
                            self,
                            uri,
                            &line_index,
                            &it,
                            SyntaxKind::RESULT,
                            range,
                        ) {
                            actions.push(CodeActionOrCommand::CodeAction(action));
                        }
                    }
                    if inline {
                        if let Some(action) =
                            inline_func_type::act(self, uri, &line_index, &root, &symbol_table, &it)
                        {
                            actions.push(CodeActionOrCommand::CodeAction(action));
                        }
                    }
                }
                SyntaxKind::BLOCK_IF => {
                    if rewrite {
                        if let Some(action) = if_br_to_br_if::act(self, uri, &line_index, &it) {
                            actions.push(CodeActionOrCommand::CodeAction(action));
                        }
                    }
                }
                SyntaxKind::GLOBAL_TYPE => {
                    if quickfix {
                        if let Some(action) =
                            remove_mut::act(self, uri, &line_index, &it, &params.context)
                        {
                            actions.push(CodeActionOrCommand::CodeAction(action));
                        }
                    }
                }
                SyntaxKind::IMMEDIATE | SyntaxKind::INDEX => {
                    if rewrite {
                        if let Some(action) =
                            idx_conversion::act(self, uri, &line_index, &symbol_table, &it)
                        {
                            actions.push(CodeActionOrCommand::CodeAction(action));
                        }
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
