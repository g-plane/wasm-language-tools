use crate::{
    LanguageService,
    binder::{Symbol, SymbolKey, SymbolKind, SymbolTable},
    cfa::{self, BasicBlock, ControlFlowGraph, FlowNode, FlowNodeKind},
    config::LintLevel,
    document::Document,
    helpers,
    idx::Idx,
};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticSeverity, Union2};
use petgraph::graph::NodeIndex;
use rowan::ast::{SyntaxNodePtr, support};
use rustc_hash::{FxBuildHasher, FxHashMap, FxHashSet};
use std::{cell::RefCell, rc::Rc};
use wat_syntax::{SyntaxKind, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "unread";

#[expect(clippy::too_many_arguments)]
pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    service: &LanguageService,
    document: Document,
    lint_level: LintLevel,
    line_index: &LineIndex,
    root: &SyntaxNode,
    symbol_table: &SymbolTable,
    node: &SyntaxNode,
) {
    let severity = match lint_level {
        LintLevel::Allow => return,
        LintLevel::Hint => DiagnosticSeverity::Hint,
        LintLevel::Warn => DiagnosticSeverity::Warning,
        LintLevel::Deny => DiagnosticSeverity::Error,
    };

    // avoid expensive analysis if there are no locals
    if !helpers::locals::has_locals(service, document, SymbolKey::new(node)) {
        return;
    }

    let cfg = cfa::analyze(service, document, SyntaxNodePtr::new(node));
    let mut block_vars = cfg
        .graph
        .node_indices()
        .filter_map(|node_index| {
            cfg.graph.node_weight(node_index).and_then(|node| {
                if node.unreachable {
                    None
                } else {
                    match &node.kind {
                        FlowNodeKind::BasicBlock(bb) => Some((
                            node_index,
                            RefCell::new(BlockVars::new(bb, root, symbol_table)),
                        )),
                        FlowNodeKind::BlockEntry(..) | FlowNodeKind::BlockExit => {
                            Some((node_index, RefCell::new(BlockVars::default())))
                        }
                        _ => None,
                    }
                }
            })
        })
        .collect::<FxHashMap<_, _>>();
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
                detect_unread(bb, &vars, root, symbol_table)
                    .filter_map(|immediate| symbol_table.symbols.get(&SymbolKey::new(&immediate)))
                    .map(|symbol| Diagnostic {
                        range: helpers::rowan_range_to_lsp_range(
                            line_index,
                            symbol.key.text_range(),
                        ),
                        severity: Some(severity),
                        source: Some("wat".into()),
                        code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                        message: format!(
                            "local `{}` is set but never read",
                            symbol.idx.render(service),
                        ),
                        ..Default::default()
                    }),
            );
        }
    });
}

fn hydrate_block_vars(
    cfg: &ControlFlowGraph,
    block_vars: &mut FxHashMap<NodeIndex, RefCell<BlockVars>>,
) {
    let mut changed = true;
    while changed {
        changed = false;
        block_vars.iter().for_each(|(node_index, vars)| {
            let Ok(mut vars) = vars.try_borrow_mut() else {
                return;
            };
            let kills = Rc::clone(&vars.kills);
            cfg.graph
                .neighbors_directed(*node_index, petgraph::Direction::Outgoing)
                .filter_map(|node_index| block_vars.get(&node_index))
                .filter_map(|outgoing| outgoing.try_borrow().ok())
                .for_each(|outgoing| {
                    outgoing.in_gens.iter().for_each(|idx| {
                        if !kills.contains(idx) {
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
) -> impl Iterator<Item = SyntaxNode> {
    let mut sets =
        FxHashMap::<_, Vec<_>>::with_capacity_and_hasher(vars.kills.len(), FxBuildHasher);
    bb.instrs(root).for_each(|instr| {
        match support::token(&instr, SyntaxKind::INSTR_NAME)
            .as_ref()
            .map(|token| token.text())
        {
            Some("local.get") => {
                if let Some(immediate) =
                    instr.first_child_by_kind(&|kind| kind == SyntaxKind::IMMEDIATE)
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
                if let Some(immediate) =
                    instr.first_child_by_kind(&|kind| kind == SyntaxKind::IMMEDIATE)
                    && let Some(Symbol {
                        idx: Idx { num: Some(num), .. },
                        kind: SymbolKind::Local,
                        ..
                    }) = symbol_table.find_def(SymbolKey::new(&immediate))
                {
                    sets.entry(*num)
                        .or_insert_with(|| Vec::with_capacity(1))
                        .push(Some(immediate));
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

#[derive(Default)]
struct BlockVars {
    in_gens: FxHashSet<u32>,
    out_gens: FxHashSet<u32>,
    kills: Rc<FxHashSet<u32>>,
}
impl BlockVars {
    fn new(bb: &BasicBlock, root: &SyntaxNode, symbol_table: &SymbolTable) -> Self {
        let mut gens = FxHashSet::default();
        let mut kills = FxHashSet::default();
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
            in_gens: gens.clone(),
            out_gens: gens,
            kills: Rc::new(kills),
        }
    }
}
