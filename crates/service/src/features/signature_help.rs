use crate::{
    binder::SymbolTablesCtx, files::FilesCtx, helpers, idx::IdentsCtx,
    types_analyzer::TypesAnalyzerCtx, LanguageService,
};
use lsp_types::{
    Documentation, MarkupContent, MarkupKind, ParameterInformation, ParameterLabel, SignatureHelp,
    SignatureHelpParams, SignatureInformation,
};
use rowan::{ast::support, Direction};
use wat_syntax::{SyntaxElement, SyntaxKind, SyntaxNode};

impl LanguageService {
    pub fn signature_help(&self, params: SignatureHelpParams) -> Option<SignatureHelp> {
        let uri = self.uri(params.text_document_position_params.text_document.uri);
        let line_index = self.line_index(uri);
        let root = SyntaxNode::new_root(self.root(uri));
        let token = helpers::ast::find_token(
            &root,
            helpers::lsp_pos_to_rowan_pos(
                &line_index,
                params.text_document_position_params.position,
            )?,
        )?;
        let (node, operand, is_next) = if token.kind() == SyntaxKind::ERROR {
            (
                token.parent()?,
                token
                    .siblings_with_tokens(Direction::Prev)
                    .skip(1)
                    .find_map(|sibling| match sibling {
                        SyntaxElement::Node(node) if node.kind() == SyntaxKind::OPERAND => {
                            Some(node)
                        }
                        _ => None,
                    }),
                true,
            )
        } else {
            let operand = token.parent()?.parent()?;
            (operand.parent()?, Some(operand), false)
        };
        if node.kind() != SyntaxKind::PLAIN_INSTR
            || support::token(&node, SyntaxKind::INSTR_NAME)
                .is_none_or(|token| token.text() != "call")
        {
            return None;
        }

        let symbol_table = self.symbol_table(uri);
        let func = symbol_table
            .find_defs(
                &node
                    .children()
                    .find(|child| child.kind() == SyntaxKind::OPERAND)?
                    .into(),
            )?
            .next()?;
        let signature = self
            .get_func_sig(uri, func.clone().into())
            .unwrap_or_default();

        let mut label = "(func".to_string();
        let mut parameters = Vec::with_capacity(signature.params.len());
        if let Some(name) = func.idx.name {
            label.push(' ');
            label.push_str(&self.lookup_ident(name));
        }
        if !signature.params.is_empty() || !signature.results.is_empty() {
            label.push(' ');
            let mut written = false;
            signature.params.iter().for_each(|param| {
                if written {
                    label.push(' ');
                }
                let start = label.len();
                label.push_str("(param");
                if let Some(name) = param.1 {
                    label.push(' ');
                    label.push_str(&self.lookup_ident(name));
                }
                label.push(' ');
                label.push_str(&param.0.to_string());
                label.push(')');
                parameters.push(ParameterInformation {
                    label: ParameterLabel::LabelOffsets([start as u32, label.len() as u32]),
                    documentation: None,
                });
                written = true;
            });
            signature.results.iter().for_each(|result| {
                if written {
                    label.push(' ');
                }
                label.push_str("(result ");
                label.push_str(&result.to_string());
                label.push(')');
                written = true;
            });
        }
        label.push(')');
        Some(SignatureHelp {
            signatures: vec![SignatureInformation {
                label,
                documentation: Some(Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: helpers::ast::get_doc_comment(&func.key.ptr.to_node(&root)),
                })),
                parameters: Some(parameters),
                active_parameter: operand.and_then(|operand| {
                    node.children()
                        .filter(|child| {
                            child.kind() == SyntaxKind::OPERAND
                                && child.first_child().is_some_and(|child| {
                                    matches!(
                                        child.kind(),
                                        SyntaxKind::PLAIN_INSTR
                                            | SyntaxKind::BLOCK_BLOCK
                                            | SyntaxKind::BLOCK_IF
                                            | SyntaxKind::BLOCK_LOOP
                                    )
                                })
                        })
                        .position(|child| child == operand)
                        .map(|index| if is_next { index + 1 } else { index } as u32)
                        .or_else(|| (!signature.params.is_empty() && is_next).then_some(0))
                }),
            }],
            active_signature: Some(0),
            active_parameter: None,
        })
    }
}
