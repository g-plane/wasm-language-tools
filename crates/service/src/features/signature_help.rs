use crate::{
    LanguageService,
    binder::{Symbol, SymbolKey, SymbolTable},
    helpers::{self, LineIndexExt},
    idx::Idx,
    types_analyzer::{self, ValType},
};
use lspt::{
    MarkupContent, MarkupKind, ParameterInformation, SignatureHelp, SignatureHelpParams, SignatureInformation, Union2,
};
use std::fmt::Write;
use wat_syntax::{
    SyntaxKind, SyntaxNodePtr,
    ast::{AstNode, Instr, PlainInstr},
};

impl LanguageService {
    /// Handler for `textDocument/signatureHelp` request.
    pub fn signature_help(&self, params: SignatureHelpParams) -> Option<SignatureHelp> {
        let document = self.get_document(params.text_document.uri)?;
        self.with_db(|db| {
            let line_index = document.line_index(db);
            let root = document.root_tree(db);
            let symbol_table = SymbolTable::of(db, document);

            let token = helpers::syntax::find_token(&root, line_index.convert(params.position)?)?;
            let (node, instr, is_next) = if token.kind() == SyntaxKind::ERROR {
                (token.parent(), token.prev_siblings().find_map(Instr::cast), true)
            } else {
                let instr = token.parent();
                (instr.parent()?, Instr::cast(instr), false)
            };
            let parent_instr = PlainInstr::cast(node.clone())?;
            let (signature, func) = match parent_instr.instr_name()?.text() {
                "call" | "return_call" => {
                    let first_immediate = parent_instr.immediates().next()?;
                    let func = symbol_table.find_def(SymbolKey::new(first_immediate.syntax()))?;
                    (
                        types_analyzer::get_func_sig(db, document, func.key, &func.green),
                        Some(func),
                    )
                }
                "call_indirect" | "return_call_indirect" => {
                    let type_use = parent_instr.immediates().find_map(|immediate| immediate.type_use())?;
                    let type_use = type_use.syntax();
                    let mut sig =
                        types_analyzer::get_type_use_sig(db, document, SyntaxNodePtr::new(type_use), type_use.green());
                    sig.params.push((ValType::I32, None));
                    (sig, None)
                }
                _ => return None,
            };

            let mut label = "(func".to_string();
            let mut parameters = Vec::with_capacity(signature.params.len());
            if let Some(Symbol {
                idx: Idx { name: Some(name), .. },
                ..
            }) = func
            {
                label.push(' ');
                label.push_str(name.ident(db));
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
                        label.push_str(name.ident(db));
                    }
                    label.push(' ');
                    let _ = write!(label, "{}", param.0.render(db));
                    label.push(')');
                    parameters.push(ParameterInformation {
                        label: Union2::B((start as u32, label.len() as u32)),
                        documentation: None,
                    });
                    written = true;
                });
                signature.results.iter().for_each(|result| {
                    if written {
                        label.push(' ');
                    }
                    label.push_str("(result ");
                    let _ = write!(label, "{}", result.render(db));
                    label.push(')');
                    written = true;
                });
            }
            label.push(')');
            Some(SignatureHelp {
                signatures: vec![SignatureInformation {
                    label,
                    documentation: func.map(|func| {
                        Union2::B(MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: helpers::syntax::get_doc_comment(&func.key.to_node(&root)),
                        })
                    }),
                    parameters: Some(parameters),
                    active_parameter: instr
                        .and_then(|instr| {
                            let instr = instr.syntax();
                            node.children_by_kind(Instr::can_cast)
                                .position(|child| &child == instr)
                                .map(|index| if is_next { index + 1 } else { index } as u32)
                        })
                        .or_else(|| (!signature.params.is_empty() && is_next).then_some(0)),
                }],
                active_signature: Some(0),
                active_parameter: None,
            })
        })
        .flatten()
    }
}
