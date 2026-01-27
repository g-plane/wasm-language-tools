use super::Diagnostic;
use crate::{
    binder::{Symbol, SymbolKey, SymbolKind, SymbolTable},
    cfa::{self, BasicBlock, ControlFlowGraph, FlowNode, FlowNodeKind},
    config::LintLevel,
    document::Document,
    helpers,
    idx::Idx,
};
use lspt::DiagnosticSeverity;
use oxc_allocator::{Allocator, HashMap as OxcHashMap, HashSet as OxcHashSet, Vec as OxcVec};
use petgraph::graph::NodeIndex;
use rowan::ast::{SyntaxNodePtr, support};
use std::cell::RefCell;
use wat_syntax::{SyntaxKind, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "unread";

#[expect(clippy::too_many_arguments)]
pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    db: &dyn salsa::Database,
    document: Document,
    lint_level: LintLevel,
    root: &SyntaxNode,
    symbol_table: &SymbolTable,
    node: &SyntaxNode,
    allocator: &mut Allocator,
) {
    let severity = match lint_level {
        LintLevel::Allow => return,
        LintLevel::Hint => DiagnosticSeverity::Hint,
        LintLevel::Warn => DiagnosticSeverity::Warning,
        LintLevel::Deny => DiagnosticSeverity::Error,
    };

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
                        FlowNodeKind::BasicBlock(bb) => Some((
                            node_index,
                            RefCell::new(BlockVars::new(bb, root, symbol_table, allocator)),
                        )),
                        FlowNodeKind::BlockEntry(..) | FlowNodeKind::BlockExit => Some((
                            node_index,
                            RefCell::new(BlockVars {
                                in_gens: OxcHashSet::new_in(allocator),
                                out_gens: OxcHashSet::new_in(allocator),
                                kills: OxcHashSet::new_in(allocator),
                            }),
                        )),
                        _ => None,
                    }
                }
            })
        }),
        allocator,
    );
    hydrate_block_vars(cfg, &mut block_vars);
    cfg.graph.node_indices().for_each(|node_index| {
        if let Some(FlowNode {
            kind: FlowNodeKind::BasicBlock(bb),
            unreachable: false,
        }) = cfg.graph.node_weight(node_index)
            && let Some(vars) = block_vars.get(&node_index)
            && let Ok(vars) = vars.try_borrow()
        {
            diagnostics.extend(
                detect_unread(bb, &vars, root, symbol_table, allocator)
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

    allocator.reset();
}

fn hydrate_block_vars(cfg: &ControlFlowGraph, block_vars: &mut OxcHashMap<NodeIndex, RefCell<BlockVars>>) {
    let mut changed = true;
    while changed {
        changed = false;
        block_vars.iter().for_each(|(node_index, vars)| {
            let Ok(mut vars) = vars.try_borrow_mut() else {
                return;
            };
            cfg.graph
                .neighbors_directed(*node_index, petgraph::Direction::Outgoing)
                .filter_map(|node_index| block_vars.get(&node_index))
                .filter_map(|outgoing| outgoing.try_borrow().ok())
                .for_each(|outgoing| {
                    outgoing.in_gens.iter().for_each(|idx| {
                        if !vars.kills.contains(idx) {
                            changed |= vars.in_gens.insert(*idx);
                        }
                        changed |= vars.out_gens.insert(*idx);
                    });
                });
        });
    }
}

fn detect_unread(
    bb: &BasicBlock,
    vars: &BlockVars,
    root: &SyntaxNode,
    symbol_table: &SymbolTable,
    allocator: &Allocator,
) -> impl Iterator<Item = SymbolKey> {
    let mut sets = OxcHashMap::<_, OxcVec<_>>::with_capacity_in(vars.kills.len(), allocator);
    bb.instrs(root).for_each(|instr| {
        match support::token(&instr, SyntaxKind::INSTR_NAME)
            .as_ref()
            .map(|token| token.text())
        {
            Some("local.get") => {
                if let Some(immediate) = instr.first_child_by_kind(&|kind| kind == SyntaxKind::IMMEDIATE)
                    && let Some(Symbol {
                        idx: Idx { num: Some(num), .. },
                        kind: SymbolKind::Local,
                        ..
                    }) = symbol_table.find_def(SymbolKey::new(&immediate))
                    && let Some(last) = sets.get_mut(num).and_then(|sets| sets.last_mut())
                {
                    *last = None;
                }
            }
            Some("local.set" | "local.tee") => {
                if let Some(immediate) = instr.first_child_by_kind(&|kind| kind == SyntaxKind::IMMEDIATE)
                    && let Some(Symbol {
                        idx: Idx { num: Some(num), .. },
                        kind: SymbolKind::Local,
                        ..
                    }) = symbol_table.find_def(SymbolKey::new(&immediate))
                {
                    sets.entry(*num)
                        .or_insert_with(|| OxcVec::with_capacity_in(1, allocator))
                        .push(Some(SymbolKey::new(&immediate)));
                }
            }
            _ => {}
        }
    });
    vars.out_gens.iter().for_each(|idx| {
        if let Some(last) = sets.get_mut(idx).and_then(|sets| sets.last_mut()) {
            *last = None;
        }
    });
    sets.into_values().flatten().flatten()
}

struct BlockVars<'alloc> {
    in_gens: OxcHashSet<'alloc, u32>,
    out_gens: OxcHashSet<'alloc, u32>,
    kills: OxcHashSet<'alloc, u32>,
}
impl<'alloc> BlockVars<'alloc> {
    fn new(bb: &BasicBlock, root: &SyntaxNode, symbol_table: &SymbolTable, allocator: &'alloc Allocator) -> Self {
        let mut gens = OxcHashSet::new_in(allocator);
        let mut kills = OxcHashSet::new_in(allocator);
        bb.instrs(root).for_each(|instr| {
            match support::token(&instr, SyntaxKind::INSTR_NAME)
                .as_ref()
                .map(|token| token.text())
            {
                Some("local.get") => {
                    if let Some(idx) = helpers::locals::find_local_def_idx(&instr, symbol_table)
                        && !kills.contains(&idx)
                    {
                        gens.insert(idx);
                    }
                }
                Some("local.set" | "local.tee") => {
                    if let Some(idx) = helpers::locals::find_local_def_idx(&instr, symbol_table) {
                        kills.insert(idx);
                    }
                }
                _ => {}
            }
        });
        Self {
            in_gens: OxcHashSet::from_iter_in(gens.iter().copied(), allocator),
            out_gens: gens,
            kills,
        }
    }
}
