use super::{Diagnostic, DiagnosticCtx, RelatedInformation};
use crate::{
    binder::{SymbolKey, SymbolKind},
    helpers,
    idx::Idx,
    types_analyzer::{self, ValType},
};
use std::num::IntErrorKind;
use wat_syntax::{AmberNode, AmberToken, SyntaxKind};

const DIAGNOSTIC_CODE: &str = "mem-arg";

pub fn check(ctx: &DiagnosticCtx, node: AmberNode, instr_name: AmberToken) -> Option<Diagnostic> {
    let (nt, action) = instr_name.text().split_once('.')?;
    let rest = action.strip_prefix("store").or_else(|| action.strip_prefix("load"))?;
    // check if instr name is applicable
    if !(rest.is_empty() || rest.starts_with(|c: char| c.is_ascii_digit())) {
        return None;
    }
    let mut immediates = node.children_by_kind(SyntaxKind::IMMEDIATE);
    let first = immediates.next()?;
    let (mem_def, mem_arg) = if let Some(mem_arg) = first.children_by_kind(SyntaxKind::MEM_ARG).next() {
        (
            ctx.symbol_table.find_def_by_idx(
                Idx {
                    num: Some(0),
                    name: None,
                },
                SymbolKind::MemoryDef,
                SymbolKey::new(ctx.module),
            ),
            mem_arg,
        )
    } else {
        (
            ctx.symbol_table.find_def(first.to_ptr().into()),
            immediates.next()?.children_by_kind(SyntaxKind::MEM_ARG).next()?,
        )
    };
    match mem_arg.tokens_by_kind(SyntaxKind::MEM_ARG_KEYWORD).next()?.text() {
        "align" => {
            let ty_size = match rest.split_once('_').map(|(left, _)| left).unwrap_or(rest) {
                "8" => 1,
                "16" => 2,
                "32" => 4,
                "64" | "8x8" | "16x4" | "32x2" => 8,
                "" => match nt {
                    "i32" | "f32" => 4,
                    "i64" | "f64" => 8,
                    "v128" => 16,
                    _ => return None,
                },
                _ => return None,
            };
            let align = helpers::parse_u32(mem_arg.tokens_by_kind(SyntaxKind::UNSIGNED_INT).next()?.text()).ok()?;
            if align.is_power_of_two() {
                if let Some(alignment) = 1u32.checked_shl(align)
                    && alignment <= ty_size
                {
                    None
                } else {
                    Some(Diagnostic {
                        range: mem_arg.text_range(),
                        code: DIAGNOSTIC_CODE.into(),
                        message: format!("alignment must be between 1 and {ty_size} inclusively"),
                        ..Default::default()
                    })
                }
            } else {
                Some(Diagnostic {
                    range: mem_arg.text_range(),
                    code: DIAGNOSTIC_CODE.into(),
                    message: "alignment must be power-of-two".into(),
                    ..Default::default()
                })
            }
        }
        "offset" => {
            if let Some(mem_def) = mem_def
                && types_analyzer::extract_addr_type(&mem_def.green) == ValType::I32
                && let Err(error) = helpers::parse_u32(mem_arg.tokens_by_kind(SyntaxKind::UNSIGNED_INT).next()?.text())
                && error.kind() == &IntErrorKind::PosOverflow
            {
                Some(Diagnostic {
                    range: mem_arg.text_range(),
                    code: DIAGNOSTIC_CODE.into(),
                    message: "offset is out of range".into(),
                    related_information: Some(vec![RelatedInformation {
                        range: mem_def.key.text_range(),
                        message: format!(
                            "memory `{}` defined here uses `i32` address type",
                            mem_def.idx.render(ctx.db),
                        ),
                    }]),
                    ..Default::default()
                })
            } else {
                None
            }
        }
        _ => None,
    }
}
