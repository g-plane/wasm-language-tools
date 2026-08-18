use super::Diagnostic;
use crate::{
    binder::{Symbol, SymbolKey, SymbolTable},
    cfa::{self, BasicBlock, ControlFlowGraph, FlowNode, FlowNodeId, FlowNodeKind},
    config::LintLevel,
    helpers::{BumpCollectionsExt, BumpHashMap},
};
use bumpalo::{Bump, collections::Vec as BumpVec};
use lspt::DiagnosticSeverity;
use wat_syntax::{AmberNode, SyntaxKind};

const DIAGNOSTIC_CODE: &str = "unread";

pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    db: &dyn salsa::Database,
    lint_level: LintLevel,
    symbol_table: &SymbolTable,
    func: AmberNode,
    local: &Symbol,
    bump: &Bump,
) {
    let severity = match lint_level {
        LintLevel::Allow => return,
        LintLevel::Hint => DiagnosticSeverity::Hint,
        LintLevel::Warn => DiagnosticSeverity::Warning,
        LintLevel::Deny => DiagnosticSeverity::Error,
    };

    let cfg = cfa::analyze(db, func.green().clone().into(), func.text_range());

    let mut block_marks = BumpHashMap::with_capacity_in(cfg.nodes().len(), bump);
    block_marks.extend(cfg.nodes_with_ids().filter_map(|(flow_node, node_id)| {
        if flow_node.unreachable {
            None
        } else {
            match &flow_node.kind {
                FlowNodeKind::BasicBlock(bb) => Some((node_id, BlockMark::new(bb, symbol_table, local.key))),
                FlowNodeKind::BlockEntry(..) | FlowNodeKind::BlockExit => Some((node_id, BlockMark::default())),
                _ => None,
            }
        }
    }));
    propagate(cfg, &mut block_marks, bump);
    cfg.nodes_with_ids().for_each(|(flow_node, node_id)| {
        if let FlowNode {
            kind: FlowNodeKind::BasicBlock(bb),
            unreachable: false,
            ..
        } = flow_node
            && let Some(mark) = block_marks.get(&node_id)
        {
            diagnostics.extend(
                detect_unread(bb, local.key, mark, symbol_table, bump)
                    .filter_map(|key| symbol_table.symbols.get(key))
                    .map(|symbol| Diagnostic {
                        range: symbol.key.text_range(),
                        severity,
                        code: DIAGNOSTIC_CODE.into(),
                        message: format!("local `{}` is set but never read", symbol.idx.render(db)),
                        ..Default::default()
                    }),
            );
        }
    });
}

fn propagate(cfg: &ControlFlowGraph, block_marks: &mut BumpHashMap<FlowNodeId, BlockMark>, bump: &Bump) {
    // Propagate information from successor flow nodes to predecessors flow nodes.
    // For those flow nodes whose `in_gen` are initially true, they don't need to be added to worklist,
    // but we should add their incomings to worklist.
    let mut worklist = BumpVec::from_iter_in(
        block_marks
            .iter()
            .filter(|(_, mark)| mark.in_gen)
            .filter_map(|(flow_node_id, _)| cfg.get_node(*flow_node_id))
            .flat_map(|flow_node| &flow_node.incomings)
            .copied(),
        bump,
    );
    while let Some(flow_node_id) = worklist.pop() {
        let Some(mark) = block_marks.get_mut(&flow_node_id) else {
            continue;
        };
        // Information flow: successor's `in_gen` --> predecessor's `out_gen`,
        // so if there're any successor flow nodes whose `in_gen` are true,
        // current flow node's `out_gen` will be true.
        // Also, skip a flow node whose `out_gen` is already true.
        // This can happen when a flow node has multiple outcomings whose `in_gen` are true.
        if mark.out_gen {
            continue;
        } else {
            mark.out_gen = true;
        }
        // Since information is propagated from successors to predecessors by respecting `in_gen`,
        // if `in_gen` of this flow node is marked true, it will be propagated further.
        // If this flow node has no local read before first write, `kill` prevents
        // the read information from propagating through this flow node.
        // But what about any read before first write?
        // In such case, `in_gen` is already true before propagation.
        if !mark.kill && !mark.in_gen {
            mark.in_gen = true;
            if let Some(flow_node) = cfg.get_node(flow_node_id) {
                worklist.extend_from_slice_copy(&flow_node.incomings);
            }
        }
    }
}

fn detect_unread(
    bb: &BasicBlock,
    def_key: SymbolKey,
    mark: &BlockMark,
    symbol_table: &SymbolTable,
    bump: &Bump,
) -> impl Iterator<Item = SymbolKey> {
    let mut set = BumpVec::<Option<SymbolKey>>::with_capacity_in(1, bump);
    bb.instrs().for_each(|instr| {
        match instr
            .tokens_by_kind(SyntaxKind::INSTR_NAME)
            .next()
            .map(|token| token.text())
        {
            Some("local.get") => {
                if let Some(last) = set.last_mut()
                    && last.is_some()
                    && let Some(immediate) = instr.children_by_kind(SyntaxKind::IMMEDIATE).next()
                    && symbol_table
                        .find_def(immediate.into())
                        .is_some_and(|symbol| symbol.key == def_key)
                {
                    *last = None;
                }
            }
            Some("local.set" | "local.tee") => {
                if let Some(immediate) = instr.children_by_kind(SyntaxKind::IMMEDIATE).next()
                    && symbol_table
                        .find_def(immediate.into())
                        .is_some_and(|symbol| symbol.key == def_key)
                {
                    set.push(Some(immediate.into()));
                }
            }
            _ => {}
        }
    });
    if mark.out_gen
        && let Some(last) = set.last_mut()
    {
        *last = None;
    }
    set.into_iter().flatten()
}

#[derive(Default)]
struct BlockMark {
    /// If true, local has been read by `local.get` before first write in this flow node
    /// and can be propagated to incoming flow nodes.
    in_gen: bool,
    /// If true, local has been read by `local.get` from outcoming flow nodes.
    out_gen: bool,
    /// If true, local has been written by `local.set` or `local.tee` in this flow node.
    kill: bool,
}
impl BlockMark {
    fn new(bb: &BasicBlock, symbol_table: &SymbolTable, def_key: SymbolKey) -> Self {
        let mut in_gen = false;
        let mut kill = false;
        bb.instrs().for_each(|instr| {
            match instr
                .tokens_by_kind(SyntaxKind::INSTR_NAME)
                .next()
                .map(|token| token.text())
            {
                Some("local.get") => {
                    // `in_gen` will be propagated to incomings,
                    // so it must be before any `local.set` or `local.tee` instructions.
                    if !kill
                        && let Some(immediate) = instr.children_by_kind(SyntaxKind::IMMEDIATE).next()
                        && symbol_table
                            .find_def(immediate.into())
                            .is_some_and(|symbol| symbol.key == def_key)
                    {
                        in_gen = true;
                    }
                }
                Some("local.set" | "local.tee") => {
                    if let Some(immediate) = instr.children_by_kind(SyntaxKind::IMMEDIATE).next()
                        && symbol_table
                            .find_def(immediate.into())
                            .is_some_and(|symbol| symbol.key == def_key)
                    {
                        kill = true;
                    }
                }
                _ => {}
            }
        });
        Self {
            in_gen,
            out_gen: false,
            kill,
        }
    }
}
