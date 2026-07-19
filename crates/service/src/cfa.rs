use crate::{
    binder::{SymbolKey, SymbolTable},
    document::Document,
    idx::InternIdent,
};
use smallvec::SmallVec;
use std::fmt::Write;
use wat_syntax::{
    GreenToken, SyntaxKind, SyntaxNode, SyntaxNodePtr, TextRange,
    ast::{AstNode, BlockInstr, Cat, Instr, support},
};

#[salsa::tracked(returns(ref))]
pub fn analyze(db: &dyn salsa::Database, document: Document, ptr: SyntaxNodePtr) -> ControlFlowGraph {
    let root = SyntaxNode::new_root(document.root(db));
    Builder::new(db, document).build(ptr.to_node(&root).expect("invalid ptr in control flow analysis"))
}

struct Builder<'db> {
    db: &'db dyn salsa::Database,
    symbol_table: &'db SymbolTable<'db>,
    graph: ControlFlowGraph,
    block_stack: Vec<(FlowNodeId, Option<InternIdent<'db>>)>,
    current: Option<FlowNodeId>,
    bb_instrs: Vec<BasicBlockInstr>,
    unreachable: bool,
}
impl<'db> Builder<'db> {
    fn new(db: &'db dyn salsa::Database, document: Document) -> Self {
        Self {
            db,
            symbol_table: SymbolTable::of(db, document),
            graph: ControlFlowGraph {
                nodes: Vec::with_capacity(16),
            },
            block_stack: Vec::new(),
            current: None,
            bb_instrs: Vec::with_capacity(2),
            unreachable: false,
        }
    }

    fn build(mut self, node: SyntaxNode) -> ControlFlowGraph {
        let entry = self.graph.add_node(FlowNodeKind::Entry, false);
        self.current = Some(entry);
        let exit = self.graph.add_node(FlowNodeKind::Exit, false);

        self.block_stack.push((exit, None));
        self.visit_block_like(&node, exit);
        self.finish_exit(exit);

        self.graph
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
                            self.graph.add_edge(bb, *exit);
                        }
                        true
                    }
                    "br" | "br_table" => {
                        if let Some(bb) = self.add_basic_block() {
                            for immediate in plain.immediates() {
                                if let Some(target) = self.find_jump_target(immediate) {
                                    self.graph.add_edge(bb, target);
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
                            self.graph.add_edge(bb, target);
                        }
                        false
                    }
                    "resume" | "resume_throw" | "resume_throw_ref" => {
                        if let Some(bb) = self.add_basic_block() {
                            for index in plain
                                .immediates()
                                .filter_map(|immediate| immediate.on_clause())
                                .filter_map(|on_clause| on_clause.label_index())
                            {
                                if let Some(target) = self.find_jump_target(index) {
                                    self.graph.add_edge(bb, target);
                                }
                            }
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
                let block_entry = self.graph.add_node(
                    FlowNodeKind::BlockEntry(SyntaxNodePtr::new(block.syntax())),
                    self.unreachable,
                );
                self.connect_current_to(block_entry);
                self.current = Some(block_entry);

                let block_exit = self.graph.add_node(FlowNodeKind::BlockExit, false);
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
                            self.graph.add_edge(condition, block_exit);
                        }
                    }
                    BlockInstr::TryTable(block_try_table) => {
                        for index in block_try_table.catches().filter_map(|cat| match cat {
                            Cat::Catch(catch) => catch.label_index(),
                            Cat::CatchAll(catch_all) => catch_all.label_index(),
                        }) {
                            if let Some(target) = self.find_jump_target(index) {
                                self.graph.add_edge(block_entry, target);
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

    fn visit_block_like(&mut self, node: &SyntaxNode, exit: FlowNodeId) {
        self.visit_instrs(node);
        self.add_basic_block();
        self.connect_current_to(exit);
    }

    fn add_basic_block(&mut self) -> Option<FlowNodeId> {
        if self.bb_instrs.is_empty() {
            None
        } else {
            let bb = self.graph.add_node(
                FlowNodeKind::BasicBlock(BasicBlock(self.bb_instrs.drain(..).collect())),
                self.unreachable,
            );
            self.connect_current_to(bb);
            self.current = Some(bb);
            Some(bb)
        }
    }

    fn connect_current_to(&mut self, node_id: FlowNodeId) {
        if let Some(current) = self.current {
            self.graph.add_edge(current, node_id);
        }
    }

    fn find_jump_target<'a, N: AstNode<'a>>(&self, node: N) -> Option<FlowNodeId> {
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

    fn finish_exit(&mut self, exit: FlowNodeId) {
        // If all basic blocks connecting to this exit are unreachable,
        // or no basic blocks connect to this exit, next will be unreachable.
        self.unreachable = self.graph.get_node(exit).is_some_and(|flow_node| {
            flow_node.incomings.iter().all(|node_id| {
                matches!(
                    self.graph.get_node(*node_id),
                    Some(FlowNode { unreachable: true, .. }) | None
                )
            })
        });
        if let Some(flow_node) = self.graph.get_node_mut(exit) {
            flow_node.unreachable = self.unreachable;
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FlowNodeId(u32);

#[derive(Clone, PartialEq, Eq)]
pub struct FlowNode {
    pub kind: FlowNodeKind,
    pub unreachable: bool,
    pub incomings: SmallVec<[FlowNodeId; 4]>,
    pub outgoings: SmallVec<[FlowNodeId; 4]>,
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
        self.text_range()
            .is_some_and(|range| range.start() < end && end <= range.end())
    }
    fn text_range(&self) -> Option<TextRange> {
        self.0.first().map(|first| {
            let first = first.ptr.text_range();
            let (start, end) = self.0.iter().fold((first.start(), first.end()), |(start, end), instr| {
                let range = instr.ptr.text_range();
                (start.min(range.start()), end.max(range.end()))
            });
            TextRange::new(start, end)
        })
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BasicBlockInstr {
    pub ptr: SyntaxNodePtr,
    pub name: GreenToken,
    pub immediates: Vec<SyntaxNodePtr>,
}

#[derive(Clone, PartialEq, Eq)]
pub struct ControlFlowGraph {
    nodes: Vec<FlowNode>,
}

impl ControlFlowGraph {
    #[inline]
    pub fn nodes(&self) -> &[FlowNode] {
        &self.nodes
    }
    #[inline]
    pub fn nodes_with_ids(&self) -> impl Iterator<Item = (&FlowNode, FlowNodeId)> {
        self.nodes
            .iter()
            .enumerate()
            .map(|(i, flow_node)| (flow_node, FlowNodeId(i as u32)))
    }

    #[inline]
    pub fn get_node(&self, id: FlowNodeId) -> Option<&FlowNode> {
        self.nodes.get(id.0 as usize)
    }
    #[inline]
    fn get_node_mut(&mut self, id: FlowNodeId) -> Option<&mut FlowNode> {
        self.nodes.get_mut(id.0 as usize)
    }

    fn add_node(&mut self, kind: FlowNodeKind, unreachable: bool) -> FlowNodeId {
        let id = FlowNodeId(self.nodes.len() as u32);
        self.nodes.push(FlowNode {
            kind,
            unreachable,
            incomings: SmallVec::with_capacity(4),
            outgoings: SmallVec::with_capacity(4),
        });
        id
    }

    fn add_edge(&mut self, from: FlowNodeId, to: FlowNodeId) {
        debug_assert_ne!(from, to);
        if let Some(flow_node) = self.nodes.get_mut(from.0 as usize) {
            flow_node.outgoings.push(to);
        }
        if let Some(flow_node) = self.nodes.get_mut(to.0 as usize) {
            flow_node.incomings.push(from);
        }
    }

    /// Generate a DOT representation for Graphviz.
    pub fn generate_dot(&self) -> String {
        let mut output = String::from("digraph {\n");
        self.nodes.iter().enumerate().for_each(|(i, node)| {
            match &node.kind {
                FlowNodeKind::Entry => {
                    let _ = write!(&mut output, "  {i} [label=\"Entry\"]");
                }
                FlowNodeKind::Exit => {
                    let _ = write!(&mut output, "  {i} [label=\"Exit\"]");
                }
                FlowNodeKind::BasicBlock(bb) => {
                    if let Some(range) = bb.text_range() {
                        let _ = write!(&mut output, "  {i} [label=\"BasicBlock@{range:?}\"]");
                    } else {
                        let _ = write!(&mut output, "  {i} [label=\"BasicBlock\"]");
                    }
                }
                FlowNodeKind::BlockEntry(ptr) => {
                    let _ = write!(
                        &mut output,
                        "  {i} [label=\"BlockEntry:{:?}@{:?}\"]",
                        ptr.kind(),
                        ptr.text_range(),
                    );
                }
                FlowNodeKind::BlockExit => {
                    let _ = write!(&mut output, "  {i} [label=\"BlockExit\"]");
                }
            }
            if node.unreachable {
                output.push_str(" [class=\"unreachable\"]");
            }
            output.push('\n');
        });
        output.push('\n');
        self.nodes.iter().enumerate().for_each(|(i, node)| {
            let class = if node.unreachable {
                " [class=\"unreachable\"]"
            } else {
                ""
            };
            node.outgoings.iter().for_each(|outgoing| {
                let _ = writeln!(&mut output, "  {i} -> {}{class}", outgoing.0);
            });
        });
        output.push('}');
        output
    }
}
