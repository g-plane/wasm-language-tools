use super::{Diagnostic, RelatedInformation};
use crate::{
    binder::{SymbolKey, SymbolTable},
    data_set,
    document::Document,
    helpers,
    idx::{Idx, InternIdent},
    imex,
    types_analyzer::{
        CompositeType, HeapType, OperandType, RefType, Signature, ValType, extract_global_type, extract_type,
        get_def_types, get_func_sig, get_type_use_sig, join_types, resolve_array_type_with_idx, resolve_br_types,
        resolve_field_type_with_struct_idx,
    },
};
use itertools::{EitherOrBoth, Itertools};
use oxc_allocator::{Allocator, Vec as OxcVec};
use rowan::{
    TextRange,
    ast::{AstNode, support},
};
use std::iter;
use wat_syntax::{
    SyntaxElement, SyntaxKind, SyntaxNode, SyntaxNodePtr,
    ast::{BlockInstr, ElemList, Instr, ModuleFieldFunc, ModuleFieldTable, PlainInstr},
};

const DIAGNOSTIC_CODE: &str = "type-check";

pub fn check_func(
    diagnostics: &mut Vec<Diagnostic>,
    db: &dyn salsa::Database,
    document: Document,
    symbol_table: &SymbolTable,
    module_id: u32,
    node: &SyntaxNode,
    allocator: &mut Allocator,
) {
    let results = OxcVec::from_iter_in(
        get_func_sig(db, document, SymbolKey::new(node), &node.green())
            .results
            .into_iter()
            .map(OperandType::Val),
        allocator,
    );
    check_block_like(
        diagnostics,
        &Shared {
            db,
            document,
            symbol_table,
            module_id,
            allocator,
        },
        node,
        if imex::get_imports(db, document).contains(&SymbolKey::new(node)) {
            results.iter().map(|ty| (ty.clone(), None)).collect()
        } else {
            Vec::with_capacity(2)
        },
        &results,
    );
    allocator.reset();
}

pub fn check_global(
    diagnostics: &mut Vec<Diagnostic>,
    db: &dyn salsa::Database,
    document: Document,
    symbol_table: &SymbolTable,
    module_id: u32,
    node: &SyntaxNode,
    allocator: &mut Allocator,
) {
    let ty = extract_global_type(db, document, node.green().into())
        .map(OperandType::Val)
        .unwrap_or(OperandType::Any);
    check_block_like(
        diagnostics,
        &Shared {
            db,
            document,
            symbol_table,
            module_id,
            allocator,
        },
        node,
        if imex::get_imports(db, document).contains(&SymbolKey::new(node)) {
            vec![(ty.clone(), None)]
        } else {
            Vec::with_capacity(1)
        },
        &[ty],
    );
    allocator.reset();
}

pub fn check_table(
    diagnostics: &mut Vec<Diagnostic>,
    db: &dyn salsa::Database,
    document: Document,
    symbol_table: &SymbolTable,
    module_id: u32,
    node: &SyntaxNode,
    allocator: &mut Allocator,
) {
    let Some(ref_type) = ModuleFieldTable::cast(node.clone())
        .and_then(|table| {
            table
                .ref_type()
                .or_else(|| table.table_type().and_then(|table_type| table_type.ref_type()))
        })
        .and_then(|ref_type| RefType::from_green(&ref_type.syntax().green(), db))
    else {
        return;
    };
    let ty = ValType::Ref(ref_type);
    if ty.defaultable() && !node.children().any(|child| Instr::can_cast(child.kind())) {
        return;
    }
    let ty = OperandType::Val(ty);
    check_block_like(
        diagnostics,
        &Shared {
            db,
            document,
            symbol_table,
            module_id,
            allocator,
        },
        node,
        if imex::get_imports(db, document).contains(&SymbolKey::new(node)) {
            vec![(ty.clone(), None)]
        } else {
            Vec::with_capacity(1)
        },
        &[ty],
    );
    allocator.reset();
}

pub fn check_offset(
    diagnostics: &mut Vec<Diagnostic>,
    db: &dyn salsa::Database,
    document: Document,
    symbol_table: &SymbolTable,
    module_id: u32,
    node: &SyntaxNode,
    allocator: &mut Allocator,
) {
    check_block_like(
        diagnostics,
        &Shared {
            db,
            document,
            symbol_table,
            module_id,
            allocator,
        },
        node,
        Vec::with_capacity(1),
        &[OperandType::Val(ValType::I32)],
    );
    allocator.reset();
}

pub fn check_elem_list(
    diagnostics: &mut Vec<Diagnostic>,
    db: &dyn salsa::Database,
    document: Document,
    symbol_table: &SymbolTable,
    module_id: u32,
    node: &SyntaxNode,
    allocator: &mut Allocator,
) {
    let Some(ref_type) = ElemList::cast(node.clone())
        .and_then(|elem_list| elem_list.ref_type())
        .and_then(|ref_type| RefType::from_green(&ref_type.syntax().green(), db))
    else {
        return;
    };
    let ty = OperandType::Val(ValType::Ref(ref_type));
    node.children()
        .filter(|child| child.kind() == SyntaxKind::ELEM_EXPR)
        .for_each(|child| {
            check_block_like(
                diagnostics,
                &Shared {
                    db,
                    document,
                    symbol_table,
                    module_id,
                    allocator,
                },
                &child,
                Vec::with_capacity(1),
                std::slice::from_ref(&ty),
            );
        });
    allocator.reset();
}

struct Shared<'db, 'alloc> {
    db: &'db dyn salsa::Database,
    document: Document,
    symbol_table: &'db SymbolTable<'db>,
    module_id: u32,
    allocator: &'alloc Allocator,
}

fn check_block_like(
    diagnostics: &mut Vec<Diagnostic>,
    shared: &Shared,
    node: &SyntaxNode,
    init_stack: Vec<(OperandType, Option<Instr>)>,
    expected_results: &[OperandType],
) {
    let mut type_stack = TypeStack {
        shared,
        stack: init_stack,
        has_never: false,
    };

    fn unfold<'db, 'alloc>(
        node: SyntaxNode,
        type_stack: &mut TypeStack<'db, 'alloc>,
        diagnostics: &mut Vec<Diagnostic>,
        shared: &Shared<'db, 'alloc>,
    ) {
        if matches!(node.kind(), SyntaxKind::PLAIN_INSTR | SyntaxKind::BLOCK_IF) {
            node.children()
                .filter_map(Instr::cast)
                .for_each(|child| unfold(child.syntax().clone(), type_stack, diagnostics, shared));
        }
        if let Some(node) = Instr::cast(node) {
            check_instr(node, type_stack, diagnostics, shared);
        }
    }
    node.children()
        .filter(|child| Instr::can_cast(child.kind()))
        .for_each(|child| unfold(child, &mut type_stack, diagnostics, shared));

    if let Some(diagnostic) = type_stack.check_to_bottom(expected_results, ReportRange::Last(node)) {
        diagnostics.push(diagnostic);
    }
}

fn check_instr<'db, 'alloc>(
    instr: Instr,
    type_stack: &mut TypeStack<'db, 'alloc>,
    diagnostics: &mut Vec<Diagnostic>,
    shared: &Shared<'db, 'alloc>,
) {
    match &instr {
        Instr::Plain(plain_instr) => {
            let Some(instr_name) = plain_instr.instr_name() else {
                return;
            };
            let instr_name = instr_name.text();
            let sig = resolve_sig(shared, instr_name, plain_instr, type_stack);
            if let Some(diagnostic) = type_stack.check(&sig.params, ReportRange::Instr(&instr)) {
                diagnostics.push(diagnostic);
            }
            if helpers::is_stack_polymorphic(instr_name) {
                type_stack.has_never = true;
                type_stack.stack.clear();
            }
            type_stack
                .stack
                .extend(sig.results.into_iter().map(|ty| (ty, Some(instr.clone()))));
        }
        Instr::Block(block_instr) => {
            let node = block_instr.syntax();
            let signature = get_func_sig(shared.db, shared.document, SymbolKey::new(node), &node.green());
            let init_stack = signature
                .params
                .iter()
                .map(|(ty, ..)| (OperandType::Val(ty.clone()), Some(instr.clone())))
                .collect();
            let results = OxcVec::from_iter_in(signature.results.into_iter().map(OperandType::Val), shared.allocator);
            match block_instr {
                BlockInstr::Block(..) | BlockInstr::Loop(..) | BlockInstr::TryTable(..) => {
                    if let Some(diagnostic) = type_stack.check(
                        &OxcVec::from_iter_in(
                            signature.params.into_iter().map(|(ty, _)| OperandType::Val(ty)),
                            shared.allocator,
                        ),
                        ReportRange::Instr(&instr),
                    ) {
                        diagnostics.push(diagnostic);
                    }
                    check_block_like(diagnostics, shared, node, init_stack, &results);
                }
                BlockInstr::If(block_if) => {
                    if let Some(mut diagnostic) =
                        type_stack.check(&[OperandType::Val(ValType::I32)], ReportRange::Keyword(node))
                    {
                        diagnostic.message.push_str(" for the condition of `if` block");
                        diagnostics.push(diagnostic);
                    }
                    if let Some(diagnostic) = type_stack.check(
                        &OxcVec::from_iter_in(
                            signature.params.into_iter().map(|(ty, _)| OperandType::Val(ty)),
                            shared.allocator,
                        ),
                        ReportRange::Instr(&instr),
                    ) {
                        diagnostics.push(diagnostic);
                    }
                    if let Some(then_block) = block_if.then_block() {
                        check_block_like(diagnostics, shared, then_block.syntax(), init_stack.clone(), &results);
                    } else {
                        diagnostics.push(Diagnostic {
                            range: node.text_range(),
                            code: DIAGNOSTIC_CODE.into(),
                            message: format!(
                                "missing `then` branch with expected types {}",
                                join_types(shared.db, results.iter(), "", shared.allocator)
                            ),
                            ..Default::default()
                        });
                    }
                    if let Some(else_block) = block_if.else_block() {
                        check_block_like(diagnostics, shared, else_block.syntax(), init_stack, &results);
                    } else {
                        let mut type_stack = TypeStack {
                            shared,
                            stack: init_stack,
                            has_never: false,
                        };
                        if type_stack
                            .check_to_bottom(&results, ReportRange::Instr(&instr))
                            .is_some()
                        {
                            diagnostics.push(Diagnostic {
                                range: node.text_range(),
                                code: DIAGNOSTIC_CODE.into(),
                                message: format!(
                                    "missing `else` branch with expected types {}",
                                    join_types(shared.db, results.iter(), "", shared.allocator)
                                ),
                                ..Default::default()
                            });
                        }
                    }
                }
            }
            type_stack
                .stack
                .extend(results.into_iter().map(|ty| (ty, Some(instr.clone()))));
        }
    }
}

struct TypeStack<'db, 'alloc> {
    shared: &'alloc Shared<'db, 'alloc>,
    stack: Vec<(OperandType<'db>, Option<Instr>)>,
    has_never: bool,
}
impl<'db, 'alloc> TypeStack<'db, 'alloc> {
    fn check(&mut self, expected: &[OperandType<'db>], report_range: ReportRange) -> Option<Diagnostic> {
        let mut diagnostic = None;
        let rest_len = self.stack.len().saturating_sub(expected.len());
        let pops = self.stack.get(rest_len..).unwrap_or(&*self.stack);
        let mut mismatch = false;
        let mut related_information = vec![];
        expected
            .iter()
            .rev()
            .zip_longest(pops.iter().rev())
            .for_each(|pair| match pair {
                EitherOrBoth::Both(expected, (received, related_instr)) => {
                    if received.matches(expected, self.shared.db, self.shared.document, self.shared.module_id) {
                        return;
                    }
                    mismatch = true;
                    if let Some(related_instr) = related_instr {
                        related_information.push(RelatedInformation {
                            range: ReportRange::Instr(related_instr).pick(),
                            message: format!(
                                "expected type `{}`, found `{}`",
                                expected.render(self.shared.db),
                                received.render(self.shared.db),
                            ),
                        });
                    }
                }
                EitherOrBoth::Left(..) if !self.has_never => {
                    mismatch = true;
                }
                _ => {}
            });
        if mismatch {
            diagnostic = Some(Diagnostic {
                range: report_range.pick(),
                code: DIAGNOSTIC_CODE.into(),
                message: format!(
                    "expected types {}, found {}",
                    join_types(self.shared.db, expected.iter(), "", self.shared.allocator),
                    join_types(
                        self.shared.db,
                        pops.iter().map(|(ty, _)| ty),
                        if self.stack.len() > pops.len() { "... " } else { "" },
                        self.shared.allocator,
                    ),
                ),
                related_information: if related_information.is_empty() {
                    None
                } else {
                    Some(related_information)
                },
                ..Default::default()
            });
        }
        self.stack.truncate(rest_len);
        diagnostic
    }

    fn check_to_bottom(&mut self, expected: &[OperandType<'db>], report_range: ReportRange) -> Option<Diagnostic> {
        let mut mismatch = false;
        let mut related_information = vec![];
        expected
            .iter()
            .rev()
            .zip_longest(self.stack.iter().rev())
            .for_each(|pair| match pair {
                EitherOrBoth::Both(expected, (received, related_instr)) => {
                    if received.matches(expected, self.shared.db, self.shared.document, self.shared.module_id) {
                        return;
                    }
                    mismatch = true;
                    if let Some(related_instr) = related_instr {
                        related_information.push(RelatedInformation {
                            range: ReportRange::Instr(related_instr).pick(),
                            message: format!(
                                "expected type `{}`, found `{}`",
                                expected.render(self.shared.db),
                                received.render(self.shared.db),
                            ),
                        });
                    }
                }
                EitherOrBoth::Left(..) if !self.has_never => {
                    mismatch = true;
                }
                EitherOrBoth::Right(..) => {
                    mismatch = true;
                }
                _ => {}
            });
        if mismatch {
            Some(Diagnostic {
                range: report_range.pick(),
                code: DIAGNOSTIC_CODE.into(),
                message: format!(
                    "expected types {}, found {}{}",
                    join_types(self.shared.db, expected.iter(), "", self.shared.allocator),
                    join_types(
                        self.shared.db,
                        self.stack.iter().map(|(ty, _)| ty),
                        "",
                        self.shared.allocator,
                    ),
                    if let ReportRange::Last(..) = report_range {
                        " at the end"
                    } else {
                        ""
                    },
                ),
                related_information: if related_information.is_empty() {
                    None
                } else {
                    Some(related_information)
                },
                data: if expected.is_empty() {
                    self.stack
                        .iter()
                        .map(|(ty, _)| match ty {
                            OperandType::Val(ty) => {
                                Some(serde_json::Value::String(ty.render(self.shared.db).to_string()))
                            }
                            OperandType::Any => None,
                        })
                        .collect::<Option<Vec<_>>>()
                        .map(serde_json::Value::Array)
                } else {
                    None
                },
                ..Default::default()
            })
        } else {
            None
        }
    }
}

struct ResolvedSig<'db, 'alloc> {
    params: OxcVec<'alloc, OperandType<'db>>,
    results: OxcVec<'alloc, OperandType<'db>>,
}
impl<'db, 'alloc> ResolvedSig<'db, 'alloc> {
    fn new_in(allocator: &'alloc Allocator) -> Self {
        Self {
            params: OxcVec::new_in(allocator),
            results: OxcVec::new_in(allocator),
        }
    }
    fn from_func_sig_in(sig: Signature<'db>, allocator: &'alloc Allocator) -> Self {
        Self {
            params: OxcVec::from_iter_in(sig.params.into_iter().map(|(ty, _)| OperandType::Val(ty)), allocator),
            results: OxcVec::from_iter_in(sig.results.into_iter().map(OperandType::Val), allocator),
        }
    }
}
fn resolve_sig<'db, 'alloc>(
    shared: &Shared<'db, 'alloc>,
    instr_name: &str,
    instr: &PlainInstr,
    type_stack: &TypeStack<'db, 'alloc>,
) -> ResolvedSig<'db, 'alloc> {
    let allocator = shared.allocator;
    match instr_name {
        "call" | "return_call" | "throw" => instr
            .immediates()
            .next()
            .and_then(|idx| shared.symbol_table.find_def(SymbolKey::new(idx.syntax())))
            .map(|func| {
                ResolvedSig::from_func_sig_in(
                    get_func_sig(shared.db, shared.document, func.key, &func.green),
                    allocator,
                )
            })
            .unwrap_or_else(|| ResolvedSig::new_in(allocator)),
        "local.get" => ResolvedSig {
            params: OxcVec::new_in(allocator),
            results: OxcVec::from_array_in(
                [instr
                    .immediates()
                    .next()
                    .and_then(|idx| shared.symbol_table.find_def(SymbolKey::new(idx.syntax())))
                    .and_then(|symbol| extract_type(shared.db, shared.document, symbol.green.clone()))
                    .map_or(OperandType::Any, OperandType::Val)],
                allocator,
            ),
        },
        "local.set" => ResolvedSig {
            params: OxcVec::from_array_in(
                [instr
                    .immediates()
                    .next()
                    .and_then(|idx| shared.symbol_table.find_def(SymbolKey::new(idx.syntax())))
                    .and_then(|symbol| extract_type(shared.db, shared.document, symbol.green.clone()))
                    .map_or(OperandType::Any, OperandType::Val)],
                allocator,
            ),
            results: OxcVec::new_in(allocator),
        },
        "local.tee" => {
            let ty = instr
                .immediates()
                .next()
                .and_then(|idx| shared.symbol_table.find_def(SymbolKey::new(idx.syntax())))
                .and_then(|symbol| extract_type(shared.db, shared.document, symbol.green.clone()))
                .map_or(OperandType::Any, OperandType::Val);
            ResolvedSig {
                params: OxcVec::from_array_in([ty.clone()], allocator),
                results: OxcVec::from_array_in([ty], allocator),
            }
        }
        "global.get" => ResolvedSig {
            params: OxcVec::new_in(allocator),
            results: OxcVec::from_array_in(
                [instr
                    .immediates()
                    .next()
                    .and_then(|idx| shared.symbol_table.find_def(SymbolKey::new(idx.syntax())))
                    .and_then(|symbol| extract_global_type(shared.db, shared.document, symbol.green.clone()))
                    .map_or(OperandType::Any, OperandType::Val)],
                allocator,
            ),
        },
        "global.set" => ResolvedSig {
            params: OxcVec::from_array_in(
                [instr
                    .immediates()
                    .next()
                    .and_then(|idx| shared.symbol_table.find_def(SymbolKey::new(idx.syntax())))
                    .and_then(|symbol| extract_global_type(shared.db, shared.document, symbol.green.clone()))
                    .map_or(OperandType::Any, OperandType::Val)],
                allocator,
            ),
            results: OxcVec::new_in(allocator),
        },
        "i32.const" => ResolvedSig {
            params: OxcVec::new_in(allocator),
            results: OxcVec::from_array_in([OperandType::Val(ValType::I32)], allocator),
        },
        "i64.const" => ResolvedSig {
            params: OxcVec::new_in(allocator),
            results: OxcVec::from_array_in([OperandType::Val(ValType::I64)], allocator),
        },
        "f32.const" => ResolvedSig {
            params: OxcVec::new_in(allocator),
            results: OxcVec::from_array_in([OperandType::Val(ValType::F32)], allocator),
        },
        "f64.const" => ResolvedSig {
            params: OxcVec::new_in(allocator),
            results: OxcVec::from_array_in([OperandType::Val(ValType::F64)], allocator),
        },
        "return" => ResolvedSig {
            params: instr
                .syntax()
                .ancestors()
                .find(|node| node.kind() == SyntaxKind::MODULE_FIELD_FUNC)
                .map(|func| {
                    OxcVec::from_iter_in(
                        get_func_sig(shared.db, shared.document, SymbolKey::new(&func), &func.green())
                            .results
                            .into_iter()
                            .map(OperandType::Val),
                        allocator,
                    )
                })
                .unwrap_or_else(|| OxcVec::new_in(allocator)),
            results: OxcVec::new_in(allocator),
        },
        "br" => ResolvedSig {
            params: instr
                .immediates()
                .next()
                .and_then(|idx| {
                    resolve_br_types(
                        shared.db,
                        shared.document,
                        shared.symbol_table,
                        SymbolKey::new(idx.syntax()),
                    )
                })
                .map(|types| OxcVec::from_iter_in(types, allocator))
                .unwrap_or_else(|| OxcVec::new_in(allocator)),
            results: OxcVec::new_in(allocator),
        },
        "br_if" => {
            let results = instr
                .immediates()
                .next()
                .and_then(|idx| {
                    resolve_br_types(
                        shared.db,
                        shared.document,
                        shared.symbol_table,
                        SymbolKey::new(idx.syntax()),
                    )
                })
                .map(|types| OxcVec::from_iter_in(types, allocator))
                .unwrap_or_else(|| OxcVec::new_in(allocator));
            let params = OxcVec::from_iter_in(
                results
                    .iter()
                    .cloned()
                    .chain(iter::once(OperandType::Val(ValType::I32))),
                allocator,
            );
            ResolvedSig { params, results }
        }
        "br_table" => {
            let mut params = instr
                .immediates()
                .next()
                .and_then(|idx| {
                    resolve_br_types(
                        shared.db,
                        shared.document,
                        shared.symbol_table,
                        SymbolKey::new(idx.syntax()),
                    )
                })
                .map(|types| OxcVec::from_iter_in(types, allocator))
                .unwrap_or_else(|| OxcVec::new_in(allocator));
            params.push(OperandType::Val(ValType::I32));
            ResolvedSig {
                params,
                results: OxcVec::new_in(allocator),
            }
        }
        "br_on_null" => {
            let heap_ty =
                if let Some((OperandType::Val(ValType::Ref(RefType { heap_ty, .. })), _)) = type_stack.stack.last() {
                    heap_ty.clone()
                } else {
                    HeapType::Any
                };
            let mut results = instr
                .immediates()
                .next()
                .and_then(|idx| {
                    resolve_br_types(
                        shared.db,
                        shared.document,
                        shared.symbol_table,
                        SymbolKey::new(idx.syntax()),
                    )
                })
                .map(|types| OxcVec::from_iter_in(types, allocator))
                .unwrap_or_else(|| OxcVec::new_in(allocator));
            let params = OxcVec::from_iter_in(
                results
                    .iter()
                    .cloned()
                    .chain(iter::once(OperandType::Val(ValType::Ref(RefType {
                        heap_ty: heap_ty.clone(),
                        nullable: true,
                    })))),
                allocator,
            );
            results.push(OperandType::Val(ValType::Ref(RefType {
                heap_ty: heap_ty.clone(),
                nullable: false,
            })));
            ResolvedSig { params, results }
        }
        "br_on_non_null" => {
            let heap_ty =
                if let Some((OperandType::Val(ValType::Ref(RefType { heap_ty, .. })), _)) = type_stack.stack.last() {
                    heap_ty.clone()
                } else {
                    HeapType::Any
                };
            let results = instr
                .immediates()
                .next()
                .and_then(|idx| {
                    resolve_br_types(
                        shared.db,
                        shared.document,
                        shared.symbol_table,
                        SymbolKey::new(idx.syntax()),
                    )
                })
                .map(|types| OxcVec::from_iter_in(types, allocator))
                .unwrap_or_else(|| OxcVec::new_in(allocator));
            let params = OxcVec::from_iter_in(
                results
                    .iter()
                    .cloned()
                    .chain(iter::once(OperandType::Val(ValType::Ref(RefType {
                        heap_ty,
                        nullable: true,
                    })))),
                allocator,
            );
            ResolvedSig { params, results }
        }
        "br_on_cast" => {
            let mut immediates = instr.immediates();
            let mut types = immediates
                .next()
                .and_then(|idx| {
                    resolve_br_types(
                        shared.db,
                        shared.document,
                        shared.symbol_table,
                        SymbolKey::new(idx.syntax()),
                    )
                })
                .map(|types| OxcVec::from_iter_in(types, allocator))
                .unwrap_or_else(|| OxcVec::new_in(allocator));
            types.pop();
            let rt1 = immediates
                .next()
                .and_then(|immediate| immediate.ref_type())
                .and_then(|ref_type| RefType::from_green(&ref_type.syntax().green(), shared.db));
            let rt2 = immediates
                .next()
                .and_then(|immediate| immediate.ref_type())
                .and_then(|ref_type| RefType::from_green(&ref_type.syntax().green(), shared.db));
            let mut params = OxcVec::from_iter_in(types.iter().cloned(), allocator);
            let mut results = types;
            if let Some((rt1, rt2)) = rt1.zip(rt2) {
                params.push(OperandType::Val(ValType::Ref(rt1.clone())));
                results.push(OperandType::Val(ValType::Ref(rt1.diff(&rt2))));
            }
            ResolvedSig { params, results }
        }
        "br_on_cast_fail" => {
            let mut immediates = instr.immediates();
            let mut types = immediates
                .next()
                .and_then(|idx| {
                    resolve_br_types(
                        shared.db,
                        shared.document,
                        shared.symbol_table,
                        SymbolKey::new(idx.syntax()),
                    )
                })
                .map(|types| OxcVec::from_iter_in(types, allocator))
                .unwrap_or_else(|| OxcVec::new_in(allocator));
            types.pop();
            let rt1 = immediates
                .next()
                .and_then(|immediate| immediate.ref_type())
                .and_then(|ref_type| RefType::from_green(&ref_type.syntax().green(), shared.db));
            let rt2 = immediates
                .next()
                .and_then(|immediate| immediate.ref_type())
                .and_then(|ref_type| RefType::from_green(&ref_type.syntax().green(), shared.db));
            let mut params = OxcVec::from_iter_in(types.iter().cloned(), allocator);
            let mut results = types;
            if let Some((rt1, rt2)) = rt1.zip(rt2) {
                params.push(OperandType::Val(ValType::Ref(rt1)));
                results.push(OperandType::Val(ValType::Ref(rt2)));
            }
            ResolvedSig { params, results }
        }
        "select" => {
            let ty = if let Some(ty) = instr
                .immediates()
                .next()
                .and_then(|immediate| immediate.type_use())
                .and_then(|type_use| type_use.results().next())
                .and_then(|result| result.val_types().next())
            {
                ValType::from_ast(&ty, shared.db).map_or(OperandType::Any, OperandType::Val)
            } else {
                type_stack
                    .stack
                    .len()
                    .checked_sub(2)
                    .and_then(|i| type_stack.stack.get(i))
                    .map_or(OperandType::Any, |(ty, _)| ty.clone())
            };
            ResolvedSig {
                params: OxcVec::from_array_in([ty.clone(), ty.clone(), OperandType::Val(ValType::I32)], allocator),
                results: OxcVec::from_array_in([ty], allocator),
            }
        }
        "call_indirect" | "return_call_indirect" => {
            let mut sig = instr
                .immediates()
                .find_map(|immediate| immediate.type_use())
                .map(|type_use| {
                    let node = type_use.syntax();
                    ResolvedSig::from_func_sig_in(
                        get_type_use_sig(shared.db, shared.document, SyntaxNodePtr::new(node), &node.green()),
                        allocator,
                    )
                })
                .unwrap_or_else(|| ResolvedSig::new_in(allocator));
            sig.params.push(OperandType::Val(ValType::I32));
            sig
        }
        "struct.new" => {
            let def_types = get_def_types(shared.db, shared.document);
            instr
                .immediates()
                .next()
                .and_then(|immediate| shared.symbol_table.resolved.get(&SymbolKey::new(immediate.syntax())))
                .and_then(|key| def_types.get(key))
                .map(|def_type| {
                    let params = if let CompositeType::Struct(fields) = &def_type.comp {
                        OxcVec::from_iter_in(
                            fields.0.iter().map(|(field, _)| field.storage.clone().into()),
                            allocator,
                        )
                    } else {
                        OxcVec::new_in(allocator)
                    };
                    ResolvedSig {
                        params,
                        results: OxcVec::from_array_in(
                            [OperandType::Val(ValType::Ref(RefType {
                                heap_ty: HeapType::Type(def_type.idx),
                                nullable: false,
                            }))],
                            allocator,
                        ),
                    }
                })
                .unwrap_or_else(|| ResolvedSig {
                    params: OxcVec::new_in(allocator),
                    results: OxcVec::from_array_in([OperandType::Any], allocator),
                })
        }
        "struct.new_default" => instr
            .immediates()
            .next()
            .and_then(|idx| shared.symbol_table.find_def(SymbolKey::new(idx.syntax())))
            .map(|symbol| ResolvedSig {
                params: OxcVec::new_in(allocator),
                results: OxcVec::from_array_in(
                    [OperandType::Val(ValType::Ref(RefType {
                        heap_ty: HeapType::Type(symbol.idx),
                        nullable: false,
                    }))],
                    allocator,
                ),
            })
            .unwrap_or_else(|| ResolvedSig {
                params: OxcVec::new_in(allocator),
                results: OxcVec::from_array_in([OperandType::Any], allocator),
            }),
        "struct.get" => {
            let mut immediates = instr.immediates();
            immediates
                .next()
                .zip(immediates.next())
                .and_then(|(struct_ref, field_ref)| {
                    resolve_field_type_with_struct_idx(shared.db, shared.document, &struct_ref, &field_ref)
                })
                .map(|(idx, ty)| ResolvedSig {
                    params: OxcVec::from_array_in(
                        [OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(idx),
                            nullable: true,
                        }))],
                        allocator,
                    ),
                    results: OxcVec::from_array_in([ty.unwrap_or(OperandType::Any)], allocator),
                })
                .unwrap_or_else(|| ResolvedSig {
                    params: OxcVec::new_in(allocator),
                    results: OxcVec::from_array_in([OperandType::Any], allocator),
                })
        }
        "struct.get_s" | "struct.get_u" => ResolvedSig {
            params: instr
                .immediates()
                .next()
                .and_then(|immediate| shared.symbol_table.find_def(SymbolKey::new(immediate.syntax())))
                .map(|symbol| {
                    OxcVec::from_array_in(
                        [OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(symbol.idx),
                            nullable: true,
                        }))],
                        allocator,
                    )
                })
                .unwrap_or_else(|| OxcVec::new_in(allocator)),
            results: OxcVec::from_array_in([OperandType::Val(ValType::I32)], allocator),
        },
        "struct.set" => {
            let mut immediates = instr.immediates();
            immediates
                .next()
                .zip(immediates.next())
                .and_then(|(struct_ref, field_ref)| {
                    resolve_field_type_with_struct_idx(shared.db, shared.document, &struct_ref, &field_ref)
                })
                .map(|(idx, ty)| ResolvedSig {
                    params: OxcVec::from_array_in(
                        [
                            OperandType::Val(ValType::Ref(RefType {
                                heap_ty: HeapType::Type(idx),
                                nullable: true,
                            })),
                            ty.unwrap_or(OperandType::Any),
                        ],
                        allocator,
                    ),
                    results: OxcVec::new_in(allocator),
                })
                .unwrap_or_else(|| ResolvedSig {
                    params: OxcVec::from_array_in([OperandType::Any], allocator),
                    results: OxcVec::new_in(allocator),
                })
        }
        "array.new" => {
            let mut sig = instr
                .immediates()
                .next()
                .and_then(|immediate| {
                    let def_types = get_def_types(shared.db, shared.document);
                    resolve_array_type_with_idx(shared.symbol_table, def_types, &immediate)
                })
                .map(|(idx, ty)| ResolvedSig {
                    params: OxcVec::from_array_in([ty.unwrap_or(OperandType::Any)], allocator),
                    results: OxcVec::from_array_in(
                        [OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(idx),
                            nullable: false,
                        }))],
                        allocator,
                    ),
                })
                .unwrap_or_else(|| ResolvedSig {
                    params: OxcVec::new_in(allocator),
                    results: OxcVec::from_array_in([OperandType::Any], allocator),
                });
            sig.params.push(OperandType::Val(ValType::I32));
            sig
        }
        "array.new_default" => instr
            .immediates()
            .next()
            .and_then(|idx| shared.symbol_table.find_def(SymbolKey::new(idx.syntax())))
            .map(|symbol| ResolvedSig {
                params: OxcVec::from_array_in([OperandType::Val(ValType::I32)], allocator),
                results: OxcVec::from_array_in(
                    [OperandType::Val(ValType::Ref(RefType {
                        heap_ty: HeapType::Type(symbol.idx),
                        nullable: false,
                    }))],
                    allocator,
                ),
            })
            .unwrap_or_else(|| ResolvedSig {
                params: OxcVec::new_in(allocator),
                results: OxcVec::from_array_in([OperandType::Any], allocator),
            }),
        "array.new_fixed" => {
            let mut immediates = instr.immediates();
            immediates
                .next()
                .and_then(|immediate| {
                    let def_types = get_def_types(shared.db, shared.document);
                    resolve_array_type_with_idx(shared.symbol_table, def_types, &immediate)
                })
                .map(|(idx, ty)| {
                    let count = immediates
                        .next()
                        .and_then(|immediate| immediate.int())
                        .and_then(|int| int.text().parse().ok())
                        .unwrap_or_default();
                    ResolvedSig {
                        params: OxcVec::from_iter_in(iter::repeat_n(ty.unwrap_or(OperandType::Any), count), allocator),
                        results: OxcVec::from_array_in(
                            [OperandType::Val(ValType::Ref(RefType {
                                heap_ty: HeapType::Type(idx),
                                nullable: false,
                            }))],
                            allocator,
                        ),
                    }
                })
                .unwrap_or_else(|| ResolvedSig {
                    params: OxcVec::new_in(allocator),
                    results: OxcVec::from_array_in([OperandType::Any], allocator),
                })
        }
        "array.new_data" | "array.new_elem" => instr
            .immediates()
            .next()
            .and_then(|idx| shared.symbol_table.find_def(SymbolKey::new(idx.syntax())))
            .map(|symbol| ResolvedSig {
                params: OxcVec::from_iter_in(iter::repeat_n(OperandType::Val(ValType::I32), 2), allocator),
                results: OxcVec::from_array_in(
                    [OperandType::Val(ValType::Ref(RefType {
                        heap_ty: HeapType::Type(symbol.idx),
                        nullable: false,
                    }))],
                    allocator,
                ),
            })
            .unwrap_or_else(|| ResolvedSig {
                params: OxcVec::new_in(allocator),
                results: OxcVec::from_array_in([OperandType::Any], allocator),
            }),
        "array.get" => instr
            .immediates()
            .next()
            .and_then(|immediate| {
                let def_types = get_def_types(shared.db, shared.document);
                resolve_array_type_with_idx(shared.symbol_table, def_types, &immediate)
            })
            .map(|(idx, ty)| ResolvedSig {
                params: OxcVec::from_array_in(
                    [
                        OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(idx),
                            nullable: true,
                        })),
                        OperandType::Val(ValType::I32),
                    ],
                    allocator,
                ),
                results: OxcVec::from_array_in([ty.unwrap_or(OperandType::Any)], allocator),
            })
            .unwrap_or_else(|| ResolvedSig {
                params: OxcVec::new_in(allocator),
                results: OxcVec::from_array_in([OperandType::Any], allocator),
            }),
        "array.get_s" | "array.get_u" => instr
            .immediates()
            .next()
            .and_then(|idx| shared.symbol_table.find_def(SymbolKey::new(idx.syntax())))
            .map(|symbol| ResolvedSig {
                params: OxcVec::from_array_in(
                    [
                        OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(symbol.idx),
                            nullable: true,
                        })),
                        OperandType::Val(ValType::I32),
                    ],
                    allocator,
                ),
                results: OxcVec::from_array_in([OperandType::Val(ValType::I32)], allocator),
            })
            .unwrap_or_else(|| ResolvedSig {
                params: OxcVec::new_in(allocator),
                results: OxcVec::from_array_in([OperandType::Val(ValType::I32)], allocator),
            }),
        "array.set" => instr
            .immediates()
            .next()
            .and_then(|immediate| {
                let def_types = get_def_types(shared.db, shared.document);
                resolve_array_type_with_idx(shared.symbol_table, def_types, &immediate)
            })
            .map(|(idx, ty)| ResolvedSig {
                params: OxcVec::from_array_in(
                    [
                        OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(idx),
                            nullable: true,
                        })),
                        OperandType::Val(ValType::I32),
                        ty.unwrap_or(OperandType::Any),
                    ],
                    allocator,
                ),
                results: OxcVec::new_in(allocator),
            })
            .unwrap_or_else(|| ResolvedSig::new_in(allocator)),
        "array.fill" => instr
            .immediates()
            .next()
            .and_then(|immediate| {
                let def_types = get_def_types(shared.db, shared.document);
                resolve_array_type_with_idx(shared.symbol_table, def_types, &immediate)
            })
            .map(|(idx, ty)| ResolvedSig {
                params: OxcVec::from_array_in(
                    [
                        OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(idx),
                            nullable: true,
                        })),
                        OperandType::Val(ValType::I32),
                        ty.unwrap_or(OperandType::Any),
                        OperandType::Val(ValType::I32),
                    ],
                    allocator,
                ),
                results: OxcVec::new_in(allocator),
            })
            .unwrap_or_else(|| ResolvedSig::new_in(allocator)),
        "array.copy" => {
            let mut immediates = instr.immediates();
            immediates
                .next()
                .and_then(|idx| shared.symbol_table.find_def(SymbolKey::new(idx.syntax())))
                .zip(
                    immediates
                        .next()
                        .and_then(|idx| shared.symbol_table.find_def(SymbolKey::new(idx.syntax()))),
                )
                .map(|(dst, src)| ResolvedSig {
                    params: OxcVec::from_array_in(
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
                        allocator,
                    ),
                    results: OxcVec::new_in(allocator),
                })
                .unwrap_or_else(|| ResolvedSig::new_in(allocator))
        }
        "array.init_data" | "array.init_elem" => instr
            .immediates()
            .next()
            .and_then(|idx| shared.symbol_table.find_def(SymbolKey::new(idx.syntax())))
            .map(|symbol| ResolvedSig {
                params: OxcVec::from_array_in(
                    [
                        OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(symbol.idx),
                            nullable: true,
                        })),
                        OperandType::Val(ValType::I32),
                        OperandType::Val(ValType::I32),
                        OperandType::Val(ValType::I32),
                    ],
                    allocator,
                ),
                results: OxcVec::new_in(allocator),
            })
            .unwrap_or_else(|| ResolvedSig::new_in(allocator)),
        "ref.null" => {
            let ty = instr
                .immediates()
                .next()
                .and_then(|immediate| immediate.syntax().first_child_or_token())
                .and_then(|element| match element {
                    SyntaxElement::Node(node) if node.kind() == SyntaxKind::HEAP_TYPE => {
                        HeapType::from_green(&node.green(), shared.db)
                    }
                    SyntaxElement::Token(token) if token.kind() == SyntaxKind::IDENT => Some(HeapType::Type(Idx {
                        num: None,
                        name: Some(InternIdent::new(shared.db, token.text())),
                    })),
                    SyntaxElement::Token(token) if token.kind() == SyntaxKind::INT => Some(HeapType::Type(Idx {
                        num: token.text().parse().ok(),
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
                params: OxcVec::new_in(allocator),
                results: OxcVec::from_array_in([ty], allocator),
            }
        }
        "ref.is_null" => {
            let heap_ty =
                if let Some((OperandType::Val(ValType::Ref(RefType { heap_ty, .. })), _)) = type_stack.stack.last() {
                    heap_ty.clone()
                } else {
                    HeapType::Any
                };
            ResolvedSig {
                params: OxcVec::from_array_in(
                    [OperandType::Val(ValType::Ref(RefType {
                        heap_ty,
                        nullable: true,
                    }))],
                    allocator,
                ),
                results: OxcVec::from_array_in([OperandType::Val(ValType::I32)], allocator),
            }
        }
        "ref.as_non_null" => {
            let heap_ty =
                if let Some((OperandType::Val(ValType::Ref(RefType { heap_ty, .. })), _)) = type_stack.stack.last() {
                    heap_ty.clone()
                } else {
                    HeapType::Any
                };
            ResolvedSig {
                params: OxcVec::from_array_in(
                    [OperandType::Val(ValType::Ref(RefType {
                        heap_ty: heap_ty.clone(),
                        nullable: true,
                    }))],
                    allocator,
                ),
                results: OxcVec::from_array_in(
                    [OperandType::Val(ValType::Ref(RefType {
                        heap_ty: heap_ty.clone(),
                        nullable: false,
                    }))],
                    allocator,
                ),
            }
        }
        "ref.test" => {
            let heap_ty = instr
                .immediates()
                .next()
                .and_then(|immediate| immediate.ref_type())
                .and_then(|ref_type| RefType::from_green(&ref_type.syntax().green(), shared.db))
                .and_then(|ref_type| {
                    ref_type
                        .heap_ty
                        .to_top_type(shared.db, shared.document, shared.module_id)
                })
                .unwrap_or(HeapType::Any);
            ResolvedSig {
                params: OxcVec::from_array_in(
                    [OperandType::Val(ValType::Ref(RefType {
                        heap_ty,
                        nullable: true,
                    }))],
                    allocator,
                ),
                results: OxcVec::from_array_in([OperandType::Val(ValType::I32)], allocator),
            }
        }
        "ref.cast" => {
            let ref_type = instr
                .immediates()
                .next()
                .and_then(|immediate| immediate.ref_type())
                .and_then(|ref_type| RefType::from_green(&ref_type.syntax().green(), shared.db))
                .unwrap_or(RefType {
                    heap_ty: HeapType::Any,
                    nullable: true,
                });
            let heap_ty = ref_type
                .heap_ty
                .to_top_type(shared.db, shared.document, shared.module_id)
                .unwrap_or(HeapType::Any);
            ResolvedSig {
                params: OxcVec::from_array_in(
                    [OperandType::Val(ValType::Ref(RefType {
                        heap_ty,
                        nullable: true,
                    }))],
                    allocator,
                ),
                results: OxcVec::from_array_in([OperandType::Val(ValType::Ref(ref_type))], allocator),
            }
        }
        "ref.func" => {
            let immediate = instr.immediates().next();
            let heap_ty = immediate
                .as_ref()
                .and_then(|immediate| shared.symbol_table.resolved.get(&SymbolKey::new(immediate.syntax())))
                .and_then(|key| {
                    let root = shared.document.root_tree(shared.db);
                    ModuleFieldFunc::cast(key.to_node(&root))
                })
                .and_then(|func| func.type_use())
                .and_then(|type_use| type_use.index())
                .map(|index| {
                    HeapType::Type(Idx {
                        num: index.unsigned_int_token().and_then(|int| int.text().parse().ok()),
                        name: index
                            .ident_token()
                            .map(|ident| InternIdent::new(shared.db, ident.text())),
                    })
                })
                .or_else(|| immediate.map(|immediate| HeapType::DefFunc(Idx::from_immediate(&immediate, shared.db))));
            ResolvedSig {
                params: OxcVec::new_in(allocator),
                results: OxcVec::from_array_in(
                    [heap_ty.map_or(OperandType::Any, |heap_ty| {
                        OperandType::Val(ValType::Ref(RefType {
                            heap_ty,
                            nullable: false,
                        }))
                    })],
                    allocator,
                ),
            }
        }
        "call_ref" | "return_call_ref" => {
            let def_types = get_def_types(shared.db, shared.document);
            instr
                .immediates()
                .next()
                .and_then(|immediate| shared.symbol_table.resolved.get(&SymbolKey::new(immediate.syntax())))
                .and_then(|key| def_types.get(key))
                .map(|def_type| {
                    let mut sig = def_type
                        .comp
                        .as_func()
                        .map(|sig| ResolvedSig {
                            params: OxcVec::from_iter_in(
                                sig.params.iter().map(|(ty, _)| OperandType::Val(ty.clone())),
                                allocator,
                            ),
                            results: OxcVec::from_iter_in(
                                sig.results.iter().map(|ty| OperandType::Val(ty.clone())),
                                allocator,
                            ),
                        })
                        .unwrap_or_else(|| ResolvedSig::new_in(allocator));
                    sig.params.push(OperandType::Val(ValType::Ref(RefType {
                        heap_ty: HeapType::Type(def_type.idx),
                        nullable: true,
                    })));
                    sig
                })
                .unwrap_or_else(|| ResolvedSig::new_in(allocator))
        }
        _ => data_set::INSTR_SIG
            .get(instr_name)
            .map(|sig| ResolvedSig {
                params: OxcVec::from_iter_in(sig.params.iter().cloned(), allocator),
                results: OxcVec::from_iter_in(sig.results.iter().cloned(), allocator),
            })
            .unwrap_or_else(|| ResolvedSig::new_in(allocator)),
    }
}

enum ReportRange<'a> {
    Instr(&'a Instr),
    Keyword(&'a SyntaxNode),
    Last(&'a SyntaxNode),
}
impl ReportRange<'_> {
    fn pick(&self) -> TextRange {
        match self {
            ReportRange::Instr(instr) => match instr {
                Instr::Plain(plain_instr) => plain_instr.syntax().text_range(),
                Instr::Block(block_instr) => block_instr
                    .syntax()
                    .first_child_by_kind(&|kind| kind == SyntaxKind::TYPE_USE)
                    .map(|type_use| type_use.text_range())
                    .unwrap_or_else(|| block_instr.syntax().text_range()),
            },
            ReportRange::Keyword(node) => support::token(node, SyntaxKind::KEYWORD)
                .map(|token| token.text_range())
                .unwrap_or_else(|| node.text_range()),
            ReportRange::Last(node) => node
                .last_child_or_token()
                .map(|it| it.text_range())
                .unwrap_or_else(|| node.text_range()),
        }
    }
}
