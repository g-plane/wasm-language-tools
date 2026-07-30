use super::Diagnostic;
use crate::{
    binder::{Symbol, SymbolKey, SymbolTable},
    cfa::{self, BasicBlock, ControlFlowGraph, FlowNode, FlowNodeId, FlowNodeKind},
    document::Document,
    helpers::{BumpCollectionsExt, BumpHashMap},
    types_analyzer,
};
use bumpalo::Bump;
use std::cell::Cell;
use wat_syntax::{AmberNode, SyntaxKind};

const DIAGNOSTIC_CODE: &str = "uninit";

pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    db: &dyn salsa::Database,
    document: Document,
    symbol_table: &SymbolTable,
    func: AmberNode,
    local: &Symbol,
    bump: &Bump,
) {
    if types_analyzer::extract_type(db, &local.ty.0).is_none_or(|ty| ty.defaultable()) {
        return;
    }
    let cfg = cfa::analyze(db, document, func.into());
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
    hydrate_block_marks(cfg, &mut block_marks);
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
                    .filter_map(|key| symbol_table.symbols.get(&key))
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

fn hydrate_block_marks(cfg: &ControlFlowGraph, block_marks: &mut BumpHashMap<FlowNodeId, BlockMark>) {
    let mut changed = true;
    while changed {
        changed = false;
        block_marks.iter().for_each(|(node_id, mark)| {
            let Some(current) = cfg.get_node(*node_id) else {
                return;
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
                .map(|mark| mark.out.get())
                .reduce(|acc, cur| acc && cur)
                .unwrap_or_default();
            if initialized {
                if !mark.r#in.get() {
                    mark.r#in.set(true);
                    changed = true;
                }
                if !mark.out.get() {
                    mark.out.set(true);
                    changed = true;
                }
            }
        });
    }
}

fn detect_uninit(
    bb: &BasicBlock,
    def_key: SymbolKey,
    mark: &mut BlockMark,
    symbol_table: &SymbolTable,
) -> impl Iterator<Item = SymbolKey> {
    bb.0.iter()
        .map(|instr| AmberNode::new(&instr.green, instr.range.start()))
        .filter_map(
            move |instr| match instr.tokens_by_kind(SyntaxKind::INSTR_NAME).next()?.text() {
                "local.get" => {
                    if let Some(immediate) = instr.children_by_kind(SyntaxKind::IMMEDIATE).next()
                        && symbol_table
                            .resolved
                            .get(&immediate.into())
                            .is_some_and(|key| *key == def_key)
                        && !mark.r#in.get()
                    {
                        Some(immediate.into())
                    } else {
                        None
                    }
                }
                "local.set" | "local.tee" => {
                    if let Some(immediate) = instr.children_by_kind(SyntaxKind::IMMEDIATE).next()
                        && symbol_table
                            .resolved
                            .get(&immediate.into())
                            .is_some_and(|key| *key == def_key)
                    {
                        *mark.r#in.get_mut() = true;
                    }
                    None
                }
                _ => None,
            },
        )
}

#[derive(Default)]
struct BlockMark {
    r#in: Cell<bool>,
    out: Cell<bool>,
}
impl BlockMark {
    fn new(bb: &BasicBlock, symbol_table: &SymbolTable, def_key: SymbolKey) -> Self {
        Self {
            r#in: Cell::new(false),
            out: Cell::new(
                bb.0.iter()
                    .map(|instr| AmberNode::new(&instr.green, instr.range.start()))
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
                            .and_then(|immediate| symbol_table.resolved.get(&immediate.into()))
                            .is_some_and(|key| *key == def_key)
                    }),
            ),
        }
    }
}
