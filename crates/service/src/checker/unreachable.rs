use crate::{
    LintLevel,
    binder::{SymbolKey, SymbolTable},
    helpers,
};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticSeverity, DiagnosticTag, Union2};
use rowan::{TextRange, ast::AstNode};
use rustc_hash::FxHashSet;
use std::ops::ControlFlow;
use wat_syntax::{
    SyntaxKind, SyntaxNode, SyntaxToken,
    ast::{BlockInstr, Instr},
};

const DIAGNOSTIC_CODE: &str = "unreachable";

pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    lint_level: LintLevel,
    line_index: &LineIndex,
    root: &SyntaxNode,
    symbol_table: &SymbolTable,
    node: &SyntaxNode,
) {
    Checker {
        diagnostics,
        line_index,
        symbol_table,
        root,
        severity: match lint_level {
            LintLevel::Allow => return,
            LintLevel::Hint => DiagnosticSeverity::Hint,
            LintLevel::Warn => DiagnosticSeverity::Warning,
            LintLevel::Deny => DiagnosticSeverity::Error,
        },
        start_jumps: FxHashSet::default(),
        end_jumps: FxHashSet::default(),
        last_reported: TextRange::default(),
    }
    .check_block_like(node);
}

struct Checker<'a, 'db> {
    diagnostics: &'a mut Vec<Diagnostic>,
    line_index: &'db LineIndex,
    symbol_table: &'db SymbolTable<'db>,
    root: &'a SyntaxNode,
    severity: DiagnosticSeverity,
    start_jumps: FxHashSet<SyntaxNode>,
    end_jumps: FxHashSet<SyntaxNode>,
    last_reported: TextRange,
}
impl Checker<'_, '_> {
    fn check_block_like(&mut self, node: &SyntaxNode) -> bool {
        let mut unreachable = false;
        let result = node
            .children()
            .filter_map(Instr::cast)
            .try_for_each(|instr| self.check_instr(&instr, &mut unreachable));
        if self.end_jumps.contains(node) {
            unreachable = false;
        }
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

    fn check_instr(&mut self, instr: &Instr, unreachable: &mut bool) -> ControlFlow<(), ()> {
        if *unreachable {
            let instr_node = instr.syntax();
            if instr_node
                .prev_sibling()
                .is_some_and(|prev| self.end_jumps.contains(&prev))
            {
                *unreachable = false;
            } else {
                if let Some(end) = instr_node
                    .ancestors()
                    .skip(1)
                    .find(is_block_like)
                    .and_then(|parent_block| {
                        parent_block
                            .last_token()
                            .filter(is_end_keyword)
                            .map(|token| token.text_range())
                            .or_else(|| parent_block.last_child().map(|last| last.text_range()))
                    })
                    .map(|range| range.end())
                {
                    let range = TextRange::new(instr_node.text_range().start(), end);
                    // avoid duplicate diagnostics
                    if !self.last_reported.contains_range(range) {
                        self.diagnostics.push(Diagnostic {
                            range: helpers::rowan_range_to_lsp_range(self.line_index, range),
                            severity: Some(self.severity),
                            source: Some("wat".into()),
                            code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                            message: "unreachable code".into(),
                            tags: Some(vec![DiagnosticTag::Unnecessary]),
                            ..Default::default()
                        });
                        self.last_reported = range;
                    }
                }
                return ControlFlow::Break(());
            }
        }
        match instr {
            Instr::Plain(plain) => {
                let _ = plain
                    .instrs()
                    .try_for_each(|instr| self.check_instr(&instr, unreachable));
                if let Some(instr_name) = plain.instr_name() {
                    if *unreachable {
                        if plain
                            .syntax()
                            .prev_sibling()
                            .is_some_and(|prev| self.end_jumps.contains(&prev))
                        {
                            *unreachable = false;
                        } else if !self.last_reported.contains_range(instr_name.text_range()) {
                            self.report_token(&instr_name);
                        }
                    }
                    let instr_name = instr_name.text();
                    if matches!(
                        instr_name,
                        "br" | "br_if" | "br_table" | "br_on_null" | "br_on_non_null"
                    ) {
                        plain
                            .immediates()
                            .filter_map(|immediate| {
                                self.symbol_table
                                    .resolved
                                    .get(&SymbolKey::new(immediate.syntax()))
                            })
                            .for_each(|def_key| {
                                let block = def_key.to_node(self.root);
                                if block.kind() == SyntaxKind::BLOCK_LOOP {
                                    self.start_jumps.insert(block);
                                } else {
                                    self.end_jumps.insert(block);
                                }
                            });
                    }
                    *unreachable |= helpers::is_stack_polymorphic(instr_name);
                }
            }
            Instr::Block(BlockInstr::Block(block_block)) => {
                if self.check_block_like(block_block.syntax()) {
                    *unreachable = true;
                }
            }
            Instr::Block(BlockInstr::Loop(block_loop)) => {
                let block_loop = block_loop.syntax();
                if self.check_block_like(block_loop) {
                    let loop_range = block_loop.text_range();
                    *unreachable = self.start_jumps.contains(block_loop)
                        && self
                            .end_jumps
                            .iter() /* internal jumps can break out of the loop */
                            .all(|jump| !loop_range.contains_range(jump.text_range()));
                }
            }
            Instr::Block(BlockInstr::If(block_if)) => {
                let _ = block_if
                    .instrs()
                    .try_for_each(|instr| self.check_instr(&instr, unreachable));
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
            Instr::Block(BlockInstr::TryTable(..)) => todo!(),
        }
        ControlFlow::Continue(())
    }

    fn report_token(&mut self, token: &SyntaxToken) {
        // We don't update the last reported range for token here
        // because token is atomic and nothing can be smaller than it.
        self.diagnostics.push(Diagnostic {
            range: helpers::rowan_range_to_lsp_range(self.line_index, token.text_range()),
            severity: Some(self.severity),
            source: Some("wat".into()),
            code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
            message: "unreachable code".into(),
            tags: Some(vec![DiagnosticTag::Unnecessary]),
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
            | SyntaxKind::BLOCK_IF_THEN
            | SyntaxKind::BLOCK_IF_ELSE
    )
}

fn is_end_keyword(token: &SyntaxToken) -> bool {
    token.kind() == SyntaxKind::KEYWORD && token.text() == "end"
}
