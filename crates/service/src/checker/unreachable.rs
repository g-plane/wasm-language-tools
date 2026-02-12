use super::Diagnostic;
use crate::{
    LintLevel,
    cfa::{self, FlowNodeKind},
    document::Document,
};
use bumpalo::{Bump, collections::Vec as BumpVec};
use lspt::{DiagnosticSeverity, DiagnosticTag};
use wat_syntax::{
    SyntaxKind, SyntaxNode, SyntaxNodePtr, TextRange,
    ast::{AstNode, Instr, support},
};

const DIAGNOSTIC_CODE: &str = "unreachable";

pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    db: &dyn salsa::Database,
    document: Document,
    lint_level: LintLevel,
    root: &SyntaxNode,
    node: &SyntaxNode,
    bump: &mut Bump,
) {
    let severity = match lint_level {
        LintLevel::Allow => return,
        LintLevel::Hint => DiagnosticSeverity::Hint,
        LintLevel::Warn => DiagnosticSeverity::Warning,
        LintLevel::Deny => DiagnosticSeverity::Error,
    };

    let cfg = cfa::analyze(db, document, SyntaxNodePtr::new(node));
    let mut ranges = BumpVec::<TextRange>::new_in(bump);
    cfg.graph.raw_nodes().iter().for_each(|node| {
        if !node.weight.unreachable {
            return;
        }
        match &node.weight.kind {
            FlowNodeKind::BasicBlock(bb) => {
                bb.0.iter().for_each(|instr| {
                    let instr = instr.ptr.to_node(root);
                    let current = instr.text_range();
                    if let Some(last) = ranges.last_mut() {
                        if instr
                            .prev_sibling()
                            .is_some_and(|prev| last.contains_range(prev.text_range()))
                        {
                            // current and previous are adjacent, so merge ranges
                            *last = last.cover(current);
                        } else if current.contains_range(*last) {
                            // this can be occurred for folded instructions
                            if instr
                                .first_child_by_kind(Instr::can_cast)
                                .is_none_or(|first| last.contains_range(first.text_range()))
                            {
                                // current instruction is the parent of last range,
                                // and all children are from the same basic block with current instruction
                                *last = current;
                            } else if let Some(instr_name) = support::token(&instr, SyntaxKind::INSTR_NAME) {
                                // if there're child instructions from different basic blocks,
                                // only mark the instruction name as unreachable
                                ranges.push(instr_name.text_range());
                            }
                        } else if !last.contains_range(current) {
                            ranges.push(current);
                        }
                    } else if instr.children().any(|child| Instr::can_cast(child.kind()))
                        && let Some(instr_name) = support::token(&instr, SyntaxKind::INSTR_NAME)
                    {
                        // this can be occurred when all child instructions are from different basic blocks
                        ranges.push(instr_name.text_range());
                    } else {
                        ranges.push(current);
                    }
                });
            }
            FlowNodeKind::BlockEntry(entry) => {
                let node = entry.to_node(root);
                if let Some((prev, last)) = node.prev_sibling().zip(ranges.last_mut())
                    && last.contains_range(prev.text_range())
                {
                    // current and previous are adjacent, so merge ranges
                    *last = last.cover(node.text_range());
                } else {
                    ranges.push(node.text_range());
                }
            }
            _ => {}
        }
    });
    diagnostics.extend(ranges.into_iter().map(|range| Diagnostic {
        range,
        severity,
        code: DIAGNOSTIC_CODE.into(),
        message: "unreachable code".into(),
        tags: Some(vec![DiagnosticTag::Unnecessary]),
        ..Default::default()
    }));

    bump.reset();
}
