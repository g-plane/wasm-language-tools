use super::Diagnostic;
use crate::{
    binder::{Symbol, SymbolKey, SymbolTable},
    cfa::{self, BasicBlock, ControlFlowGraph, FlowNode, FlowNodeKind},
    config::LintLevel,
    document::Document,
};
use lspt::DiagnosticSeverity;
use oxc_allocator::{Allocator, HashMap as OxcHashMap, Vec as OxcVec};
use petgraph::graph::NodeIndex;
use rowan::ast::SyntaxNodePtr;
use std::cell::Cell;
use wat_syntax::SyntaxNode;

const DIAGNOSTIC_CODE: &str = "unread";

#[expect(clippy::too_many_arguments)]
pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    db: &dyn salsa::Database,
    document: Document,
    lint_level: LintLevel,
    symbol_table: &SymbolTable,
    node: &SyntaxNode,
    locals: &[&Symbol],
    allocator: &mut Allocator,
) {
    let severity = match lint_level {
        LintLevel::Allow => return,
        LintLevel::Hint => DiagnosticSeverity::Hint,
        LintLevel::Warn => DiagnosticSeverity::Warning,
        LintLevel::Deny => DiagnosticSeverity::Error,
    };

    // avoid expensive analysis if there are no locals
    if locals.is_empty() {
        return;
    }
    let cfg = cfa::analyze(db, document, SyntaxNodePtr::new(node));
    locals.iter().for_each(|local| {
        check_local(diagnostics, db, severity, local, symbol_table, cfg, allocator);
        allocator.reset();
    });
}

fn check_local(
    diagnostics: &mut Vec<Diagnostic>,
    db: &dyn salsa::Database,
    severity: DiagnosticSeverity,
    local: &Symbol,
    symbol_table: &SymbolTable,
    cfg: &ControlFlowGraph,
    allocator: &Allocator,
) {
    let mut block_marks = OxcHashMap::from_iter_in(
        cfg.graph.node_indices().filter_map(|node_index| {
            cfg.graph.node_weight(node_index).and_then(|node| {
                if node.unreachable {
                    None
                } else {
                    match &node.kind {
                        FlowNodeKind::BasicBlock(bb) => Some((node_index, BlockMark::new(bb, symbol_table, local.key))),
                        FlowNodeKind::BlockEntry(..) | FlowNodeKind::BlockExit => {
                            Some((node_index, BlockMark::default()))
                        }
                        _ => None,
                    }
                }
            })
        }),
        allocator,
    );
    hydrate_block_marks(cfg, &mut block_marks);
    cfg.graph.node_indices().for_each(|node_index| {
        if let Some(FlowNode {
            kind: FlowNodeKind::BasicBlock(bb),
            unreachable: false,
        }) = cfg.graph.node_weight(node_index)
            && let Some(mark) = block_marks.get(&node_index)
        {
            diagnostics.extend(
                detect_unread(bb, local.key, mark, symbol_table, allocator)
                    .filter_map(|key| symbol_table.symbols.get(&key))
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

fn hydrate_block_marks(cfg: &ControlFlowGraph, block_marks: &mut OxcHashMap<NodeIndex, BlockMark>) {
    let mut changed = true;
    while changed {
        changed = false;
        block_marks.iter().for_each(|(node_index, mark)| {
            cfg.graph
                .neighbors_directed(*node_index, petgraph::Direction::Outgoing)
                .filter_map(|node_index| block_marks.get(&node_index))
                .filter(|outgoing| outgoing.in_gen.get())
                .for_each(|_| {
                    if !mark.kill && !mark.in_gen.get() {
                        mark.in_gen.set(true);
                        changed = true;
                    }
                    if !mark.out_gen.get() {
                        mark.out_gen.set(true);
                        changed = true;
                    }
                });
        });
    }
}

fn detect_unread(
    bb: &BasicBlock,
    def_key: SymbolKey,
    mark: &BlockMark,
    symbol_table: &SymbolTable,
    allocator: &Allocator,
) -> impl Iterator<Item = SymbolKey> {
    let mut set = OxcVec::with_capacity_in(1, allocator);
    bb.0.iter().for_each(|instr| match instr.name.text() {
        "local.get" => {
            if let Some(immediate) = instr.immediates.first().copied()
                && symbol_table
                    .resolved
                    .get(&immediate.into())
                    .is_some_and(|key| *key == def_key)
                && let Some(last) = set.last_mut()
            {
                *last = None;
            }
        }
        "local.set" | "local.tee" => {
            if let Some(immediate) = instr.immediates.first().copied()
                && symbol_table
                    .resolved
                    .get(&immediate.into())
                    .is_some_and(|key| *key == def_key)
            {
                set.push(Some(immediate.into()));
            }
        }
        _ => {}
    });
    if mark.out_gen.get()
        && let Some(last) = set.last_mut()
    {
        *last = None;
    }
    set.into_iter().flatten()
}

#[derive(Default)]
struct BlockMark {
    in_gen: Cell<bool>,
    out_gen: Cell<bool>,
    kill: bool,
}
impl BlockMark {
    fn new(bb: &BasicBlock, symbol_table: &SymbolTable, def_key: SymbolKey) -> Self {
        let mut r#gen = false;
        let mut kill = false;
        bb.0.iter().for_each(|instr| match instr.name.text() {
            "local.get" => {
                if let Some(immediate) = instr.immediates.first().copied()
                    && symbol_table
                        .resolved
                        .get(&immediate.into())
                        .is_some_and(|key| *key == def_key)
                    && !kill
                {
                    r#gen = true;
                }
            }
            "local.set" | "local.tee" => {
                if let Some(immediate) = instr.immediates.first().copied()
                    && symbol_table
                        .resolved
                        .get(&immediate.into())
                        .is_some_and(|key| *key == def_key)
                {
                    kill = true;
                }
            }
            _ => {}
        });
        Self {
            in_gen: Cell::new(r#gen),
            out_gen: Cell::new(r#gen),
            kill,
        }
    }
}
