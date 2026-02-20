use super::Diagnostic;
use crate::{
    binder::{Symbol, SymbolKey, SymbolTable},
    cfa::{self, BasicBlock, ControlFlowGraph, FlowNode, FlowNodeKind},
    document::Document,
    helpers::{BumpCollectionsExt, BumpHashMap},
    types_analyzer,
};
use bumpalo::Bump;
use petgraph::graph::NodeIndex;
use std::cell::Cell;
use wat_syntax::AmberNode;

const DIAGNOSTIC_CODE: &str = "uninit";

pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    db: &dyn salsa::Database,
    document: Document,
    symbol_table: &SymbolTable,
    node: AmberNode,
    locals: &[&Symbol],
    bump: &mut Bump,
) {
    // avoid expensive analysis if there are no locals
    if locals.is_empty() {
        return;
    }
    let cfg = cfa::analyze(db, document, node.to_ptr());
    locals
        .iter()
        .filter(|local| types_analyzer::extract_type(db, &local.green).is_some_and(|ty| !ty.defaultable()))
        .for_each(|local| {
            check_local(diagnostics, db, local, symbol_table, cfg, bump);
            bump.reset();
        });
}

fn check_local(
    diagnostics: &mut Vec<Diagnostic>,
    db: &dyn salsa::Database,
    local: &Symbol,
    symbol_table: &SymbolTable,
    cfg: &ControlFlowGraph,
    bump: &Bump,
) {
    let mut block_marks = BumpHashMap::with_capacity_in(cfg.graph.node_count(), bump);
    block_marks.extend(cfg.graph.node_indices().filter_map(|node_index| {
        cfg.graph.node_weight(node_index).and_then(|node| {
            if node.unreachable {
                None
            } else {
                match &node.kind {
                    FlowNodeKind::BasicBlock(bb) => Some((node_index, BlockMark::new(bb, symbol_table, local.key))),
                    FlowNodeKind::BlockEntry(..) | FlowNodeKind::BlockExit => Some((node_index, BlockMark::default())),
                    _ => None,
                }
            }
        })
    }));
    hydrate_block_marks(cfg, &mut block_marks);
    cfg.graph.node_indices().for_each(|node_index| {
        if let Some(FlowNode {
            kind: FlowNodeKind::BasicBlock(bb),
            unreachable: false,
        }) = cfg.graph.node_weight(node_index)
            && let Some(mark) = block_marks.get_mut(&node_index)
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

fn hydrate_block_marks(cfg: &ControlFlowGraph, block_marks: &mut BumpHashMap<NodeIndex, BlockMark>) {
    let mut changed = true;
    while changed {
        changed = false;
        block_marks.iter().for_each(|(node_index, mark)| {
            let initialized = cfg
                .graph
                .neighbors_directed(*node_index, petgraph::Direction::Incoming)
                .filter_map(|node_index| block_marks.get(&node_index))
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
    bb.0.iter().filter_map(move |instr| match instr.name.text() {
        "local.get" => {
            if let Some(immediate) = instr.immediates.first().copied()
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
            if let Some(immediate) = instr.immediates.first().copied()
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
    })
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
                    .filter(|instr| matches!(instr.name.text(), "local.set" | "local.tee"))
                    .any(|instr| {
                        instr
                            .immediates
                            .first()
                            .copied()
                            .and_then(|immediate| symbol_table.resolved.get(&immediate.into()))
                            .is_some_and(|key| *key == def_key)
                    }),
            ),
        }
    }
}
