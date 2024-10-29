use crate::{files::FilesCtx, helpers, refactorings::*, LanguageService};
use lsp_types::{CodeActionKind, CodeActionOrCommand, CodeActionParams};
use wat_syntax::{SyntaxElement, SyntaxKind, SyntaxNode};

impl LanguageService {
    pub fn code_action(&self, params: CodeActionParams) -> Option<Vec<CodeActionOrCommand>> {
        let uri = self.uri(params.text_document.uri.clone());
        let line_index = self.line_index(uri);

        let mut quickfix = params.context.only.is_none();
        let mut rewrite = params.context.only.is_none();
        params.context.only.into_iter().flatten().for_each(|kind| {
            if kind == CodeActionKind::QUICKFIX {
                quickfix = true;
            } else if kind == CodeActionKind::REFACTOR_REWRITE {
                rewrite = true;
            }
        });

        let mut actions = vec![];
        let range = helpers::lsp_range_to_rowan_range(&line_index, params.range)?;
        let mut node = SyntaxNode::new_root(self.root(uri));
        while let Some(SyntaxElement::Node(it)) = node.child_or_token_at_range(range) {
            match it.kind() {
                SyntaxKind::PLAIN_INSTR => {
                    if quickfix {
                        if let Some(action) = fix_invalid_mem_arg::act(self, uri, &line_index, &it)
                        {
                            actions.push(CodeActionOrCommand::CodeAction(action));
                        }
                    }
                }
                SyntaxKind::PARAM => {
                    if rewrite {
                        if let Some(action) = params_split::act(self, uri, &line_index, &it) {
                            actions.push(CodeActionOrCommand::CodeAction(action));
                        }
                    }
                }
                SyntaxKind::TYPE_USE | SyntaxKind::FUNC_TYPE => {
                    if rewrite {
                        if let Some(action) = params_join::act(self, uri, &line_index, &it, range) {
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
