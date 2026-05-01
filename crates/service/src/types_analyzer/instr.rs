use super::{
    def_type::{CompositeType, DefTypes, find_comp_type_by_idx, try_deref_cont_to_func},
    extractor::{extract_addr_type, extract_global_type, extract_table_ref_type, extract_type},
    resolver::{resolve_array_type_with_idx, resolve_br_types, resolve_field_type_with_struct_idx},
    signature::{NamedSig, Sig},
    types::{HeapType, OperandType, RefType, ValType},
};
use crate::{
    binder::{SymbolKey, SymbolKind, SymbolTable},
    data_set,
    document::Document,
    helpers,
    idx::{Idx, InternIdent},
};
use bumpalo::{Bump, collections::Vec as BumpVec};
use std::iter;
use wat_syntax::{
    AmberNode, NodeOrToken, SyntaxKind, SyntaxNode,
    ast::{AstNode, ValType as AstValType},
};

pub(crate) fn resolve_instr_sig<'db, 'bump>(
    ctx: &InstrSigResolverCtx<'db, 'bump>,
    instr_name: &str,
    instr: AmberNode<'db>,
    stack: &[OperandType<'db>],
) -> ResolvedSig<'db, 'bump> {
    let bump = ctx.bump;
    match instr_name {
        "call" | "return_call" | "throw" | "suspend" => instr
            .children_by_kind(SyntaxKind::IMMEDIATE)
            .next()
            .and_then(|idx| ctx.symbol_table.find_def(idx.to_ptr().into()))
            .map(|func| ResolvedSig::from_sig_in(Sig::from_func(ctx.db, ctx.document, func.amber()), bump))
            .unwrap_or_else(|| ResolvedSig::new_in(bump)),
        "local.get" => ResolvedSig {
            params: BumpVec::new_in(bump),
            results: BumpVec::from_iter_in(
                [instr
                    .children_by_kind(SyntaxKind::IMMEDIATE)
                    .next()
                    .and_then(|idx| ctx.symbol_table.find_def(idx.to_ptr().into()))
                    .and_then(|symbol| extract_type(ctx.db, &symbol.green))
                    .map_or(OperandType::Any, OperandType::Val)],
                bump,
            ),
        },
        "local.set" => ResolvedSig {
            params: BumpVec::from_iter_in(
                [instr
                    .children_by_kind(SyntaxKind::IMMEDIATE)
                    .next()
                    .and_then(|idx| ctx.symbol_table.find_def(idx.to_ptr().into()))
                    .and_then(|symbol| extract_type(ctx.db, &symbol.green))
                    .map_or(OperandType::Any, OperandType::Val)],
                bump,
            ),
            results: BumpVec::new_in(bump),
        },
        "local.tee" => {
            let ty = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|idx| ctx.symbol_table.find_def(idx.to_ptr().into()))
                .and_then(|symbol| extract_type(ctx.db, &symbol.green))
                .map_or(OperandType::Any, OperandType::Val);
            ResolvedSig {
                params: BumpVec::from_iter_in([ty.clone()], bump),
                results: BumpVec::from_iter_in([ty], bump),
            }
        }
        "global.get" => ResolvedSig {
            params: BumpVec::new_in(bump),
            results: BumpVec::from_iter_in(
                [instr
                    .children_by_kind(SyntaxKind::IMMEDIATE)
                    .next()
                    .and_then(|idx| ctx.symbol_table.find_def(idx.to_ptr().into()))
                    .and_then(|symbol| extract_global_type(ctx.db, &symbol.green))
                    .map_or(OperandType::Any, OperandType::Val)],
                bump,
            ),
        },
        "global.set" => ResolvedSig {
            params: BumpVec::from_iter_in(
                [instr
                    .children_by_kind(SyntaxKind::IMMEDIATE)
                    .next()
                    .and_then(|idx| ctx.symbol_table.find_def(idx.to_ptr().into()))
                    .and_then(|symbol| extract_global_type(ctx.db, &symbol.green))
                    .map_or(OperandType::Any, OperandType::Val)],
                bump,
            ),
            results: BumpVec::new_in(bump),
        },
        "i32.const" => ResolvedSig {
            params: BumpVec::new_in(bump),
            results: BumpVec::from_iter_in([OperandType::Val(ValType::I32)], bump),
        },
        "i64.const" => ResolvedSig {
            params: BumpVec::new_in(bump),
            results: BumpVec::from_iter_in([OperandType::Val(ValType::I64)], bump),
        },
        "f32.const" => ResolvedSig {
            params: BumpVec::new_in(bump),
            results: BumpVec::from_iter_in([OperandType::Val(ValType::F32)], bump),
        },
        "f64.const" => ResolvedSig {
            params: BumpVec::new_in(bump),
            results: BumpVec::from_iter_in([OperandType::Val(ValType::F64)], bump),
        },
        "return" => ResolvedSig {
            params: ctx
                .symbol_table
                .symbols
                .values()
                .find(|symbol| {
                    symbol.kind == SymbolKind::Func && symbol.key.text_range().contains_range(instr.text_range())
                })
                .map(|func| {
                    BumpVec::from_iter_in(
                        Sig::from_func(ctx.db, ctx.document, func.amber())
                            .results
                            .into_iter()
                            .map(OperandType::Val),
                        bump,
                    )
                })
                .unwrap_or_else(|| BumpVec::new_in(bump)),
            results: BumpVec::new_in(bump),
        },
        "drop" => ResolvedSig {
            params: BumpVec::from_iter_in([stack.last().map_or(OperandType::Any, |ty| ty.clone())], bump),
            results: BumpVec::new_in(bump),
        },
        "br" => ResolvedSig {
            params: instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|idx| resolve_br_types(ctx.db, ctx.document, ctx.symbol_table, idx.to_ptr().into()))
                .map(|types| BumpVec::from_iter_in(types, bump))
                .unwrap_or_else(|| BumpVec::new_in(bump)),
            results: BumpVec::new_in(bump),
        },
        "br_if" => {
            let results = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|idx| resolve_br_types(ctx.db, ctx.document, ctx.symbol_table, idx.to_ptr().into()))
                .map(|types| BumpVec::from_iter_in(types, bump))
                .unwrap_or_else(|| BumpVec::new_in(bump));
            let params = BumpVec::from_iter_in(
                results
                    .iter()
                    .cloned()
                    .chain(iter::once(OperandType::Val(ValType::I32))),
                bump,
            );
            ResolvedSig { params, results }
        }
        "br_table" => {
            let mut params = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|idx| resolve_br_types(ctx.db, ctx.document, ctx.symbol_table, idx.to_ptr().into()))
                .map(|types| BumpVec::from_iter_in(types, bump))
                .unwrap_or_else(|| BumpVec::new_in(bump));
            params.push(OperandType::Val(ValType::I32));
            ResolvedSig {
                params,
                results: BumpVec::new_in(bump),
            }
        }
        "br_on_null" => {
            let heap_ty = if let Some(OperandType::Val(ValType::Ref(RefType { heap_ty, .. }))) = stack.last() {
                heap_ty.clone()
            } else {
                HeapType::Any
            };
            let mut results = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|idx| resolve_br_types(ctx.db, ctx.document, ctx.symbol_table, idx.to_ptr().into()))
                .map(|types| BumpVec::from_iter_in(types, bump))
                .unwrap_or_else(|| BumpVec::new_in(bump));
            let params = BumpVec::from_iter_in(
                results
                    .iter()
                    .cloned()
                    .chain(iter::once(OperandType::Val(ValType::Ref(RefType {
                        heap_ty: heap_ty.clone(),
                        nullable: true,
                    })))),
                bump,
            );
            results.push(OperandType::Val(ValType::Ref(RefType {
                heap_ty: heap_ty.clone(),
                nullable: false,
            })));
            ResolvedSig { params, results }
        }
        "br_on_non_null" => {
            let mut params = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|idx| resolve_br_types(ctx.db, ctx.document, ctx.symbol_table, idx.to_ptr().into()))
                .map(|types| BumpVec::from_iter_in(types, bump))
                .unwrap_or_else(|| BumpVec::new_in(bump));
            let last = if let Some(OperandType::Val(ValType::Ref(RefType { heap_ty, .. }))) = params.pop() {
                OperandType::Val(ValType::Ref(RefType {
                    heap_ty,
                    nullable: true,
                }))
            } else {
                OperandType::Any
            };
            let results = BumpVec::from_iter_in(params.iter().cloned(), bump);
            params.push(last);
            ResolvedSig { params, results }
        }
        "br_on_cast" => {
            let mut immediates = instr.children_by_kind(SyntaxKind::IMMEDIATE);
            let mut types = immediates
                .next()
                .and_then(|idx| resolve_br_types(ctx.db, ctx.document, ctx.symbol_table, idx.to_ptr().into()))
                .map(|types| BumpVec::from_iter_in(types, bump))
                .unwrap_or_else(|| BumpVec::new_in(bump));
            types.pop();
            let rt1 = immediates
                .next()
                .and_then(|immediate| immediate.children_by_kind(SyntaxKind::REF_TYPE).next())
                .and_then(|ref_type| RefType::from_green(ref_type.green(), ctx.db));
            let rt2 = immediates
                .next()
                .and_then(|immediate| immediate.children_by_kind(SyntaxKind::REF_TYPE).next())
                .and_then(|ref_type| RefType::from_green(ref_type.green(), ctx.db));
            let mut params = BumpVec::from_iter_in(types.iter().cloned(), bump);
            let mut results = types;
            if let Some((rt1, rt2)) = rt1.zip(rt2) {
                params.push(OperandType::Val(ValType::Ref(rt1.clone())));
                results.push(OperandType::Val(ValType::Ref(rt1.diff(&rt2))));
            }
            ResolvedSig { params, results }
        }
        "br_on_cast_fail" => {
            let mut immediates = instr.children_by_kind(SyntaxKind::IMMEDIATE);
            let mut types = immediates
                .next()
                .and_then(|idx| resolve_br_types(ctx.db, ctx.document, ctx.symbol_table, idx.to_ptr().into()))
                .map(|types| BumpVec::from_iter_in(types, bump))
                .unwrap_or_else(|| BumpVec::new_in(bump));
            types.pop();
            let rt1 = immediates
                .next()
                .and_then(|immediate| immediate.children_by_kind(SyntaxKind::REF_TYPE).next())
                .and_then(|ref_type| RefType::from_green(ref_type.green(), ctx.db));
            let rt2 = immediates
                .next()
                .and_then(|immediate| immediate.children_by_kind(SyntaxKind::REF_TYPE).next())
                .and_then(|ref_type| RefType::from_green(ref_type.green(), ctx.db));
            let mut params = BumpVec::from_iter_in(types.iter().cloned(), bump);
            let mut results = types;
            if let Some((rt1, rt2)) = rt1.zip(rt2) {
                params.push(OperandType::Val(ValType::Ref(rt1)));
                results.push(OperandType::Val(ValType::Ref(rt2)));
            }
            ResolvedSig { params, results }
        }
        "select" => {
            let ty = if let Some(ty) = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| immediate.children_by_kind(SyntaxKind::TYPE_USE).next())
                .and_then(|type_use| type_use.children_by_kind(SyntaxKind::RESULT).next())
                .and_then(|result| result.children_by_kind(AstValType::can_cast).next())
            {
                ValType::from_green(ty.green(), ctx.db).map_or(OperandType::Any, OperandType::Val)
            } else {
                stack
                    .len()
                    .checked_sub(2)
                    .and_then(|i| stack.get(i))
                    .map_or(OperandType::Any, |ty| ty.clone())
            };
            ResolvedSig {
                params: BumpVec::from_iter_in([ty.clone(), ty.clone(), OperandType::Val(ValType::I32)], bump),
                results: BumpVec::from_iter_in([ty], bump),
            }
        }
        "call_indirect" | "return_call_indirect" => {
            let mut sig = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .find_map(|child| child.children_by_kind(SyntaxKind::TYPE_USE).next())
                .map(|node| ResolvedSig::from_sig_in(Sig::from_type_use(ctx.db, ctx.document, node), bump))
                .unwrap_or_else(|| ResolvedSig::new_in(bump));
            sig.params.push(OperandType::Val(ValType::I32));
            sig
        }
        "struct.new" => instr
            .children_by_kind(SyntaxKind::IMMEDIATE)
            .next()
            .and_then(|immediate| ctx.symbol_table.resolved.get(&immediate.to_ptr().into()))
            .and_then(|key| ctx.def_types.get(key))
            .map(|def_type| {
                let params = if let CompositeType::Struct(fields) = &def_type.comp {
                    BumpVec::from_iter_in(fields.0.iter().map(|(field, _)| field.storage.clone().into()), bump)
                } else {
                    BumpVec::new_in(bump)
                };
                ResolvedSig {
                    params,
                    results: BumpVec::from_iter_in(
                        [OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(def_type.idx),
                            nullable: false,
                        }))],
                        bump,
                    ),
                }
            })
            .unwrap_or_else(|| ResolvedSig {
                params: BumpVec::new_in(bump),
                results: BumpVec::from_iter_in([OperandType::Any], bump),
            }),
        "struct.new_default" => instr
            .children_by_kind(SyntaxKind::IMMEDIATE)
            .next()
            .and_then(|idx| ctx.symbol_table.find_def(idx.to_ptr().into()))
            .map(|symbol| ResolvedSig {
                params: BumpVec::new_in(bump),
                results: BumpVec::from_iter_in(
                    [OperandType::Val(ValType::Ref(RefType {
                        heap_ty: HeapType::Type(symbol.idx),
                        nullable: false,
                    }))],
                    bump,
                ),
            })
            .unwrap_or_else(|| ResolvedSig {
                params: BumpVec::new_in(bump),
                results: BumpVec::from_iter_in([OperandType::Any], bump),
            }),
        "struct.get" => {
            let mut immediates = instr.children_by_kind(SyntaxKind::IMMEDIATE);
            immediates
                .next()
                .zip(immediates.next())
                .and_then(|(struct_ref, field_ref)| {
                    resolve_field_type_with_struct_idx(ctx.db, ctx.document, struct_ref.to_ptr(), field_ref.to_ptr())
                })
                .map(|(idx, ty)| ResolvedSig {
                    params: BumpVec::from_iter_in(
                        [OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(idx),
                            nullable: true,
                        }))],
                        bump,
                    ),
                    results: BumpVec::from_iter_in([ty.unwrap_or(OperandType::Any)], bump),
                })
                .unwrap_or_else(|| ResolvedSig {
                    params: BumpVec::new_in(bump),
                    results: BumpVec::from_iter_in([OperandType::Any], bump),
                })
        }
        "struct.get_s" | "struct.get_u" => ResolvedSig {
            params: instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| ctx.symbol_table.find_def(immediate.to_ptr().into()))
                .map(|symbol| {
                    BumpVec::from_iter_in(
                        [OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(symbol.idx),
                            nullable: true,
                        }))],
                        bump,
                    )
                })
                .unwrap_or_else(|| BumpVec::new_in(bump)),
            results: BumpVec::from_iter_in([OperandType::Val(ValType::I32)], bump),
        },
        "struct.set" => {
            let mut immediates = instr.children_by_kind(SyntaxKind::IMMEDIATE);
            immediates
                .next()
                .zip(immediates.next())
                .and_then(|(struct_ref, field_ref)| {
                    resolve_field_type_with_struct_idx(ctx.db, ctx.document, struct_ref.to_ptr(), field_ref.to_ptr())
                })
                .map(|(idx, ty)| ResolvedSig {
                    params: BumpVec::from_iter_in(
                        [
                            OperandType::Val(ValType::Ref(RefType {
                                heap_ty: HeapType::Type(idx),
                                nullable: true,
                            })),
                            ty.unwrap_or(OperandType::Any),
                        ],
                        bump,
                    ),
                    results: BumpVec::new_in(bump),
                })
                .unwrap_or_else(|| ResolvedSig {
                    params: BumpVec::from_iter_in([OperandType::Any], bump),
                    results: BumpVec::new_in(bump),
                })
        }
        "array.new" => {
            let mut sig = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| resolve_array_type_with_idx(ctx.symbol_table, ctx.def_types, immediate.to_ptr()))
                .map(|(idx, ty)| ResolvedSig {
                    params: BumpVec::from_iter_in([ty.unwrap_or(OperandType::Any)], bump),
                    results: BumpVec::from_iter_in(
                        [OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(idx),
                            nullable: false,
                        }))],
                        bump,
                    ),
                })
                .unwrap_or_else(|| ResolvedSig {
                    params: BumpVec::new_in(bump),
                    results: BumpVec::from_iter_in([OperandType::Any], bump),
                });
            sig.params.push(OperandType::Val(ValType::I32));
            sig
        }
        "array.new_default" => instr
            .children_by_kind(SyntaxKind::IMMEDIATE)
            .next()
            .and_then(|idx| ctx.symbol_table.find_def(idx.to_ptr().into()))
            .map(|symbol| ResolvedSig {
                params: BumpVec::from_iter_in([OperandType::Val(ValType::I32)], bump),
                results: BumpVec::from_iter_in(
                    [OperandType::Val(ValType::Ref(RefType {
                        heap_ty: HeapType::Type(symbol.idx),
                        nullable: false,
                    }))],
                    bump,
                ),
            })
            .unwrap_or_else(|| ResolvedSig {
                params: BumpVec::new_in(bump),
                results: BumpVec::from_iter_in([OperandType::Any], bump),
            }),
        "array.new_fixed" => {
            let mut immediates = instr.children_by_kind(SyntaxKind::IMMEDIATE);
            immediates
                .next()
                .and_then(|immediate| resolve_array_type_with_idx(ctx.symbol_table, ctx.def_types, immediate.to_ptr()))
                .map(|(idx, ty)| {
                    let count = immediates
                        .next()
                        .and_then(|immediate| {
                            immediate
                                .green()
                                .children()
                                .find_map(|node_or_token| match node_or_token {
                                    NodeOrToken::Token(token) if token.kind() == SyntaxKind::INT => Some(token),
                                    _ => None,
                                })
                        })
                        .and_then(|int| helpers::parse_u32(int.text()).ok())
                        .unwrap_or_default();
                    ResolvedSig {
                        params: BumpVec::from_iter_in(
                            iter::repeat_n(ty.unwrap_or(OperandType::Any), count as usize),
                            bump,
                        ),
                        results: BumpVec::from_iter_in(
                            [OperandType::Val(ValType::Ref(RefType {
                                heap_ty: HeapType::Type(idx),
                                nullable: false,
                            }))],
                            bump,
                        ),
                    }
                })
                .unwrap_or_else(|| ResolvedSig {
                    params: BumpVec::new_in(bump),
                    results: BumpVec::from_iter_in([OperandType::Any], bump),
                })
        }
        "array.new_data" | "array.new_elem" => instr
            .children_by_kind(SyntaxKind::IMMEDIATE)
            .next()
            .and_then(|idx| ctx.symbol_table.find_def(idx.to_ptr().into()))
            .map(|symbol| ResolvedSig {
                params: BumpVec::from_iter_in(iter::repeat_n(OperandType::Val(ValType::I32), 2), bump),
                results: BumpVec::from_iter_in(
                    [OperandType::Val(ValType::Ref(RefType {
                        heap_ty: HeapType::Type(symbol.idx),
                        nullable: false,
                    }))],
                    bump,
                ),
            })
            .unwrap_or_else(|| ResolvedSig {
                params: BumpVec::new_in(bump),
                results: BumpVec::from_iter_in([OperandType::Any], bump),
            }),
        "array.get" => instr
            .children_by_kind(SyntaxKind::IMMEDIATE)
            .next()
            .and_then(|immediate| resolve_array_type_with_idx(ctx.symbol_table, ctx.def_types, immediate.to_ptr()))
            .map(|(idx, ty)| ResolvedSig {
                params: BumpVec::from_iter_in(
                    [
                        OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(idx),
                            nullable: true,
                        })),
                        OperandType::Val(ValType::I32),
                    ],
                    bump,
                ),
                results: BumpVec::from_iter_in([ty.unwrap_or(OperandType::Any)], bump),
            })
            .unwrap_or_else(|| ResolvedSig {
                params: BumpVec::new_in(bump),
                results: BumpVec::from_iter_in([OperandType::Any], bump),
            }),
        "array.get_s" | "array.get_u" => instr
            .children_by_kind(SyntaxKind::IMMEDIATE)
            .next()
            .and_then(|idx| ctx.symbol_table.find_def(idx.to_ptr().into()))
            .map(|symbol| ResolvedSig {
                params: BumpVec::from_iter_in(
                    [
                        OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(symbol.idx),
                            nullable: true,
                        })),
                        OperandType::Val(ValType::I32),
                    ],
                    bump,
                ),
                results: BumpVec::from_iter_in([OperandType::Val(ValType::I32)], bump),
            })
            .unwrap_or_else(|| ResolvedSig {
                params: BumpVec::new_in(bump),
                results: BumpVec::from_iter_in([OperandType::Val(ValType::I32)], bump),
            }),
        "array.set" => instr
            .children_by_kind(SyntaxKind::IMMEDIATE)
            .next()
            .and_then(|immediate| resolve_array_type_with_idx(ctx.symbol_table, ctx.def_types, immediate.to_ptr()))
            .map(|(idx, ty)| ResolvedSig {
                params: BumpVec::from_iter_in(
                    [
                        OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(idx),
                            nullable: true,
                        })),
                        OperandType::Val(ValType::I32),
                        ty.unwrap_or(OperandType::Any),
                    ],
                    bump,
                ),
                results: BumpVec::new_in(bump),
            })
            .unwrap_or_else(|| ResolvedSig::new_in(bump)),
        "array.fill" => instr
            .children_by_kind(SyntaxKind::IMMEDIATE)
            .next()
            .and_then(|immediate| resolve_array_type_with_idx(ctx.symbol_table, ctx.def_types, immediate.to_ptr()))
            .map(|(idx, ty)| ResolvedSig {
                params: BumpVec::from_iter_in(
                    [
                        OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(idx),
                            nullable: true,
                        })),
                        OperandType::Val(ValType::I32),
                        ty.unwrap_or(OperandType::Any),
                        OperandType::Val(ValType::I32),
                    ],
                    bump,
                ),
                results: BumpVec::new_in(bump),
            })
            .unwrap_or_else(|| ResolvedSig::new_in(bump)),
        "array.copy" => {
            let mut immediates = instr.children_by_kind(SyntaxKind::IMMEDIATE);
            immediates
                .next()
                .and_then(|idx| ctx.symbol_table.find_def(idx.to_ptr().into()))
                .zip(
                    immediates
                        .next()
                        .and_then(|idx| ctx.symbol_table.find_def(idx.to_ptr().into())),
                )
                .map(|(dst, src)| ResolvedSig {
                    params: BumpVec::from_iter_in(
                        [
                            OperandType::Val(ValType::Ref(RefType {
                                heap_ty: HeapType::Type(dst.idx),
                                nullable: true,
                            })),
                            OperandType::Val(ValType::I32),
                            OperandType::Val(ValType::Ref(RefType {
                                heap_ty: HeapType::Type(src.idx),
                                nullable: true,
                            })),
                            OperandType::Val(ValType::I32),
                            OperandType::Val(ValType::I32),
                        ],
                        bump,
                    ),
                    results: BumpVec::new_in(bump),
                })
                .unwrap_or_else(|| ResolvedSig::new_in(bump))
        }
        "array.init_data" | "array.init_elem" => instr
            .children_by_kind(SyntaxKind::IMMEDIATE)
            .next()
            .and_then(|idx| ctx.symbol_table.find_def(idx.to_ptr().into()))
            .map(|symbol| ResolvedSig {
                params: BumpVec::from_iter_in(
                    [
                        OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(symbol.idx),
                            nullable: true,
                        })),
                        OperandType::Val(ValType::I32),
                        OperandType::Val(ValType::I32),
                        OperandType::Val(ValType::I32),
                    ],
                    bump,
                ),
                results: BumpVec::new_in(bump),
            })
            .unwrap_or_else(|| ResolvedSig::new_in(bump)),
        "ref.null" => {
            let ty = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| immediate.green().children().next())
                .and_then(|node_or_token| match node_or_token {
                    NodeOrToken::Node(node) if node.kind() == SyntaxKind::HEAP_TYPE => {
                        HeapType::from_green(node, ctx.db)
                    }
                    NodeOrToken::Token(token) if token.kind() == SyntaxKind::IDENT => Some(HeapType::Type(Idx {
                        num: None,
                        name: Some(InternIdent::new(ctx.db, token.text())),
                    })),
                    NodeOrToken::Token(token) if token.kind() == SyntaxKind::INT => Some(HeapType::Type(Idx {
                        num: helpers::parse_u32(token.text()).ok(),
                        name: None,
                    })),
                    _ => None,
                })
                .map_or(OperandType::Any, |heap_ty| {
                    OperandType::Val(ValType::Ref(RefType {
                        heap_ty,
                        nullable: true,
                    }))
                });
            ResolvedSig {
                params: BumpVec::new_in(bump),
                results: BumpVec::from_iter_in([ty], bump),
            }
        }
        "ref.is_null" => {
            let heap_ty = if let Some(OperandType::Val(ValType::Ref(RefType { heap_ty, .. }))) = stack.last() {
                heap_ty.clone()
            } else {
                HeapType::Any
            };
            ResolvedSig {
                params: BumpVec::from_iter_in(
                    [OperandType::Val(ValType::Ref(RefType {
                        heap_ty,
                        nullable: true,
                    }))],
                    bump,
                ),
                results: BumpVec::from_iter_in([OperandType::Val(ValType::I32)], bump),
            }
        }
        "ref.as_non_null" => {
            let heap_ty = if let Some(OperandType::Val(ValType::Ref(RefType { heap_ty, .. }))) = stack.last() {
                heap_ty.clone()
            } else {
                HeapType::Any
            };
            ResolvedSig {
                params: BumpVec::from_iter_in(
                    [OperandType::Val(ValType::Ref(RefType {
                        heap_ty: heap_ty.clone(),
                        nullable: true,
                    }))],
                    bump,
                ),
                results: BumpVec::from_iter_in(
                    [OperandType::Val(ValType::Ref(RefType {
                        heap_ty: heap_ty.clone(),
                        nullable: false,
                    }))],
                    bump,
                ),
            }
        }
        "ref.test" => {
            let heap_ty = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| immediate.children_by_kind(SyntaxKind::REF_TYPE).next())
                .and_then(|ref_type| RefType::from_green(ref_type.green(), ctx.db))
                .and_then(|ref_type| ref_type.heap_ty.to_top_type(ctx.db, ctx.document, ctx.module_id))
                .unwrap_or(HeapType::Any);
            ResolvedSig {
                params: BumpVec::from_iter_in(
                    [OperandType::Val(ValType::Ref(RefType {
                        heap_ty,
                        nullable: true,
                    }))],
                    bump,
                ),
                results: BumpVec::from_iter_in([OperandType::Val(ValType::I32)], bump),
            }
        }
        "ref.cast" => {
            let ref_type = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| immediate.children_by_kind(SyntaxKind::REF_TYPE).next())
                .and_then(|ref_type| RefType::from_green(ref_type.green(), ctx.db))
                .unwrap_or(RefType {
                    heap_ty: HeapType::Any,
                    nullable: true,
                });
            let heap_ty = ref_type
                .heap_ty
                .to_top_type(ctx.db, ctx.document, ctx.module_id)
                .unwrap_or(HeapType::Any);
            ResolvedSig {
                params: BumpVec::from_iter_in(
                    [OperandType::Val(ValType::Ref(RefType {
                        heap_ty,
                        nullable: true,
                    }))],
                    bump,
                ),
                results: BumpVec::from_iter_in([OperandType::Val(ValType::Ref(ref_type))], bump),
            }
        }
        "ref.func" => {
            let immediate = instr.children_by_kind(SyntaxKind::IMMEDIATE).next();
            let heap_ty = immediate
                .as_ref()
                .and_then(|immediate| ctx.symbol_table.find_def(immediate.to_ptr().into()))
                .and_then(|def_symbol| def_symbol.amber().children_by_kind(SyntaxKind::TYPE_USE).next())
                .and_then(|type_use| type_use.children_by_kind(SyntaxKind::INDEX).next())
                .and_then(|index| Idx::from_green_for_ref(index.green(), ctx.db))
                .map(HeapType::Type)
                .or_else(|| {
                    immediate
                        .and_then(|immediate| Idx::from_green_for_ref(immediate.green(), ctx.db))
                        .map(HeapType::DefFunc)
                });
            ResolvedSig {
                params: BumpVec::new_in(bump),
                results: BumpVec::from_iter_in(
                    [heap_ty.map_or(OperandType::Any, |heap_ty| {
                        OperandType::Val(ValType::Ref(RefType {
                            heap_ty,
                            nullable: false,
                        }))
                    })],
                    bump,
                ),
            }
        }
        "call_ref" | "return_call_ref" => instr
            .children_by_kind(SyntaxKind::IMMEDIATE)
            .next()
            .and_then(|immediate| ctx.symbol_table.resolved.get(&immediate.to_ptr().into()))
            .and_then(|key| ctx.def_types.get(key))
            .map(|def_type| {
                let mut sig = def_type
                    .comp
                    .as_func()
                    .map(|sig| ResolvedSig {
                        params: BumpVec::from_iter_in(
                            sig.params.iter().map(|(ty, _)| OperandType::Val(ty.clone())),
                            bump,
                        ),
                        results: BumpVec::from_iter_in(sig.results.iter().map(|ty| OperandType::Val(ty.clone())), bump),
                    })
                    .unwrap_or_else(|| ResolvedSig::new_in(bump));
                sig.params.push(OperandType::Val(ValType::Ref(RefType {
                    heap_ty: HeapType::Type(def_type.idx),
                    nullable: true,
                })));
                sig
            })
            .unwrap_or_else(|| ResolvedSig::new_in(bump)),
        "cont.new" => instr
            .children_by_kind(SyntaxKind::IMMEDIATE)
            .next()
            .and_then(|immediate| ctx.symbol_table.resolved.get(&immediate.to_ptr().into()))
            .and_then(|key| ctx.def_types.get(key))
            .map(|def_type| {
                let param = if let CompositeType::Cont(heap_ty) = &def_type.comp {
                    OperandType::Val(ValType::Ref(RefType {
                        heap_ty: heap_ty.clone(),
                        nullable: true,
                    }))
                } else {
                    OperandType::Any
                };
                ResolvedSig {
                    params: BumpVec::from_iter_in([param], bump),
                    results: BumpVec::from_iter_in(
                        [OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(def_type.idx),
                            nullable: false,
                        }))],
                        bump,
                    ),
                }
            })
            .unwrap_or_else(|| ResolvedSig {
                params: BumpVec::from_iter_in([OperandType::Any], bump),
                results: BumpVec::from_iter_in([OperandType::Any], bump),
            }),
        "cont.bind" => {
            let mut immediates = instr.children_by_kind(SyntaxKind::IMMEDIATE);
            immediates
                .next()
                .and_then(|immediate| ctx.symbol_table.resolved.get(&immediate.to_ptr().into()))
                .and_then(|key| ctx.def_types.get(key))
                .zip(
                    immediates
                        .next()
                        .and_then(|immediate| ctx.symbol_table.resolved.get(&immediate.to_ptr().into()))
                        .and_then(|key| ctx.def_types.get(key)),
                )
                .map(|(fst, snd)| {
                    let module = SymbolKey::new(ctx.module);
                    let applied = if let Some(fst_sig) =
                        try_deref_cont_to_func(ctx.symbol_table, ctx.def_types, &fst.comp, module)
                        && let Some(snd_sig) =
                            try_deref_cont_to_func(ctx.symbol_table, ctx.def_types, &snd.comp, module)
                    {
                        fst_sig
                            .params
                            .get(0..fst_sig.params.len().saturating_sub(snd_sig.params.len()))
                    } else {
                        None
                    };
                    ResolvedSig {
                        params: BumpVec::from_iter_in(
                            applied
                                .into_iter()
                                .flatten()
                                .map(|(ty, _)| OperandType::Val(ty.clone()))
                                .chain(iter::once(OperandType::Val(ValType::Ref(RefType {
                                    heap_ty: HeapType::Type(fst.idx),
                                    nullable: true,
                                })))),
                            bump,
                        ),
                        results: BumpVec::from_iter_in(
                            [OperandType::Val(ValType::Ref(RefType {
                                heap_ty: HeapType::Type(snd.idx),
                                nullable: false,
                            }))],
                            bump,
                        ),
                    }
                })
                .unwrap_or_else(|| ResolvedSig {
                    params: BumpVec::from_iter_in([OperandType::Any], bump),
                    results: BumpVec::from_iter_in([OperandType::Any], bump),
                })
        }
        "resume" => instr
            .children_by_kind(SyntaxKind::IMMEDIATE)
            .next()
            .and_then(|immediate| ctx.symbol_table.resolved.get(&immediate.to_ptr().into()))
            .and_then(|key| ctx.def_types.get(key))
            .and_then(|def_type| {
                try_deref_cont_to_func(
                    ctx.symbol_table,
                    ctx.def_types,
                    &def_type.comp,
                    SymbolKey::new(ctx.module),
                )
                .map(|sig| {
                    let mut sig = ResolvedSig::from_named_sig_in(sig.clone(), bump);
                    sig.params.push(OperandType::Val(ValType::Ref(RefType {
                        heap_ty: HeapType::Type(def_type.idx),
                        nullable: true,
                    })));
                    sig
                })
            })
            .unwrap_or_else(|| ResolvedSig {
                params: BumpVec::from_iter_in([OperandType::Any], bump),
                results: BumpVec::new_in(bump),
            }),
        "resume_throw" => {
            let mut immediates = instr.children_by_kind(SyntaxKind::IMMEDIATE);
            let ct = immediates
                .next()
                .and_then(|immediate| ctx.symbol_table.find_def(immediate.to_ptr().into()));
            let params = BumpVec::from_iter_in(
                immediates
                    .next()
                    .and_then(|immediate| ctx.symbol_table.find_def(immediate.to_ptr().into()))
                    .into_iter()
                    .flat_map(|symbol| {
                        Sig::from_func(ctx.db, ctx.document, symbol.amber())
                            .params
                            .into_iter()
                            .map(OperandType::Val)
                    })
                    .chain(ct.map(|symbol| {
                        OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(symbol.idx),
                            nullable: true,
                        }))
                    })),
                bump,
            );
            let results = ct
                .and_then(|symbol| ctx.def_types.get(&symbol.key))
                .and_then(|def_type| {
                    try_deref_cont_to_func(
                        ctx.symbol_table,
                        ctx.def_types,
                        &def_type.comp,
                        SymbolKey::new(ctx.module),
                    )
                })
                .map(|sig| BumpVec::from_iter_in(sig.results.iter().map(|ty| OperandType::Val(ty.clone())), bump))
                .unwrap_or_else(|| BumpVec::new_in(bump));
            ResolvedSig { params, results }
        }
        "resume_throw_ref" => instr
            .children_by_kind(SyntaxKind::IMMEDIATE)
            .next()
            .and_then(|immediate| ctx.symbol_table.find_def(immediate.to_ptr().into()))
            .map(|symbol| {
                let params = BumpVec::from_iter_in(
                    [
                        OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Exn,
                            nullable: true,
                        })),
                        OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(symbol.idx),
                            nullable: true,
                        })),
                    ],
                    bump,
                );
                let results = ctx
                    .def_types
                    .get(&symbol.key)
                    .and_then(|def_type| {
                        try_deref_cont_to_func(
                            ctx.symbol_table,
                            ctx.def_types,
                            &def_type.comp,
                            SymbolKey::new(ctx.module),
                        )
                    })
                    .map(|sig| BumpVec::from_iter_in(sig.results.iter().map(|ty| OperandType::Val(ty.clone())), bump))
                    .unwrap_or_else(|| BumpVec::new_in(bump));
                ResolvedSig { params, results }
            })
            .unwrap_or_else(|| ResolvedSig {
                params: BumpVec::from_iter_in(
                    [
                        OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Exn,
                            nullable: true,
                        })),
                        OperandType::Any,
                    ],
                    bump,
                ),
                results: BumpVec::new_in(bump),
            }),
        "switch" => {
            let module = SymbolKey::new(ctx.module);
            instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| ctx.symbol_table.find_def(immediate.to_ptr().into()))
                .and_then(|def_symbol| {
                    ctx.def_types
                        .get(&def_symbol.key)
                        .and_then(|def_type| {
                            try_deref_cont_to_func(ctx.symbol_table, ctx.def_types, &def_type.comp, module)
                        })
                        .and_then(|sig| sig.params.split_last())
                        .map(|((last, _), lead)| {
                            let params = BumpVec::from_iter_in(
                                lead.iter()
                                    .map(|(ty, _)| OperandType::Val(ty.clone()))
                                    .chain(iter::once(OperandType::Val(ValType::Ref(RefType {
                                        heap_ty: HeapType::Type(def_symbol.idx),
                                        nullable: true,
                                    })))),
                                bump,
                            );
                            let results = if let ValType::Ref(RefType {
                                heap_ty: HeapType::Type(idx),
                                ..
                            }) = last
                                && let Some(comp) = find_comp_type_by_idx(ctx.symbol_table, ctx.def_types, *idx, module)
                                && let Some(last_ct_sig) =
                                    try_deref_cont_to_func(ctx.symbol_table, ctx.def_types, comp, module)
                            {
                                BumpVec::from_iter_in(
                                    last_ct_sig.params.iter().map(|(ty, _)| OperandType::Val(ty.clone())),
                                    bump,
                                )
                            } else {
                                BumpVec::new_in(bump)
                            };
                            ResolvedSig { params, results }
                        })
                })
                .unwrap_or_else(|| ResolvedSig {
                    params: BumpVec::from_iter_in([OperandType::Any], bump),
                    results: BumpVec::new_in(bump),
                })
        }
        "i32.load" | "i32.load8_s" | "i32.load8_u" | "i32.load16_s" | "i32.load16_u" => {
            let at = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| ctx.symbol_table.find_def(immediate.to_ptr().into()))
                .map_or(ValType::I32, |symbol| extract_addr_type(&symbol.green));
            ResolvedSig {
                params: BumpVec::from_iter_in([OperandType::Val(at)], bump),
                results: BumpVec::from_iter_in([OperandType::Val(ValType::I32)], bump),
            }
        }
        "i64.load" | "i64.load8_s" | "i64.load8_u" | "i64.load16_s" | "i64.load16_u" | "i64.load32_s"
        | "i64.load32_u" => {
            let at = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| ctx.symbol_table.find_def(immediate.to_ptr().into()))
                .map_or(ValType::I32, |symbol| extract_addr_type(&symbol.green));
            ResolvedSig {
                params: BumpVec::from_iter_in([OperandType::Val(at)], bump),
                results: BumpVec::from_iter_in([OperandType::Val(ValType::I64)], bump),
            }
        }
        "f32.load" => {
            let at = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| ctx.symbol_table.find_def(immediate.to_ptr().into()))
                .map_or(ValType::I32, |symbol| extract_addr_type(&symbol.green));
            ResolvedSig {
                params: BumpVec::from_iter_in([OperandType::Val(at)], bump),
                results: BumpVec::from_iter_in([OperandType::Val(ValType::F32)], bump),
            }
        }
        "f64.load" => {
            let at = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| ctx.symbol_table.find_def(immediate.to_ptr().into()))
                .map_or(ValType::I32, |symbol| extract_addr_type(&symbol.green));
            ResolvedSig {
                params: BumpVec::from_iter_in([OperandType::Val(at)], bump),
                results: BumpVec::from_iter_in([OperandType::Val(ValType::F64)], bump),
            }
        }
        "i32.store" | "i32.store8" | "i32.store16" => {
            let at = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| ctx.symbol_table.find_def(immediate.to_ptr().into()))
                .map_or(ValType::I32, |symbol| extract_addr_type(&symbol.green));
            ResolvedSig {
                params: BumpVec::from_iter_in([OperandType::Val(at), OperandType::Val(ValType::I32)], bump),
                results: BumpVec::new_in(bump),
            }
        }
        "i64.store" | "i64.store8" | "i64.store16" | "i64.store32" => {
            let at = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| ctx.symbol_table.find_def(immediate.to_ptr().into()))
                .map_or(ValType::I32, |symbol| extract_addr_type(&symbol.green));
            ResolvedSig {
                params: BumpVec::from_iter_in([OperandType::Val(at), OperandType::Val(ValType::I64)], bump),
                results: BumpVec::new_in(bump),
            }
        }
        "f32.store" => {
            let at = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| ctx.symbol_table.find_def(immediate.to_ptr().into()))
                .map_or(ValType::I32, |symbol| extract_addr_type(&symbol.green));
            ResolvedSig {
                params: BumpVec::from_iter_in([OperandType::Val(at), OperandType::Val(ValType::F32)], bump),
                results: BumpVec::new_in(bump),
            }
        }
        "f64.store" => {
            let at = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| ctx.symbol_table.find_def(immediate.to_ptr().into()))
                .map_or(ValType::I32, |symbol| extract_addr_type(&symbol.green));
            ResolvedSig {
                params: BumpVec::from_iter_in([OperandType::Val(at), OperandType::Val(ValType::F64)], bump),
                results: BumpVec::new_in(bump),
            }
        }
        "memory.size" | "table.size" => {
            let at = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| ctx.symbol_table.find_def(immediate.to_ptr().into()))
                .map_or(ValType::I32, |symbol| extract_addr_type(&symbol.green));
            ResolvedSig {
                params: BumpVec::new_in(bump),
                results: BumpVec::from_iter_in([OperandType::Val(at)], bump),
            }
        }
        "memory.grow" => {
            let at = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| ctx.symbol_table.find_def(immediate.to_ptr().into()))
                .map_or(ValType::I32, |symbol| extract_addr_type(&symbol.green));
            ResolvedSig {
                params: BumpVec::from_iter_in([OperandType::Val(at.clone())], bump),
                results: BumpVec::from_iter_in([OperandType::Val(at)], bump),
            }
        }
        "memory.init" | "table.init" => {
            let at = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| ctx.symbol_table.find_def(immediate.to_ptr().into()))
                .map_or(ValType::I32, |symbol| extract_addr_type(&symbol.green));
            ResolvedSig {
                params: BumpVec::from_iter_in(
                    [
                        OperandType::Val(at),
                        OperandType::Val(ValType::I32),
                        OperandType::Val(ValType::I32),
                    ],
                    bump,
                ),
                results: BumpVec::new_in(bump),
            }
        }
        "memory.copy" | "table.copy" => {
            let mut immediates = instr.children_by_kind(SyntaxKind::IMMEDIATE);
            let at1 = immediates
                .next()
                .and_then(|immediate| ctx.symbol_table.find_def(immediate.to_ptr().into()))
                .map_or(ValType::I32, |symbol| extract_addr_type(&symbol.green));
            let at2 = immediates
                .next()
                .and_then(|immediate| ctx.symbol_table.find_def(immediate.to_ptr().into()))
                .map_or(ValType::I32, |symbol| extract_addr_type(&symbol.green));
            // i32 is less than i64, so if either is i32, the min is i32. Otherwise, it's i64.
            let min = if at1 == ValType::I32 || at2 == ValType::I32 {
                ValType::I32
            } else {
                ValType::I64
            };
            ResolvedSig {
                params: BumpVec::from_iter_in(
                    [OperandType::Val(at1), OperandType::Val(at2), OperandType::Val(min)],
                    bump,
                ),
                results: BumpVec::new_in(bump),
            }
        }
        "memory.fill" => {
            let at = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| ctx.symbol_table.find_def(immediate.to_ptr().into()))
                .map_or(ValType::I32, |symbol| extract_addr_type(&symbol.green));
            ResolvedSig {
                params: BumpVec::from_iter_in(
                    [
                        OperandType::Val(at.clone()),
                        OperandType::Val(ValType::I32),
                        OperandType::Val(at),
                    ],
                    bump,
                ),
                results: BumpVec::new_in(bump),
            }
        }
        "v128.load" | "v128.load8x8_s" | "v128.load8x8_u" | "v128.load16x4_s" | "v128.load16x4_u"
        | "v128.load32x2_s" | "v128.load32x2_u" | "v128.load8_splat" | "v128.load16_splat" | "v128.load32_splat"
        | "v128.load64_splat" | "v128.load32_zero" | "v128.load64_zero" => {
            let at = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| ctx.symbol_table.find_def(immediate.to_ptr().into()))
                .map_or(ValType::I32, |symbol| extract_addr_type(&symbol.green));
            ResolvedSig {
                params: BumpVec::from_iter_in([OperandType::Val(at)], bump),
                results: BumpVec::from_iter_in([OperandType::Val(ValType::V128)], bump),
            }
        }
        "v128.store" | "v128.store8_lane" | "v128.store16_lane" | "v128.store32_lane" | "v128.store64_lane" => {
            let at = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| ctx.symbol_table.find_def(immediate.to_ptr().into()))
                .map_or(ValType::I32, |symbol| extract_addr_type(&symbol.green));
            ResolvedSig {
                params: BumpVec::from_iter_in([OperandType::Val(at), OperandType::Val(ValType::V128)], bump),
                results: BumpVec::new_in(bump),
            }
        }
        "v128.load8_lane" | "v128.load16_lane" | "v128.load32_lane" | "v128.load64_lane" => {
            let at = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| ctx.symbol_table.find_def(immediate.to_ptr().into()))
                .map_or(ValType::I32, |symbol| extract_addr_type(&symbol.green));
            ResolvedSig {
                params: BumpVec::from_iter_in([OperandType::Val(at), OperandType::Val(ValType::V128)], bump),
                results: BumpVec::from_iter_in([OperandType::Val(ValType::V128)], bump),
            }
        }
        "table.get" => instr
            .children_by_kind(SyntaxKind::IMMEDIATE)
            .next()
            .and_then(|immediate| ctx.symbol_table.find_def(immediate.to_ptr().into()))
            .map(|symbol| {
                let at = extract_addr_type(&symbol.green);
                let ref_type = extract_table_ref_type(ctx.db, &symbol.green)
                    .map_or(OperandType::Any, |ref_type| OperandType::Val(ValType::Ref(ref_type)));
                ResolvedSig {
                    params: BumpVec::from_iter_in([OperandType::Val(at)], bump),
                    results: BumpVec::from_iter_in([ref_type], bump),
                }
            })
            .unwrap_or_else(|| ResolvedSig {
                params: BumpVec::from_iter_in([OperandType::Val(ValType::I32)], bump),
                results: BumpVec::from_iter_in([OperandType::Any], bump),
            }),
        "table.set" => instr
            .children_by_kind(SyntaxKind::IMMEDIATE)
            .next()
            .and_then(|immediate| ctx.symbol_table.find_def(immediate.to_ptr().into()))
            .map(|symbol| {
                let at = extract_addr_type(&symbol.green);
                let ref_type = extract_table_ref_type(ctx.db, &symbol.green)
                    .map_or(OperandType::Any, |ref_type| OperandType::Val(ValType::Ref(ref_type)));
                ResolvedSig {
                    params: BumpVec::from_iter_in([OperandType::Val(at), ref_type], bump),
                    results: BumpVec::new_in(bump),
                }
            })
            .unwrap_or_else(|| ResolvedSig {
                params: BumpVec::from_iter_in([OperandType::Val(ValType::I32), OperandType::Any], bump),
                results: BumpVec::new_in(bump),
            }),
        "table.grow" => instr
            .children_by_kind(SyntaxKind::IMMEDIATE)
            .next()
            .and_then(|immediate| ctx.symbol_table.find_def(immediate.to_ptr().into()))
            .map(|symbol| {
                let at = extract_addr_type(&symbol.green);
                let ref_type = extract_table_ref_type(ctx.db, &symbol.green)
                    .map_or(OperandType::Any, |ref_type| OperandType::Val(ValType::Ref(ref_type)));
                ResolvedSig {
                    params: BumpVec::from_iter_in([ref_type, OperandType::Val(at.clone())], bump),
                    results: BumpVec::from_iter_in([OperandType::Val(at)], bump),
                }
            })
            .unwrap_or_else(|| ResolvedSig {
                params: BumpVec::from_iter_in([OperandType::Any, OperandType::Val(ValType::I32)], bump),
                results: BumpVec::from_iter_in([OperandType::Val(ValType::I32)], bump),
            }),
        "table.fill" => instr
            .children_by_kind(SyntaxKind::IMMEDIATE)
            .next()
            .and_then(|immediate| ctx.symbol_table.find_def(immediate.to_ptr().into()))
            .map(|symbol| {
                let at = extract_addr_type(&symbol.green);
                let ref_type = extract_table_ref_type(ctx.db, &symbol.green)
                    .map_or(OperandType::Any, |ref_type| OperandType::Val(ValType::Ref(ref_type)));
                ResolvedSig {
                    params: BumpVec::from_iter_in([OperandType::Val(at.clone()), ref_type, OperandType::Val(at)], bump),
                    results: BumpVec::new_in(bump),
                }
            })
            .unwrap_or_else(|| ResolvedSig {
                params: BumpVec::from_iter_in(
                    [
                        OperandType::Val(ValType::I32),
                        OperandType::Any,
                        OperandType::Val(ValType::I32),
                    ],
                    bump,
                ),
                results: BumpVec::new_in(bump),
            }),
        _ => data_set::INSTR_SIG
            .get(instr_name)
            .map(|sig| ResolvedSig {
                params: BumpVec::from_iter_in(sig.params.iter().cloned(), bump),
                results: BumpVec::from_iter_in(sig.results.iter().cloned(), bump),
            })
            .unwrap_or_else(|| ResolvedSig::new_in(bump)),
    }
}

pub(crate) struct InstrSigResolverCtx<'db, 'bump> {
    pub db: &'db dyn salsa::Database,
    pub document: Document,
    pub symbol_table: &'db SymbolTable<'db>,
    pub def_types: &'db DefTypes<'db>,
    pub module: &'db SyntaxNode,
    pub module_id: u32,
    pub bump: &'bump Bump,
}

pub(crate) struct ResolvedSig<'db, 'bump> {
    pub params: BumpVec<'bump, OperandType<'db>>,
    pub results: BumpVec<'bump, OperandType<'db>>,
}
impl<'db, 'bump> ResolvedSig<'db, 'bump> {
    fn new_in(bump: &'bump Bump) -> Self {
        Self {
            params: BumpVec::new_in(bump),
            results: BumpVec::new_in(bump),
        }
    }
    fn from_sig_in(sig: Sig<'db>, bump: &'bump Bump) -> Self {
        Self {
            params: BumpVec::from_iter_in(sig.params.into_iter().map(OperandType::Val), bump),
            results: BumpVec::from_iter_in(sig.results.into_iter().map(OperandType::Val), bump),
        }
    }
    fn from_named_sig_in(sig: NamedSig<'db>, bump: &'bump Bump) -> Self {
        Self {
            params: BumpVec::from_iter_in(sig.params.into_iter().map(|(ty, _)| OperandType::Val(ty)), bump),
            results: BumpVec::from_iter_in(sig.results.into_iter().map(OperandType::Val), bump),
        }
    }
}
