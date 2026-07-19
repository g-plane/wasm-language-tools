use crate::{LanguageService, cfa, helpers::LineIndexExt};
use lspt::{ExecuteCommandParams, Position};
use wat_syntax::{SyntaxKind, SyntaxNode, SyntaxNodePtr, TextRange};

impl LanguageService {
    /// Handler for `workspace/executeCommand` request.
    pub fn execute_command(&self, params: ExecuteCommandParams) -> Option<serde_json::Value> {
        match &*params.command {
            // args: [uri: String, position: Position]
            "wasmLanguageTools.__generateControlFlowGraphDot" => {
                let mut args = params.arguments?;
                let position = serde_json::from_value::<Position>(args.pop()?).ok()?;
                let document = self.get_document(args.pop()?.as_str()?)?;

                let line_index = document.line_index(self);
                let range = TextRange::empty(line_index.convert(position)?);
                let func = SyntaxNode::new_root(document.root(self))
                    .child_at_range(range)
                    .and_then(|module| module.child_at_range(range))
                    .filter(|node| node.kind() == SyntaxKind::MODULE_FIELD_FUNC)?;
                let cfg = cfa::analyze(self, document, SyntaxNodePtr::new(&func));
                Some(serde_json::Value::String(cfg.generate_dot()))
            }
            _ => None,
        }
    }
}
