use super::{Diagnostic, DiagnosticCtx, RelatedInformation};
use crate::{
    binder::SymbolKey,
    helpers,
    types_analyzer::{self, ValType},
};
use std::num::IntErrorKind;
use wat_syntax::{AmberNode, AmberToken, SyntaxKind};

const DIAGNOSTIC_CODE: &str = "mem-arg";

pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    ctx: &DiagnosticCtx,
    node: AmberNode,
    instr_name: AmberToken,
) -> Option<()> {
    let (nt, action) = instr_name.text().split_once('.')?;
    let rest = action.strip_prefix("store").or_else(|| action.strip_prefix("load"))?;
    // check if instr name is applicable
    if !(rest.is_empty() || rest.starts_with(|c: char| c.is_ascii_digit())) {
        return None;
    }
    let mut immediates = node.children_by_kind(SyntaxKind::IMMEDIATE).peekable();
    let mem_def = if let Some(immediate) = immediates.next_if(|immediate| {
        immediate
            .tokens_by_kind([SyntaxKind::IDENT, SyntaxKind::INT, SyntaxKind::UNSIGNED_INT])
            .next()
            .is_some()
    }) {
        ctx.symbol_table.find_def(immediate.into())
    } else {
        ctx.symbol_table
            .modules
            .get(&SymbolKey::new(ctx.module))?
            .memories
            .first()
            .and_then(|key| ctx.symbol_table.symbols.get(key))
    };
    let mut has_align = false;
    let mut has_offset = false;
    immediates
        .filter_map(|immediate| immediate.children_by_kind(SyntaxKind::MEM_ARG).next())
        .for_each(|mem_arg| {
            match mem_arg
                .tokens_by_kind(SyntaxKind::MEM_ARG_KEYWORD)
                .next()
                .map(|token| token.text())
            {
                Some("align") => {
                    if has_align {
                        diagnostics.push(Diagnostic {
                            range: mem_arg.text_range(),
                            code: DIAGNOSTIC_CODE.into(),
                            message: "multiple alignments are not allowed".into(),
                            ..Default::default()
                        });
                    }
                    has_align = true;
                    let ty_size = match rest.split_once('_').map(|(left, _)| left).unwrap_or(rest) {
                        "8" => 1,
                        "16" => 2,
                        "32" => 4,
                        "64" | "8x8" | "16x4" | "32x2" => 8,
                        "" => match nt {
                            "i32" | "f32" => 4,
                            "i64" | "f64" => 8,
                            "v128" => 16,
                            _ => return,
                        },
                        _ => return,
                    };
                    let Some(align) = mem_arg
                        .tokens_by_kind(SyntaxKind::UNSIGNED_INT)
                        .next()
                        .and_then(|token| helpers::parse_u32(token.text()).ok())
                    else {
                        return;
                    };
                    if !align.is_power_of_two() {
                        diagnostics.push(Diagnostic {
                            range: mem_arg.text_range(),
                            code: DIAGNOSTIC_CODE.into(),
                            message: "alignment must be power-of-two".into(),
                            ..Default::default()
                        });
                    } else if align > ty_size {
                        diagnostics.push(Diagnostic {
                            range: mem_arg.text_range(),
                            code: DIAGNOSTIC_CODE.into(),
                            message: format!("alignment must be between 1 and {ty_size} inclusively"),
                            ..Default::default()
                        });
                    }
                }
                Some("offset") => {
                    if has_offset {
                        diagnostics.push(Diagnostic {
                            range: mem_arg.text_range(),
                            code: DIAGNOSTIC_CODE.into(),
                            message: "multiple offsets are not allowed".into(),
                            ..Default::default()
                        });
                    }
                    has_offset = true;
                    if let Some(mem_def) = mem_def
                        && types_analyzer::extract_addr_type(ctx.symbol_table.get_type_node_of(mem_def).green())
                            == ValType::I32
                        && let Some(uint) = mem_arg.tokens_by_kind(SyntaxKind::UNSIGNED_INT).next()
                        && let Err(error) = helpers::parse_u32(uint.text())
                        && error.kind() == &IntErrorKind::PosOverflow
                    {
                        diagnostics.push(Diagnostic {
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
                        });
                    }
                }
                _ => {}
            }
        });
    None
}
