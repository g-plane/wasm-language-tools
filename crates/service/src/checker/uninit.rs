use super::Diagnostic;
use crate::{
    binder::{Symbol, SymbolKey, SymbolTable},
    cfa::{self, BasicBlock, ControlFlowGraph, FlowNode, FlowNodeId, FlowNodeKind},
    helpers::{BumpCollectionsExt, BumpHashMap},
    types_analyzer,
};
use bumpalo::{Bump, collections::Vec as BumpVec};
use wat_syntax::{AmberNode, SyntaxKind};

const DIAGNOSTIC_CODE: &str = "uninit";

pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    db: &dyn salsa::Database,
    symbol_table: &SymbolTable,
    func: AmberNode,
    local: &Symbol,
    bump: &Bump,
) {
    if types_analyzer::extract_type(db, &local.green).is_none_or(|ty| ty.defaultable()) {
        return;
    }
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
            && let Some(mark) = block_marks.get_mut(&node_id)
        {
            diagnostics.extend(
                detect_uninit(bb, local.key, mark, symbol_table)
                    .filter_map(|key| symbol_table.symbols.get(key))
                    .map(|symbol| Diagnostic {
                        range: symbol.key.text_range(),
                        code: DIAGNOSTIC_CODE.into(),
                        message: format!("local `{}` is read before being initialized", symbol.idx.render(db)),
                        ..Default::default()
                    }),
            );
        }
    });
}

fn propagate(cfg: &ControlFlowGraph, block_marks: &mut BumpHashMap<FlowNodeId, BlockMark>, bump: &Bump) {
    // Propagate information from predecessor flow nodes to successor flow nodes.
    // For those flow nodes whose `out_kill` are initially true, they don't need to be added to worklist,
    // but we should add their outgoings to worklist.
    let mut worklist = BumpVec::from_iter_in(
        block_marks
            .iter()
            .filter(|(_, mark)| mark.out_kill)
            .filter_map(|(flow_node_id, _)| cfg.get_node(*flow_node_id))
            .flat_map(|flow_node| &flow_node.outgoings)
            .copied(),
        bump,
    );
    while let Some(flow_node_id) = worklist.pop() {
        let Some(current) = cfg.get_node(flow_node_id) else {
            continue;
        };
        let initialized = current
            .incomings
            .iter()
            .filter(|incoming| {
                // ignore loop back for assuming the first iteration
                if let Some(FlowNode {
                    kind: FlowNodeKind::BasicBlock(BasicBlock(instrs)),
                    ..
                }) = cfg.get_node(**incoming)
                    && let FlowNode {
                        kind: FlowNodeKind::BlockEntry(block_entry),
                        ..
                    } = current
                {
                    // label jumping always happens from the body of a block,
                    // so it's safe to compare syntax text range
                    block_entry.kind() != SyntaxKind::BLOCK_LOOP
                        || instrs
                            .last()
                            .is_some_and(|instr| !block_entry.text_range().contains(instr.range.end()))
                } else {
                    true
                }
            })
            .filter_map(|incoming| block_marks.get(incoming))
            .map(|mark| mark.out_kill)
            .reduce(|acc, cur| acc && cur)
            .unwrap_or_default();
        if !initialized {
            continue;
        }
        let Some(mark) = block_marks.get_mut(&flow_node_id) else {
            continue;
        };
        // Information flow: predecessor's `out_kill` --> successor's `in_kill`,
        // so only if all predecessor flow nodes whose `out_kill` are true,
        // current flow node's `in_kill` can be true.
        if mark.in_kill {
            continue;
        } else {
            mark.in_kill = true;
        }
        // Set `out_kill` true and add its outgoings to propagate further.
        if !mark.out_kill {
            mark.out_kill = true;
            worklist.extend_from_slice_copy(&current.outgoings);
        }
    }
}

fn detect_uninit(
    bb: &BasicBlock,
    def_key: SymbolKey,
    mark: &mut BlockMark,
    symbol_table: &SymbolTable,
) -> impl Iterator<Item = SymbolKey> {
    bb.instrs().filter_map(
        move |instr| match instr.tokens_by_kind(SyntaxKind::INSTR_NAME).next()?.text() {
            "local.get" => {
                if let Some(immediate) = instr.children_by_kind(SyntaxKind::IMMEDIATE).next()
                    && symbol_table
                        .find_def(immediate.into())
                        .is_some_and(|symbol| symbol.key == def_key)
                    && !mark.in_kill
                {
                    Some(immediate.into())
                } else {
                    None
                }
            }
            "local.set" | "local.tee" => {
                if let Some(immediate) = instr.children_by_kind(SyntaxKind::IMMEDIATE).next()
                    && symbol_table
                        .find_def(immediate.into())
                        .is_some_and(|symbol| symbol.key == def_key)
                {
                    mark.in_kill = true;
                }
                None
            }
            _ => None,
        },
    )
}

#[derive(Default)]
struct BlockMark {
    /// If true, local has been written by `local.set` or `local.tee` from incoming flow nodes.
    in_kill: bool,
    /// If true, local has been written by `local.set` or `local.tee` in this flow node
    /// and can be propagated to outgoing flow nodes.
    out_kill: bool,
}
impl BlockMark {
    fn new(bb: &BasicBlock, symbol_table: &SymbolTable, def_key: SymbolKey) -> Self {
        Self {
            in_kill: false,
            out_kill: bb
                .instrs()
                .filter(|instr| {
                    matches!(
                        instr
                            .tokens_by_kind(SyntaxKind::INSTR_NAME)
                            .next()
                            .map(|token| token.text()),
                        Some("local.set" | "local.tee")
                    )
                })
                .any(|instr| {
                    instr
                        .children_by_kind(SyntaxKind::IMMEDIATE)
                        .next()
                        .and_then(|immediate| symbol_table.find_def(immediate.into()))
                        .is_some_and(|symbol| symbol.key == def_key)
                }),
        }
    }
}
