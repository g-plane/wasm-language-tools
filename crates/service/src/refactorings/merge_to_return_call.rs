use crate::{
    binder::{SymbolKey, SymbolTable},
    cfa::{self, FlowNode, FlowNodeKind},
    document::Document,
    helpers::LineIndexExt,
    uri::InternUri,
};
use line_index::LineIndex;
use lspt::{CodeAction, CodeActionKind, TextEdit, WorkspaceEdit};
use rustc_hash::{FxBuildHasher, FxHashMap};
use wat_syntax::{SyntaxKind, SyntaxNode, SyntaxNodePtr, ast::support};

pub fn act(
    db: &dyn salsa::Database,
    uri: InternUri,
    document: Document,
    line_index: &LineIndex,
    symbol_table: &SymbolTable,
    node: &SyntaxNode,
) -> Option<CodeAction> {
    let instr_name = support::token(node, SyntaxKind::INSTR_NAME)?;
    let (call_instr, return_instr) = match instr_name.text() {
        "call" => {
            if node
                .parent()
                .is_some_and(|parent| parent.kind() == SyntaxKind::PLAIN_INSTR)
            {
                // if parent instr is `return`, this will be handled in the "return" case below;
                // for other instrs, it can't be replaced with `return_call`
                return None;
            }
            let next_instr = node
                .next_siblings()
                .next()
                .filter(|node| node.kind() == SyntaxKind::PLAIN_INSTR);
            if next_instr
                .as_ref()
                .and_then(|parent| support::token(parent, SyntaxKind::INSTR_NAME))
                .is_some_and(|token| token.text() != "return")
            {
                return None;
            }
            (node.clone(), next_instr)
        }
        "return" => {
            let prev_instr = node
                .children()
                .next()
                .or_else(|| node.prev_siblings().next())
                .filter(|node| node.kind() == SyntaxKind::PLAIN_INSTR)?;
            if support::token(&prev_instr, SyntaxKind::INSTR_NAME).is_some_and(|token| token.text() != "call") {
                return None;
            }
            (prev_instr, Some(node.clone()))
        }
        _ => return None,
    };

    let func = node
        .ancestors()
        .find(|ancestor| ancestor.kind() == SyntaxKind::MODULE_FIELD_FUNC)?;
    if symbol_table
        .resolved
        .get(&SymbolKey::new(
            &call_instr.children_by_kind(SyntaxKind::IMMEDIATE).next()?,
        ))
        .is_none_or(|def_key| *def_key != SymbolKey::new(&func))
    {
        return None;
    }

    let cfg = cfa::analyze(db, document, SyntaxNodePtr::new(&func));
    let bb_flow_node = cfg.nodes().iter().find(|flow_node| {
        if let FlowNode {
            kind: FlowNodeKind::BasicBlock(bb),
            ..
        } = flow_node
        {
            bb.contains_instr(node)
        } else {
            false
        }
    })?;
    let mut pending_flow_node_ids = bb_flow_node.outgoings.to_vec();
    while let Some(flow_node) = pending_flow_node_ids.pop().and_then(|node_id| cfg.get_node(node_id)) {
        // make sure there're no basic blocks or block entries after the "call-return" pair
        if matches!(
            flow_node,
            FlowNode {
                kind: FlowNodeKind::BlockExit | FlowNodeKind::Exit,
                ..
            }
        ) {
            pending_flow_node_ids.extend_from_slice(&flow_node.outgoings);
        } else {
            return None;
        }
    }

    let mut text_edits = vec![TextEdit {
        range: line_index.convert(support::token(&call_instr, SyntaxKind::INSTR_NAME)?.text_range())?,
        new_text: "return_call".into(),
    }];
    if let Some(instr) = &return_instr {
        text_edits.extend(
            instr
                .tokens_by_kind([SyntaxKind::L_PAREN, SyntaxKind::INSTR_NAME, SyntaxKind::R_PAREN])
                .filter_map(|token| line_index.convert(token.text_range()))
                .map(|range| TextEdit {
                    range,
                    new_text: "".into(),
                }),
        );
    }
    let mut changes = FxHashMap::with_capacity_and_hasher(1, FxBuildHasher);
    changes.insert(uri.raw(db), text_edits);
    Some(CodeAction {
        title: if return_instr.is_some() {
            "Merge `call` and `return` to `return_call`".into()
        } else {
            "Replace `call` with `return_call`".into()
        },
        kind: Some(CodeActionKind::RefactorRewrite),
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        }),
        ..Default::default()
    })
}
