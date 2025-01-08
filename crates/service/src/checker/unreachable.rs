use crate::{
    binder::{SymbolItemKey, SymbolTable},
    helpers, LintLevel, ServiceConfig,
};
use line_index::LineIndex;
use lsp_types::{Diagnostic, DiagnosticSeverity, DiagnosticTag, NumberOrString};
use rowan::{ast::AstNode, TextRange};
use rustc_hash::FxHashSet;
use std::ops::ControlFlow;
use wat_syntax::{
    ast::{BlockInstr, Instr},
    SyntaxKind, SyntaxNode, SyntaxToken,
};

const DIAGNOSTIC_CODE: &str = "unreachable";

pub fn check(
    diags: &mut Vec<Diagnostic>,
    config: &ServiceConfig,
    line_index: &LineIndex,
    symbol_table: &SymbolTable,
    root: &SyntaxNode,
    node: &SyntaxNode,
) {
    Checker {
        diags,
        line_index,
        symbol_table,
        root,
        severity: match config.lint.unreachable {
            LintLevel::Allow => return,
            LintLevel::Warn => DiagnosticSeverity::WARNING,
            LintLevel::Deny => DiagnosticSeverity::ERROR,
        },
        jumps: FxHashSet::default(),
        last_reported: TextRange::default(),
    }
    .check_block_like(node);
}

struct Checker<'a> {
    diags: &'a mut Vec<Diagnostic>,
    line_index: &'a LineIndex,
    symbol_table: &'a SymbolTable,
    root: &'a SyntaxNode,
    severity: DiagnosticSeverity,
    jumps: FxHashSet<SyntaxNode>,
    last_reported: TextRange,
}
impl Checker<'_> {
    fn check_block_like(&mut self, node: &SyntaxNode) -> bool {
        let mut unreachable = false;
        let result = node
            .children()
            .filter_map(Instr::cast)
            .try_for_each(|instr| self.check_instr(&instr, node, &mut unreachable));
        // When the last instr produces never, the `end` keyword should be marked unreachable.
        // But for then-block and else-block, they don't have `end` keyword.
        // (That `end` keyword comes from their children.)
        if !matches!(
            node.kind(),
            SyntaxKind::BLOCK_IF_THEN | SyntaxKind::BLOCK_IF_ELSE
        ) {
            match (result, unreachable, node.last_token()) {
                (ControlFlow::Continue(..), true, Some(token)) if is_end_keyword(&token) => {
                    self.report_token(&token);
                }
                _ => {}
            }
        }
        unreachable
    }

    fn check_instr(
        &mut self,
        instr: &Instr,
        parent_block: &SyntaxNode,
        unreachable: &mut bool,
    ) -> ControlFlow<(), ()> {
        if *unreachable {
            if self.jumps.contains(parent_block) {
                *unreachable = false;
            } else {
                let end = parent_block
                    .last_token()
                    .filter(is_end_keyword)
                    .map(|token| token.text_range())
                    .unwrap_or_else(|| {
                        parent_block
                            .last_child()
                            .as_ref()
                            .unwrap_or_else(|| instr.syntax())
                            .text_range()
                    })
                    .end();
                let range = TextRange::new(instr.syntax().text_range().start(), end);
                // avoid duplicate diagnostics
                if !self.last_reported.contains_range(range) {
                    self.diags.push(Diagnostic {
                        range: helpers::rowan_range_to_lsp_range(self.line_index, range),
                        severity: Some(self.severity),
                        source: Some("wat".into()),
                        code: Some(NumberOrString::String(DIAGNOSTIC_CODE.into())),
                        message: "unreachable code".into(),
                        tags: Some(vec![DiagnosticTag::UNNECESSARY]),
                        ..Default::default()
                    });
                    self.last_reported = range;
                }
                return ControlFlow::Break(());
            }
        }
        match instr {
            Instr::Plain(plain) => {
                plain
                    .instrs()
                    .try_for_each(|instr| self.check_instr(&instr, parent_block, unreachable));
                if let Some(instr_name) = plain.instr_name() {
                    if *unreachable && !self.last_reported.contains_range(instr_name.text_range()) {
                        self.report_token(&instr_name);
                    }
                    let instr_name = instr_name.text();
                    if matches!(instr_name, "br" | "br_if" | "br_table") {
                        self.jumps
                            .extend(plain.immediates().filter_map(|immediate| {
                                let key = SymbolItemKey::new(immediate.syntax());
                                self.symbol_table
                                    .blocks
                                    .iter()
                                    .find(|block| block.ref_key == key)
                                    .and_then(|block| {
                                        block
                                            .def_key
                                            .to_node(self.root)
                                            .ancestors()
                                            .skip(1)
                                            .find(is_block_like)
                                    })
                            }));
                    }
                    *unreachable |= helpers::can_produce_never(instr_name);
                }
            }
            Instr::Block(BlockInstr::Block(block_block)) => {
                if self.check_block_like(block_block.syntax()) {
                    *unreachable = true;
                }
            }
            Instr::Block(BlockInstr::Loop(block_loop)) => {
                if self.check_block_like(block_loop.syntax()) {
                    *unreachable = true;
                }
            }
            Instr::Block(BlockInstr::If(block_if)) => {
                let if_branch = block_if
                    .then_block()
                    .is_some_and(|block| self.check_block_like(block.syntax()));
                let else_branch = block_if
                    .else_block()
                    .is_some_and(|block| self.check_block_like(block.syntax()));
                if if_branch && else_branch {
                    *unreachable = true;
                    if let Some(keyword) = block_if.end_keyword() {
                        self.report_token(&keyword);
                    }
                }
            }
        }
        ControlFlow::Continue(())
    }

    fn report_token(&mut self, token: &SyntaxToken) {
        // We don't update the last reported range for token here
        // because token is atomic and nothing can be smaller than it.
        self.diags.push(Diagnostic {
            range: helpers::rowan_range_to_lsp_range(self.line_index, token.text_range()),
            severity: Some(self.severity),
            source: Some("wat".into()),
            code: Some(NumberOrString::String(DIAGNOSTIC_CODE.into())),
            message: "unreachable code".into(),
            tags: Some(vec![DiagnosticTag::UNNECESSARY]),
            ..Default::default()
        });
    }
}

fn is_block_like(node: &SyntaxNode) -> bool {
    matches!(
        node.kind(),
        SyntaxKind::MODULE_FIELD_FUNC
            | SyntaxKind::MODULE_FIELD_GLOBAL
            | SyntaxKind::BLOCK_BLOCK
            | SyntaxKind::BLOCK_LOOP
            | SyntaxKind::BLOCK_IF
    )
}

fn is_end_keyword(token: &SyntaxToken) -> bool {
    token.kind() == SyntaxKind::KEYWORD && token.text() == "end"
}
