use crate::{
    binder::{SymbolKey, SymbolKind, SymbolTable},
    helpers::LineIndexExt,
    idx::Idx,
    uri::InternUri,
};
use line_index::LineIndex;
use lspt::{CodeAction, CodeActionKind, TextEdit, WorkspaceEdit};
use rustc_hash::FxBuildHasher;
use std::collections::HashMap;
use wat_syntax::{SyntaxKind, SyntaxNode, SyntaxToken, TextRange};

pub fn act(
    db: &dyn salsa::Database,
    uri: InternUri,
    line_index: &LineIndex,
    symbol_table: &SymbolTable,
    node: &SyntaxNode,
) -> Option<CodeAction> {
    let instr_name_token = node.tokens_by_kind(SyntaxKind::INSTR_NAME).next()?;
    let instr_name = instr_name_token.text();
    match instr_name {
        "memory.size" | "memory.grow" | "memory.fill" | "i32.load" | "i64.load" | "f32.load" | "f64.load"
        | "i32.load8_s" | "i32.load8_u" | "i32.load16_s" | "i32.load16_u" | "i64.load8_s" | "i64.load8_u"
        | "i64.load16_s" | "i64.load16_u" | "i64.load32_s" | "i64.load32_u" | "i32.store" | "i64.store"
        | "f32.store" | "f64.store" | "i32.store8" | "i32.store16" | "i64.store8" | "i64.store16" | "i64.store32"
        | "v128.load" | "v128.load8x8_s" | "v128.load8x8_u" | "v128.load16x4_s" | "v128.load16x4_u"
        | "v128.load32x2_s" | "v128.load32x2_u" | "v128.load8_splat" | "v128.load16_splat" | "v128.load32_splat"
        | "v128.load64_splat" | "v128.load32_zero" | "v128.load64_zero" | "v128.store" | "v128.load8_lane"
        | "v128.load16_lane" | "v128.load32_lane" | "v128.load64_lane" | "v128.store8_lane" | "v128.store16_lane"
        | "v128.store32_lane" | "v128.store64_lane" => {
            if node
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .is_none_or(|immediate| immediate.has_child_or_token_by_kind(SyntaxKind::MEM_ARG))
            {
                Some(build_action(
                    db,
                    uri,
                    line_index,
                    &instr_name_token,
                    format!("Add memory idx for `{instr_name}`"),
                    format!(
                        " {}",
                        retrieve_idx(symbol_table, node, SymbolKind::MemoryDef)?.render(db),
                    ),
                ))
            } else {
                None
            }
        }
        "memory.init" => {
            if node.children_by_kind(SyntaxKind::IMMEDIATE).count() < 2 {
                Some(build_action(
                    db,
                    uri,
                    line_index,
                    &instr_name_token,
                    format!("Add memory idx for `{instr_name}`"),
                    format!(
                        " {}",
                        retrieve_idx(symbol_table, node, SymbolKind::MemoryDef)?.render(db),
                    ),
                ))
            } else {
                None
            }
        }
        "memory.copy" => {
            let new_text = match node.children_by_kind(SyntaxKind::IMMEDIATE).count() {
                0 => {
                    let idx = retrieve_idx(symbol_table, node, SymbolKind::MemoryDef)?;
                    format!(" {} {}", idx.render(db), idx.render(db))
                }
                1 => {
                    format!(
                        " {}",
                        retrieve_idx(symbol_table, node, SymbolKind::MemoryDef)?.render(db)
                    )
                }
                _ => return None,
            };
            Some(build_action(
                db,
                uri,
                line_index,
                &instr_name_token,
                format!("Add memory idx for `{instr_name}`"),
                new_text,
            ))
        }
        "table.get" | "table.set" | "table.size" | "table.grow" | "table.fill" => {
            if node.children_by_kind(SyntaxKind::IMMEDIATE).next().is_none() {
                Some(build_action(
                    db,
                    uri,
                    line_index,
                    &instr_name_token,
                    format!("Add table idx for `{instr_name}`"),
                    format!(
                        " {}",
                        retrieve_idx(symbol_table, node, SymbolKind::TableDef)?.render(db),
                    ),
                ))
            } else {
                None
            }
        }
        "table.init" => {
            if node.children_by_kind(SyntaxKind::IMMEDIATE).count() < 2 {
                Some(build_action(
                    db,
                    uri,
                    line_index,
                    &instr_name_token,
                    format!("Add table idx for `{instr_name}`"),
                    format!(
                        " {}",
                        retrieve_idx(symbol_table, node, SymbolKind::TableDef)?.render(db),
                    ),
                ))
            } else {
                None
            }
        }
        "table.copy" => {
            let new_text = match node.children_by_kind(SyntaxKind::IMMEDIATE).count() {
                0 => {
                    let idx = retrieve_idx(symbol_table, node, SymbolKind::TableDef)?;
                    format!(" {} {}", idx.render(db), idx.render(db))
                }
                1 => {
                    format!(
                        " {}",
                        retrieve_idx(symbol_table, node, SymbolKind::TableDef)?.render(db)
                    )
                }
                _ => return None,
            };
            Some(build_action(
                db,
                uri,
                line_index,
                &instr_name_token,
                format!("Add table idx for `{instr_name}`"),
                new_text,
            ))
        }
        _ => None,
    }
}

fn retrieve_idx<'a>(symbol_table: &'a SymbolTable, node: &SyntaxNode, kind: SymbolKind) -> Option<Idx<'a>> {
    node.ancestors()
        .find(|ancestor| ancestor.kind() == SyntaxKind::MODULE)
        .and_then(|module| {
            symbol_table.find_def_by_idx(
                Idx {
                    num: Some(0),
                    name: None,
                },
                kind,
                SymbolKey::new(&module),
            )
        })
        .map(|symbol| symbol.idx)
}

fn build_action(
    db: &dyn salsa::Database,
    uri: InternUri,
    line_index: &LineIndex,
    instr_name_token: &SyntaxToken,
    title: String,
    new_text: String,
) -> CodeAction {
    let mut changes = HashMap::with_capacity_and_hasher(1, FxBuildHasher);
    changes.insert(
        uri.raw(db),
        vec![TextEdit {
            range: line_index.convert(TextRange::empty(instr_name_token.text_range().end())),
            new_text,
        }],
    );
    CodeAction {
        title,
        kind: Some(CodeActionKind::QuickFix),
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        }),
        ..Default::default()
    }
}
