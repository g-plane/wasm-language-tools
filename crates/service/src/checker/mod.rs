use crate::{
    binder::{SymbolKey, SymbolKind, SymbolTable},
    config::ServiceConfig,
    document::Document,
    helpers::LineIndexExt,
    imex,
    types_analyzer::{DefTypes, get_def_types},
};
use bumpalo::Bump;
use lspt::{DiagnosticRelatedInformation, DiagnosticSeverity, DiagnosticTag, Location, Union2};
use std::cmp::Ordering;
use wat_syntax::{AmberNode, SyntaxKind, SyntaxNode, TextRange};

mod block_type;
mod br_table_branches;
mod catch_type;
mod const_expr;
mod deprecated;
mod dup_names;
mod elem_type;
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
mod plain_instr;
mod shadow;
mod start;
mod subtyping;
mod syntax;
mod tag_type;
mod type_misuse;
mod typeck;
mod undef;
mod uninit;
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
    let def_types = get_def_types(db, document);
    let imports = imex::get_imports(db, document);

    let mut diagnostics = Vec::with_capacity(4);
    syntax::check(db, &mut diagnostics, document);
    multi_modules::check(&mut diagnostics, config.lint.multi_modules, &root);
    root.children().enumerate().for_each(|(module_id, module)| {
        if let Some(diagnostic) = implicit_module::check(config.lint.implicit_module, &module) {
            diagnostics.push(diagnostic);
        }
        let mut ctx = DiagnosticCtx {
            db,
            document,
            config,
            symbol_table,
            def_types,
            imports,
            module: &module,
            module_id: module_id as u32,
            bump: &mut bump,
        };
        visit_node(&mut diagnostics, &mut ctx, module.amber());
        fn visit_node(diagnostics: &mut Vec<Diagnostic>, ctx: &mut DiagnosticCtx, node: AmberNode) {
            match node.kind() {
                SyntaxKind::MODULE_FIELD_FUNC => {
                    typeck::check_func(diagnostics, ctx, node);
                    unreachable::check(diagnostics, ctx, ctx.config.lint.unreachable, node);
                    let locals = ctx
                        .symbol_table
                        .symbols
                        .values()
                        .filter(|symbol| {
                            symbol.kind == SymbolKind::Local
                                && node.text_range().contains_range(symbol.key.text_range())
                        })
                        .collect::<Vec<_>>();
                    uninit::check(diagnostics, ctx, node, &locals);
                    unread::check(diagnostics, ctx, ctx.config.lint.unread, node, &locals);
                    if let Some(diagnostic) = import_with_def::check(ctx, node) {
                        diagnostics.push(diagnostic);
                    }
                }
                SyntaxKind::MODULE_FIELD_GLOBAL => {
                    typeck::check_global(diagnostics, ctx, node);
                    if let Some(diagnostic) = const_expr::check(node) {
                        diagnostics.push(diagnostic);
                    }
                    if let Some(diagnostic) = import_with_def::check(ctx, node) {
                        diagnostics.push(diagnostic);
                    }
                }
                SyntaxKind::PLAIN_INSTR => {
                    if let Some(instr_name) = node.tokens_by_kind(SyntaxKind::INSTR_NAME).next() {
                        plain_instr::check(diagnostics, node, instr_name);
                        br_table_branches::check(diagnostics, ctx, node, instr_name);
                        if let Some(diagnostic) = packing::check(ctx, node, instr_name) {
                            diagnostics.push(diagnostic);
                        }
                        type_misuse::check(diagnostics, ctx, node, instr_name);
                        if let Some(diagnostic) = new_non_defaultable::check(ctx, node, instr_name) {
                            diagnostics.push(diagnostic);
                        }
                    }
                    ctx.bump.reset();
                }
                SyntaxKind::BLOCK_BLOCK | SyntaxKind::BLOCK_LOOP | SyntaxKind::BLOCK_IF => {
                    if let Some(diagnostic) = block_type::check(ctx, node) {
                        diagnostics.push(diagnostic);
                    }
                }
                SyntaxKind::MODULE_FIELD_START => {
                    if let Some(diagnostic) = start::check(ctx, node) {
                        diagnostics.push(diagnostic);
                    }
                }
                SyntaxKind::MODULE_FIELD_TABLE => {
                    typeck::check_table(diagnostics, ctx, node);
                    if let Some(diagnostic) = const_expr::check(node) {
                        diagnostics.push(diagnostic);
                    }
                    if let Some(diagnostic) = import_with_def::check(ctx, node) {
                        diagnostics.push(diagnostic);
                    }
                }
                SyntaxKind::MODULE_FIELD_ELEM => {
                    if let Some(diagnostic) = elem_type::check(ctx, node) {
                        diagnostics.push(diagnostic);
                    }
                }
                SyntaxKind::MODULE_FIELD_MEMORY => {
                    if let Some(diagnostic) = import_with_def::check(ctx, node) {
                        diagnostics.push(diagnostic);
                    }
                }
                SyntaxKind::MEM_TYPE => {
                    mem_type::check(diagnostics, node);
                }
                SyntaxKind::OFFSET => {
                    typeck::check_offset(diagnostics, ctx, node);
                    if let Some(diagnostic) = const_expr::check(node) {
                        diagnostics.push(diagnostic);
                    }
                }
                SyntaxKind::ELEM_LIST => {
                    typeck::check_elem_list(diagnostics, ctx, node);
                }
                SyntaxKind::ELEM_EXPR => {
                    if let Some(diagnostic) = const_expr::check(node) {
                        diagnostics.push(diagnostic);
                    }
                }
                SyntaxKind::MODULE_FIELD_TAG => {
                    tag_type::check(diagnostics, ctx, node);
                    if let Some(diagnostic) = import_with_def::check(ctx, node) {
                        diagnostics.push(diagnostic);
                    }
                }
                SyntaxKind::EXTERN_TYPE_TAG => {
                    tag_type::check(diagnostics, ctx, node);
                }
                SyntaxKind::BLOCK_TRY_TABLE => {
                    if let Some(diagnostic) = needless_try_table::check(ctx.config.lint.needless_try_table, node) {
                        diagnostics.push(diagnostic);
                    }
                    useless_catch::check(diagnostics, ctx, ctx.config.lint.useless_catch, node);
                    if let Some(diagnostic) = block_type::check(ctx, node) {
                        diagnostics.push(diagnostic);
                    }
                }
                SyntaxKind::CATCH | SyntaxKind::CATCH_ALL => {
                    if let Some(diagnostic) = catch_type::check(ctx, node) {
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
                | SyntaxKind::GLOBAL_TYPE
                | SyntaxKind::MODULE_FIELD_EXPORT => {
                    return;
                }
                _ => {}
            }
            node.children().for_each(|child| visit_node(diagnostics, ctx, child));
        }
        multi_starts::check(&mut diagnostics, module.amber());
        import_occur::check(&mut diagnostics, db, document, module.amber());
    });
    undef::check(db, &mut diagnostics, symbol_table);
    dup_names::check(db, &mut diagnostics, document, symbol_table, &mut bump);
    unused::check(
        db,
        &mut diagnostics,
        document,
        config.lint.unused,
        symbol_table,
        imports,
        &bump,
    );
    shadow::check(db, &mut diagnostics, config.lint.shadow, symbol_table, &mut bump);
    mutated_immutable::check(db, &mut diagnostics, document, symbol_table);
    needless_mut::check(db, &mut diagnostics, config.lint.needless_mut, document, symbol_table);
    subtyping::check(&mut diagnostics, db, document, symbol_table, def_types);
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

struct DiagnosticCtx<'db, 'bump> {
    db: &'db dyn salsa::Database,
    document: Document,
    config: &'db ServiceConfig,
    symbol_table: &'db SymbolTable<'db>,
    def_types: &'db DefTypes<'db>,
    imports: &'db [SymbolKey],
    module: &'db SyntaxNode,
    module_id: u32,
    bump: &'bump mut Bump,
}
