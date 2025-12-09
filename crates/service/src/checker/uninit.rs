use crate::{
    LanguageService,
    binder::{Symbol, SymbolKey, SymbolKind, SymbolTable},
    cfa::{self, BasicBlock, ControlFlowGraph, FlowNode, FlowNodeKind},
    document::Document,
    helpers,
    idx::Idx,
    types_analyzer,
};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticSeverity, Union2};
use petgraph::graph::NodeIndex;
use rowan::ast::{SyntaxNodePtr, support};
use rustc_hash::{FxHashMap, FxHashSet};
use std::cell::RefCell;
use wat_syntax::{SyntaxKind, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "uninit";

pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    service: &LanguageService,
    document: Document,
    line_index: &LineIndex,
    root: &SyntaxNode,
    symbol_table: &SymbolTable,
    node: &SyntaxNode,
) {
    // avoid expensive analysis if there are no locals
    let func_key = SymbolKey::new(node);
    if !symbol_table
        .symbols
        .values()
        .any(|symbol| symbol.region == func_key && symbol.kind == SymbolKind::Local)
    {
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
                        FlowNodeKind::BlockEntry(..) | FlowNodeKind::BlockExit => Some((
                            node_index,
                            RefCell::new(BlockVars {
                                in_set: FxHashSet::default(),
                                out_set: FxHashSet::default(),
                            }),
                        )),
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
            && let Ok(mut vars) = vars.try_borrow_mut()
        {
            diagnostics.extend(
                detect_uninit(bb, &mut vars, service, document, root, symbol_table)
                    .filter_map(|immediate| symbol_table.symbols.get(&SymbolKey::new(&immediate)))
                    .map(|symbol| Diagnostic {
                        range: helpers::rowan_range_to_lsp_range(
                            line_index,
                            symbol.key.text_range(),
                        ),
                        severity: Some(DiagnosticSeverity::Error),
                        source: Some("wat".into()),
                        code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                        message: format!(
                            "local `{}` is used before being initialized",
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
            let incomings = cfg
                .graph
                .neighbors_directed(*node_index, petgraph::Direction::Incoming)
                .filter_map(|node_index| block_vars.get(&node_index))
                .filter_map(|vars| vars.try_borrow().ok())
                .collect::<Vec<_>>();
            if let Some((first, rest)) = incomings.split_first() {
                first
                    .out_set
                    .iter()
                    .filter(|idx| rest.iter().all(|other| other.out_set.contains(idx)))
                    .for_each(|idx| {
                        changed |= vars.in_set.insert(*idx) || vars.out_set.insert(*idx);
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
    root: &SyntaxNode,
    symbol_table: &SymbolTable,
) -> impl Iterator<Item = SyntaxNode> {
    bb.instrs(root).filter_map(move |instr| {
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
                        green,
                        ..
                    }) = symbol_table.find_def(SymbolKey::new(&immediate))
                    && types_analyzer::extract_type(db, document, green.clone())
                        .is_some_and(|ty| !ty.defaultable())
                    && !vars.in_set.contains(num)
                {
                    Some(immediate)
                } else {
                    None
                }
            }
            Some("local.set" | "local.tee") => {
                if let Some(idx) = find_local_def_idx(&instr, symbol_table) {
                    vars.in_set.insert(idx);
                }
                None
            }
            _ => None,
        }
    })
}

fn find_local_def_idx(instr: &SyntaxNode, symbol_table: &SymbolTable) -> Option<u32> {
    instr
        .first_child_by_kind(&|kind| kind == SyntaxKind::IMMEDIATE)
        .and_then(|immediate| symbol_table.find_def(SymbolKey::new(&immediate)))
        .and_then(|def| def.idx.num)
}

struct BlockVars {
    in_set: FxHashSet<u32>,
    out_set: FxHashSet<u32>,
}
impl BlockVars {
    fn new(bb: &BasicBlock, root: &SyntaxNode, symbol_table: &SymbolTable) -> Self {
        let out_set = bb
            .instrs(root)
            .filter_map(|instr| {
                support::token(&instr, SyntaxKind::INSTR_NAME)
                    .filter(|token| matches!(token.text(), "local.set" | "local.tee"))
                    .and_then(|_| find_local_def_idx(&instr, symbol_table))
            })
            .collect();
        Self {
            in_set: FxHashSet::default(),
            out_set,
        }
    }
}
