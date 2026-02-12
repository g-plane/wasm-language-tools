use crate::{
    binder::{SymbolKey, SymbolTable},
    document::Document,
    idx::InternIdent,
};
use petgraph::graph::{Graph, NodeIndex};
use wat_syntax::{
    GreenToken, SyntaxKind, SyntaxNode, SyntaxNodePtr,
    ast::{AstNode, BlockInstr, Cat, Instr, support},
};

#[salsa::tracked(returns(ref))]
pub fn analyze(db: &dyn salsa::Database, document: Document, ptr: SyntaxNodePtr) -> ControlFlowGraph {
    let root = document.root_tree(db);
    Builder::new(db, document).build(ptr.to_node(&root))
}

struct Builder<'db> {
    db: &'db dyn salsa::Database,
    symbol_table: &'db SymbolTable<'db>,
    graph: Graph<FlowNode, ()>,
    block_stack: Vec<(NodeIndex, Option<InternIdent<'db>>)>,
    current: Option<NodeIndex>,
    bb_instrs: Vec<BasicBlockInstr>,
    unreachable: bool,
}
impl<'db> Builder<'db> {
    fn new(db: &'db dyn salsa::Database, document: Document) -> Self {
        Self {
            db,
            symbol_table: SymbolTable::of(db, document),
            graph: Graph::new(),
            block_stack: Vec::new(),
            current: None,
            bb_instrs: Vec::with_capacity(2),
            unreachable: false,
        }
    }

    fn build(mut self, node: SyntaxNode) -> ControlFlowGraph {
        let entry = self.graph.add_node(FlowNode {
            kind: FlowNodeKind::Entry,
            unreachable: false,
        });
        self.current = Some(entry);
        let exit = self.graph.add_node(FlowNode {
            kind: FlowNodeKind::Exit,
            unreachable: false,
        });

        self.block_stack.push((exit, None));
        self.visit_block_like(&node, exit);
        self.finish_exit(exit);

        ControlFlowGraph {
            graph: self.graph,
            entry,
        }
    }

    fn visit_instrs(&mut self, node: &SyntaxNode) {
        support::children::<Instr>(node).for_each(|instr| match instr {
            Instr::Plain(plain) => {
                self.visit_instrs(plain.syntax());

                let Some(instr_name) = plain.instr_name() else {
                    return;
                };
                let instr_name = instr_name.green();

                self.bb_instrs.push(BasicBlockInstr {
                    ptr: SyntaxNodePtr::new(plain.syntax()),
                    name: instr_name.to_owned(),
                    immediates: plain
                        .immediates()
                        .map(|immediate| SyntaxNodePtr::new(immediate.syntax()))
                        .collect(),
                });

                let unreachable = match instr_name.text() {
                    "unreachable" | "throw" | "throw_ref" => {
                        self.add_basic_block();
                        true
                    }
                    "return" | "return_call" | "return_call_indirect" | "return_call_ref" => {
                        if let Some((bb, (exit, _))) = self.add_basic_block().zip(self.block_stack.first()) {
                            self.graph.add_edge(bb, *exit, ());
                        }
                        true
                    }
                    "br" | "br_table" => {
                        if let Some(bb) = self.add_basic_block() {
                            for immediate in plain.immediates() {
                                if let Some(target) = self.find_jump_target(immediate) {
                                    self.graph.add_edge(bb, target, ());
                                }
                            }
                        }
                        true
                    }
                    "br_if" | "br_on_null" | "br_on_non_null" | "br_on_cast" | "br_on_cast_fail" => {
                        let target = plain
                            .immediates()
                            .next()
                            .and_then(|immediate| self.find_jump_target(immediate));
                        if let Some((bb, target)) = self.add_basic_block().zip(target) {
                            self.graph.add_edge(bb, target, ());
                        }
                        false
                    }
                    _ => false,
                };

                if unreachable {
                    // clear current flow node to avoid connecting to next flow node
                    self.current = None;
                }
                self.unreachable |= unreachable;
            }
            Instr::Block(block) => {
                self.add_basic_block();
                let block_entry = self.graph.add_node(FlowNode {
                    kind: FlowNodeKind::BlockEntry(SyntaxNodePtr::new(block.syntax())),
                    unreachable: self.unreachable,
                });
                self.connect_current_to(block_entry);
                self.current = Some(block_entry);

                let block_exit = self.graph.add_node(FlowNode {
                    kind: FlowNodeKind::BlockExit,
                    unreachable: false,
                });
                let ident = support::token(block.syntax(), SyntaxKind::IDENT)
                    .map(|token| InternIdent::new(self.db, token.text()));
                match block {
                    BlockInstr::Block(block_block) => {
                        self.block_stack.push((block_exit, ident));
                        self.visit_block_like(block_block.syntax(), block_exit);
                    }
                    BlockInstr::Loop(block_loop) => {
                        self.block_stack.push((block_entry, ident));
                        self.visit_block_like(block_loop.syntax(), block_exit);
                    }
                    BlockInstr::If(block_if) => {
                        self.visit_instrs(block_if.syntax());
                        self.add_basic_block();

                        self.block_stack.push((block_exit, ident));
                        let condition = self.current;
                        let saved_unreachable = self.unreachable;

                        if let Some(then_block) = block_if.then_block() {
                            self.visit_block_like(then_block.syntax(), block_exit);
                            self.current = condition;
                            self.unreachable = saved_unreachable;
                        }

                        if let Some(else_block) = block_if.else_block() {
                            self.visit_block_like(else_block.syntax(), block_exit);
                        } else if let Some(condition) = condition {
                            self.graph.add_edge(condition, block_exit, ());
                        }
                    }
                    BlockInstr::TryTable(block_try_table) => {
                        for index in block_try_table.catches().filter_map(|cat| match cat {
                            Cat::Catch(catch) => catch.label_index(),
                            Cat::CatchAll(catch_all) => catch_all.label_index(),
                        }) {
                            if let Some(target) = self.find_jump_target(index) {
                                self.graph.add_edge(block_entry, target, ());
                            }
                        }
                        self.block_stack.push((block_exit, ident));
                        self.visit_block_like(block_try_table.syntax(), block_exit);
                    }
                }

                self.finish_exit(block_exit);
                self.current = Some(block_exit);
                self.block_stack.pop();
            }
        });
    }

    fn visit_block_like(&mut self, node: &SyntaxNode, exit: NodeIndex) {
        self.visit_instrs(node);
        self.add_basic_block();
        self.connect_current_to(exit);
    }

    fn add_basic_block(&mut self) -> Option<NodeIndex> {
        if self.bb_instrs.is_empty() {
            None
        } else {
            let bb = self.graph.add_node(FlowNode {
                kind: FlowNodeKind::BasicBlock(BasicBlock(self.bb_instrs.drain(..).collect())),
                unreachable: self.unreachable,
            });
            self.connect_current_to(bb);
            self.current = Some(bb);
            Some(bb)
        }
    }

    fn connect_current_to(&mut self, node_index: NodeIndex) {
        if let Some(current) = self.current {
            self.graph.add_edge(current, node_index, ());
        }
    }

    fn find_jump_target<N: AstNode>(&self, node: N) -> Option<NodeIndex> {
        self.symbol_table
            .symbols
            .get(&SymbolKey::new(node.syntax()))
            .and_then(|symbol| {
                if let Some(num) = symbol.idx.num {
                    let num = num as usize;
                    let total = self.block_stack.len();
                    if num < total {
                        self.block_stack.get(total - 1 - num)
                    } else {
                        None
                    }
                } else {
                    self.block_stack.iter().rev().find(|(_, it)| *it == symbol.idx.name)
                }
            })
            .map(|(target, _)| *target)
    }

    fn finish_exit(&mut self, exit: NodeIndex) {
        // If all basic blocks connecting to this exit are unreachable,
        // or no basic blocks connect to this exit, next will be unreachable.
        self.unreachable = self
            .graph
            .neighbors_directed(exit, petgraph::Direction::Incoming)
            .all(|node_index| {
                matches!(
                    self.graph.node_weight(node_index),
                    Some(FlowNode { unreachable: true, .. }) | None
                )
            });
        if let Some(flow_node) = self.graph.node_weight_mut(exit) {
            flow_node.unreachable = self.unreachable;
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowNode {
    pub kind: FlowNodeKind,
    pub unreachable: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FlowNodeKind {
    Entry,
    Exit,
    BasicBlock(BasicBlock),
    BlockEntry(SyntaxNodePtr),
    BlockExit,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BasicBlock(pub Vec<BasicBlockInstr>);
impl BasicBlock {
    pub fn contains_instr(&self, node: &SyntaxNode) -> bool {
        let end = node.text_range().end();
        self.0
            .first()
            .zip(self.0.last())
            .is_some_and(|(first, last)| first.ptr.text_range().start() < end && end <= last.ptr.text_range().end())
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BasicBlockInstr {
    pub ptr: SyntaxNodePtr,
    pub name: GreenToken,
    pub immediates: Vec<SyntaxNodePtr>,
}

#[derive(Clone)]
pub struct ControlFlowGraph {
    pub graph: Graph<FlowNode, ()>,
    pub entry: NodeIndex,
}
impl PartialEq for ControlFlowGraph {
    fn eq(&self, other: &Self) -> bool {
        self.entry == other.entry
            && self.graph.raw_nodes().iter().map(|node| &node.weight).eq(other
                .graph
                .raw_nodes()
                .iter()
                .map(|node| &node.weight))
            && self
                .graph
                .raw_edges()
                .iter()
                .map(|edge| (edge.source(), edge.target(), &edge.weight))
                .eq(other
                    .graph
                    .raw_edges()
                    .iter()
                    .map(|edge| (edge.source(), edge.target(), &edge.weight)))
    }
}
