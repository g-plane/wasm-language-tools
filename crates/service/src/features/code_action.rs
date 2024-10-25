use crate::{files::FilesCtx, helpers, refactorings::*, LanguageService};
use lsp_types::{CodeActionKind, CodeActionOrCommand, CodeActionParams};
use wat_syntax::{SyntaxKind, SyntaxNode};

impl LanguageService {
    pub fn code_action(&self, params: CodeActionParams) -> Option<Vec<CodeActionOrCommand>> {
        let uri = self.uri(params.text_document.uri.clone());
        let line_index = self.line_index(uri);
        let root = SyntaxNode::new_root(self.root(uri));
        let node = root
            .child_or_token_at_range(helpers::lsp_range_to_rowan_range(
                &line_index,
                params.range,
            )?)?
            .into_node()?;

        let mut quickfix = false;
        params.context.only.into_iter().flatten().for_each(|kind| {
            if kind == CodeActionKind::QUICKFIX {
                quickfix = true;
            }
        });

        let mut actions = vec![];
        node.descendants().for_each(|node| {
            if node.kind() == SyntaxKind::PLAIN_INSTR {
                if quickfix {
                    if let Some(action) = fix_invalid_mem_arg::act(self, uri, &line_index, &node) {
                        actions.push(CodeActionOrCommand::CodeAction(action));
                    }
                }
            }
        });
        if actions.is_empty() {
            None
        } else {
            Some(actions)
        }
    }
}
