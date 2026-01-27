use super::Diagnostic;
use crate::{
    binder::{Symbol, SymbolKey, SymbolKind, SymbolTable},
    cfa::{self, BasicBlock, ControlFlowGraph, FlowNode, FlowNodeKind},
    document::Document,
    helpers,
    idx::Idx,
    types_analyzer,
};
use oxc_allocator::{Allocator, HashMap as OxcHashMap, HashSet as OxcHashSet, Vec as OxcVec};
use petgraph::graph::NodeIndex;
use rowan::ast::SyntaxNodePtr;
use wat_syntax::SyntaxNode;

const DIAGNOSTIC_CODE: &str = "uninit";

pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    db: &dyn salsa::Database,
    document: Document,
    symbol_table: &SymbolTable,
    node: &SyntaxNode,
    allocator: &mut Allocator,
) {
    // avoid expensive analysis if there are no locals
    if !helpers::locals::has_locals(db, document, SymbolKey::new(node)) {
        return;
    }

    let cfg = cfa::analyze(db, document, SyntaxNodePtr::new(node));
    let mut block_vars = OxcHashMap::from_iter_in(
        cfg.graph.node_indices().filter_map(|node_index| {
            cfg.graph.node_weight(node_index).and_then(|node| {
                if node.unreachable {
                    None
                } else {
                    match &node.kind {
                        FlowNodeKind::BasicBlock(bb) => Some((node_index, BlockVars::new(bb, symbol_table, allocator))),
                        FlowNodeKind::BlockEntry(..) | FlowNodeKind::BlockExit => Some((
                            node_index,
                            BlockVars {
                                in_set: OxcHashSet::new_in(allocator),
                                out_set: OxcHashSet::new_in(allocator),
                            },
                        )),
                        _ => None,
                    }
                }
            })
        }),
        allocator,
    );
    hydrate_block_vars(cfg, &mut block_vars, allocator);
    cfg.graph.node_indices().for_each(|node_index| {
        if let Some(FlowNode {
            kind: FlowNodeKind::BasicBlock(bb),
            unreachable: false,
        }) = cfg.graph.node_weight(node_index)
            && let Some(vars) = block_vars.get_mut(&node_index)
        {
            diagnostics.extend(
                detect_uninit(bb, vars, db, document, symbol_table)
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

    allocator.reset();
}

fn hydrate_block_vars(
    cfg: &ControlFlowGraph,
    block_vars: &mut OxcHashMap<NodeIndex, BlockVars>,
    allocator: &Allocator,
) {
    let mut changed = true;
    while changed {
        changed = false;
        cfg.graph.node_indices().for_each(|node_index| {
            if !block_vars.contains_key(&node_index) {
                return;
            }
            let indices = OxcVec::from_iter_in(
                OxcVec::from_iter_in(
                    cfg.graph
                        .neighbors_directed(node_index, petgraph::Direction::Incoming)
                        .filter_map(|node_index| block_vars.get(&node_index)),
                    allocator,
                )
                .split_first()
                .into_iter()
                .flat_map(|(first, rest)| {
                    first
                        .out_set
                        .iter()
                        .filter(|idx| rest.iter().all(|other| other.out_set.contains(*idx))) // intersection
                        .copied()
                }),
                allocator,
            );
            if let Some(vars) = block_vars.get_mut(&node_index) {
                indices.into_iter().for_each(|idx| {
                    changed |= vars.in_set.insert(idx) || vars.out_set.insert(idx);
                });
            }
        });
    }
}

fn detect_uninit(
    bb: &BasicBlock,
    vars: &mut BlockVars,
    db: &dyn salsa::Database,
    document: Document,
    symbol_table: &SymbolTable,
) -> impl Iterator<Item = SymbolKey> {
    bb.0.iter().filter_map(move |instr| match instr.name.text() {
        "local.get" => {
            if let Some(immediate) = instr.immediates.first().copied()
                && let Some(Symbol {
                    idx: Idx { num: Some(num), .. },
                    kind: SymbolKind::Local,
                    green,
                    ..
                }) = symbol_table.find_def(immediate.into())
                && types_analyzer::extract_type(db, document, green.clone()).is_some_and(|ty| !ty.defaultable())
                && !vars.in_set.contains(num)
            {
                Some(immediate.into())
            } else {
                None
            }
        }
        "local.set" | "local.tee" => {
            if let Some(immediate) = instr.immediates.first().copied()
                && let Some(idx) = symbol_table
                    .find_def(immediate.into())
                    .and_then(|symbol| symbol.idx.num)
            {
                vars.in_set.insert(idx);
            }
            None
        }
        _ => None,
    })
}

struct BlockVars<'alloc> {
    in_set: OxcHashSet<'alloc, u32>,
    out_set: OxcHashSet<'alloc, u32>,
}
impl<'alloc> BlockVars<'alloc> {
    fn new(bb: &BasicBlock, symbol_table: &SymbolTable, allocator: &'alloc Allocator) -> Self {
        let out_set = OxcHashSet::from_iter_in(
            bb.0.iter()
                .filter(|instr| matches!(instr.name.text(), "local.set" | "local.tee"))
                .filter_map(|instr| {
                    instr
                        .immediates
                        .first()
                        .copied()
                        .and_then(|immediate| symbol_table.find_def(immediate.into()))
                        .and_then(|symbol| symbol.idx.num)
                }),
            allocator,
        );
        Self {
            in_set: OxcHashSet::new_in(allocator),
            out_set,
        }
    }
}
