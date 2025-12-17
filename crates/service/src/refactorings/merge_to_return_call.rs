use crate::{
    LanguageService,
    binder::{SymbolKey, SymbolTable},
    cfa::{self, FlowNode, FlowNodeKind},
    document::Document,
    helpers,
    uri::InternUri,
};
use line_index::LineIndex;
use lspt::{CodeAction, CodeActionKind, TextEdit, WorkspaceEdit};
use petgraph::visit::Dfs;
use rowan::{
    NodeOrToken,
    ast::{SyntaxNodePtr, support},
};
use rustc_hash::{FxBuildHasher, FxHashMap};
use wat_syntax::{SyntaxKind, SyntaxNode};

pub fn act(
    service: &LanguageService,
    uri: InternUri,
    document: Document,
    line_index: &LineIndex,
    symbol_table: &SymbolTable,
    node: &SyntaxNode,
) -> Option<CodeAction> {
    let instr_name = node
        .first_child_or_token_by_kind(&|kind| kind == SyntaxKind::INSTR_NAME)?
        .into_token()?;
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
                .next_sibling()
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
                .first_child()
                .or_else(|| node.prev_sibling())
                .filter(|node| node.kind() == SyntaxKind::PLAIN_INSTR)?;
            if support::token(&prev_instr, SyntaxKind::INSTR_NAME)
                .is_some_and(|token| token.text() != "call")
            {
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
        .get(&SymbolKey::new(&call_instr.first_child_by_kind(
            &|kind| kind == SyntaxKind::IMMEDIATE,
        )?))
        .is_none_or(|def_key| *def_key != SymbolKey::new(&func))
    {
        return None;
    }

    let cfg = cfa::analyze(service, document, SyntaxNodePtr::new(&func));
    let bb_node_index = cfg.graph.node_indices().find(|node_index| {
        if let Some(FlowNode {
            kind: FlowNodeKind::BasicBlock(bb),
            ..
        }) = cfg.graph.node_weight(*node_index)
        {
            bb.contains_instr(node)
        } else {
            false
        }
    })?;
    let mut dfs = Dfs::new(&cfg.graph, bb_node_index);
    dfs.next(&cfg.graph); // skip the starting node
    while let Some(next) = dfs.next(&cfg.graph) {
        // make sure there're no basic blocks or block entries after the "call-return" pair
        if !matches!(
            cfg.graph.node_weight(next),
            Some(FlowNode {
                kind: FlowNodeKind::BlockExit | FlowNodeKind::Exit,
                ..
            })
        ) {
            return None;
        }
    }

    let mut text_edits = vec![TextEdit {
        range: helpers::rowan_range_to_lsp_range(
            line_index,
            support::token(&call_instr, SyntaxKind::INSTR_NAME)?.text_range(),
        ),
        new_text: "return_call".into(),
    }];
    if let Some(instr) = &return_instr {
        text_edits.extend(instr.children_with_tokens().filter_map(|node_or_token| {
            match node_or_token {
                NodeOrToken::Token(token)
                    if matches!(
                        token.kind(),
                        SyntaxKind::L_PAREN | SyntaxKind::INSTR_NAME | SyntaxKind::R_PAREN
                    ) =>
                {
                    Some(TextEdit {
                        range: helpers::rowan_range_to_lsp_range(line_index, token.text_range()),
                        new_text: "".into(),
                    })
                }
                _ => None,
            }
        }));
    }
    let mut changes = FxHashMap::with_capacity_and_hasher(1, FxBuildHasher);
    changes.insert(uri.raw(service), text_edits);
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
