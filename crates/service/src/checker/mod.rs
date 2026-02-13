use crate::{
    binder::{SymbolKind, SymbolTable},
    config::ServiceConfig,
    document::Document,
    helpers::LineIndexExt,
};
use bumpalo::{Bump, collections::Vec as BumpVec};
use lspt::{DiagnosticRelatedInformation, DiagnosticSeverity, DiagnosticTag, Location, Union2};
use std::cmp::Ordering;
use wat_syntax::{SyntaxKind, SyntaxNode, SyntaxNodePtr, TextRange, ast::support};

mod block_type;
mod br_table_branches;
mod catch_type;
mod const_expr;
mod deprecated;
mod dup_names;
mod elem_type;
mod immediates;
mod implicit_module;
mod import_occur;
mod import_with_def;
mod mem_type;
mod multi_modules;
mod multi_starts;
mod mutated_immutable;
mod needless_mut;
mod needless_try_table;
mod new_non_defaultable;
mod packing;
mod shadow;
mod start;
mod subtyping;
mod syntax;
mod tag_type;
mod type_misuse;
mod typeck;
mod undef;
mod uninit;
mod unknown_instr;
mod unreachable;
mod unread;
mod unused;
mod useless_catch;

pub fn check(db: &dyn salsa::Database, document: Document, config: &ServiceConfig) -> Vec<lspt::Diagnostic> {
    let mut bump = Bump::with_capacity(32 * 1024);

    let uri = document.uri(db);
    let line_index = document.line_index(db);
    let root = document.root_tree(db);
    let symbol_table = SymbolTable::of(db, document);

    let mut diagnostics = Vec::with_capacity(4);
    syntax::check(db, &mut diagnostics, document);
    multi_modules::check(&mut diagnostics, config.lint.multi_modules, &root);
    root.children().enumerate().for_each(|(module_id, module)| {
        let module_id = module_id as u32;
        if let Some(diagnostic) = implicit_module::check(config.lint.implicit_module, &module) {
            diagnostics.push(diagnostic);
        }
        let mut descendants = module.descendants();
        while let Some(node) = descendants.next() {
            match node.kind() {
                SyntaxKind::MODULE_FIELD_FUNC => {
                    typeck::check_func(
                        &mut diagnostics,
                        db,
                        document,
                        symbol_table,
                        module_id,
                        &node,
                        &mut bump,
                    );
                    unreachable::check(
                        &mut diagnostics,
                        db,
                        document,
                        config.lint.unreachable,
                        &root,
                        &node,
                        &mut bump,
                    );
                    let locals = symbol_table
                        .symbols
                        .values()
                        .filter(|symbol| {
                            symbol.kind == SymbolKind::Local
                                && node.text_range().contains_range(symbol.key.text_range())
                        })
                        .collect::<Vec<_>>();
                    uninit::check(&mut diagnostics, db, document, symbol_table, &node, &locals, &mut bump);
                    unread::check(
                        &mut diagnostics,
                        db,
                        document,
                        config.lint.unread,
                        symbol_table,
                        &node,
                        &locals,
                        &mut bump,
                    );
                    if let Some(diagnostic) = import_with_def::check(db, document, &node) {
                        diagnostics.push(diagnostic);
                    }
                }
                SyntaxKind::MODULE_FIELD_GLOBAL => {
                    typeck::check_global(
                        &mut diagnostics,
                        db,
                        document,
                        symbol_table,
                        module_id,
                        &node,
                        &mut bump,
                    );
                    if let Some(diagnostic) = const_expr::check(&node) {
                        diagnostics.push(diagnostic);
                    }
                    if let Some(diagnostic) = import_with_def::check(db, document, &node) {
                        diagnostics.push(diagnostic);
                    }
                }
                SyntaxKind::MODULE_FIELD_IMPORT => {
                    if let Some(diagnostic) = import_occur::check(db, document, &node) {
                        diagnostics.push(diagnostic);
                    }
                }
                SyntaxKind::PLAIN_INSTR => {
                    if let Some(instr) = FastPlainInstr::new(&node, &bump) {
                        if let Some(diagnostic) = unknown_instr::check(&instr) {
                            diagnostics.push(diagnostic);
                        }
                        immediates::check(&mut diagnostics, &node, &instr);
                        br_table_branches::check(&mut diagnostics, db, document, symbol_table, &instr, &bump);
                        if let Some(diagnostic) = packing::check(db, document, symbol_table, &instr) {
                            diagnostics.push(diagnostic);
                        }
                        type_misuse::check(db, &mut diagnostics, document, symbol_table, module_id, &node, &instr);
                        if let Some(diagnostic) = new_non_defaultable::check(db, document, symbol_table, &instr) {
                            diagnostics.push(diagnostic);
                        }
                    }
                    bump.reset();
                }
                SyntaxKind::BLOCK_BLOCK | SyntaxKind::BLOCK_LOOP | SyntaxKind::BLOCK_IF => {
                    if let Some(diagnostic) = block_type::check(db, document, symbol_table, &node) {
                        diagnostics.push(diagnostic);
                    }
                }
                SyntaxKind::MODULE_FIELD_START => {
                    if let Some(diagnostic) = start::check(db, document, symbol_table, &node) {
                        diagnostics.push(diagnostic);
                    }
                }
                SyntaxKind::MODULE_FIELD_TABLE => {
                    typeck::check_table(
                        &mut diagnostics,
                        db,
                        document,
                        symbol_table,
                        module_id,
                        &node,
                        &mut bump,
                    );
                    if let Some(diagnostic) = const_expr::check(&node) {
                        diagnostics.push(diagnostic);
                    }
                    if let Some(diagnostic) = import_with_def::check(db, document, &node) {
                        diagnostics.push(diagnostic);
                    }
                }
                SyntaxKind::MODULE_FIELD_ELEM => {
                    if let Some(diagnostic) = elem_type::check(db, document, &root, symbol_table, module_id, &node) {
                        diagnostics.push(diagnostic);
                    }
                }
                SyntaxKind::MODULE_FIELD_MEMORY => {
                    if let Some(diagnostic) = import_with_def::check(db, document, &node) {
                        diagnostics.push(diagnostic);
                    }
                }
                SyntaxKind::MEM_TYPE => {
                    mem_type::check(&mut diagnostics, &node);
                }
                SyntaxKind::OFFSET => {
                    typeck::check_offset(
                        &mut diagnostics,
                        db,
                        document,
                        symbol_table,
                        module_id,
                        &node,
                        &mut bump,
                    );
                    if let Some(diagnostic) = const_expr::check(&node) {
                        diagnostics.push(diagnostic);
                    }
                }
                SyntaxKind::ELEM_LIST => {
                    typeck::check_elem_list(
                        &mut diagnostics,
                        db,
                        document,
                        symbol_table,
                        module_id,
                        &node,
                        &mut bump,
                    );
                }
                SyntaxKind::ELEM_EXPR => {
                    if let Some(diagnostic) = const_expr::check(&node) {
                        diagnostics.push(diagnostic);
                    }
                }
                SyntaxKind::MODULE_FIELD_TAG => {
                    tag_type::check(&mut diagnostics, db, document, symbol_table, &node);
                    if let Some(diagnostic) = import_with_def::check(db, document, &node) {
                        diagnostics.push(diagnostic);
                    }
                }
                SyntaxKind::EXTERN_TYPE_TAG => {
                    tag_type::check(&mut diagnostics, db, document, symbol_table, &node);
                }
                SyntaxKind::BLOCK_TRY_TABLE => {
                    if let Some(diagnostic) = needless_try_table::check(config.lint.needless_try_table, &node) {
                        diagnostics.push(diagnostic);
                    }
                    useless_catch::check(&mut diagnostics, config.lint.useless_catch, symbol_table, &node);
                    if let Some(diagnostic) = block_type::check(db, document, symbol_table, &node) {
                        diagnostics.push(diagnostic);
                    }
                }
                SyntaxKind::CATCH | SyntaxKind::CATCH_ALL => {
                    if let Some(diagnostic) = catch_type::check(db, document, symbol_table, module_id, node) {
                        diagnostics.push(diagnostic);
                    }
                }
                SyntaxKind::IMMEDIATE
                | SyntaxKind::TYPE_DEF
                | SyntaxKind::REC_TYPE
                | SyntaxKind::TYPE_USE
                | SyntaxKind::LOCAL
                | SyntaxKind::IMPORT
                | SyntaxKind::EXPORT
                | SyntaxKind::GLOBAL_TYPE => {
                    descendants.skip_subtree();
                }
                _ => {}
            }
        }
        multi_starts::check(&mut diagnostics, &module);
    });
    undef::check(db, &mut diagnostics, symbol_table);
    dup_names::check(db, &mut diagnostics, document, symbol_table, &mut bump);
    unused::check(db, &mut diagnostics, document, config.lint.unused, symbol_table, &bump);
    shadow::check(db, &mut diagnostics, config.lint.shadow, symbol_table, &mut bump);
    mutated_immutable::check(db, &mut diagnostics, document, symbol_table);
    needless_mut::check(db, &mut diagnostics, config.lint.needless_mut, document, symbol_table);
    subtyping::check(&mut diagnostics, db, document, &root, symbol_table);
    deprecated::check(&mut diagnostics, db, document, config.lint.deprecated, symbol_table);

    diagnostics.sort_unstable_by(|a, b| match a.code.cmp(&b.code) {
        Ordering::Equal => a.range.ordering(b.range),
        other => other,
    });
    diagnostics
        .into_iter()
        .map(|diagnostic| lspt::Diagnostic {
            range: line_index.convert(diagnostic.range),
            severity: Some(diagnostic.severity),
            code: Some(Union2::B(diagnostic.code)),
            code_description: None,
            source: Some("wat".into()),
            message: diagnostic.message,
            tags: diagnostic.tags,
            related_information: diagnostic.related_information.map(|related_information| {
                related_information
                    .into_iter()
                    .map(|info| DiagnosticRelatedInformation {
                        location: Location {
                            uri: uri.raw(db),
                            range: line_index.convert(info.range),
                        },
                        message: info.message,
                    })
                    .collect()
            }),
            data: diagnostic.data,
        })
        .collect()
}

struct Diagnostic {
    range: TextRange,
    severity: DiagnosticSeverity,
    code: String,
    message: String,
    tags: Option<Vec<DiagnosticTag>>,
    related_information: Option<Vec<RelatedInformation>>,
    data: Option<serde_json::Value>,
}
pub struct RelatedInformation {
    range: TextRange,
    message: String,
}
impl Default for Diagnostic {
    fn default() -> Self {
        Self {
            range: Default::default(),
            severity: DiagnosticSeverity::Error,
            code: Default::default(),
            message: Default::default(),
            tags: Default::default(),
            related_information: Default::default(),
            data: None,
        }
    }
}

struct FastPlainInstr<'bump> {
    ptr: SyntaxNodePtr,
    name: &'bump str,
    name_range: TextRange,
    immediates: BumpVec<'bump, SyntaxNodePtr>,
}
impl<'bump> FastPlainInstr<'bump> {
    fn new(node: &SyntaxNode, bump: &'bump Bump) -> Option<Self> {
        support::token(node, SyntaxKind::INSTR_NAME).map(|instr_name| Self {
            ptr: SyntaxNodePtr::new(node),
            name: bump.alloc_str(instr_name.text()),
            name_range: instr_name.text_range(),
            immediates: BumpVec::from_iter_in(
                node.children_by_kind(|kind| kind == SyntaxKind::IMMEDIATE)
                    .map(|immediate| SyntaxNodePtr::new(&immediate)),
                bump,
            ),
        })
    }
}
