use crate::{
    binder::{SymbolKey, SymbolKind, SymbolTable},
    config::ServiceConfig,
    document::Document,
    helpers::LineIndexExt,
    imex,
    types_analyzer::{DefTypes, get_def_types},
};
use bumpalo::Bump;
use lspt::{
    DiagnosticRelatedInformation, DiagnosticSeverity, DiagnosticTag, Location, NumberOrString, StringOrMarkupContent,
};
use std::cmp::Ordering;
use wat_syntax::{NodeOrToken, SyntaxKind, SyntaxNode, TextRange};

mod block_type;
mod br_table_branches;
mod catch_type;
mod const_expr;
mod cont_type;
mod deprecated;
mod dup_names;
mod elem_type;
mod implicit_module;
mod import_occur;
mod import_with_def;
mod lane;
mod mem_arg;
mod mem_type;
mod multi_modules;
mod multi_starts;
mod mutated_immutable;
mod needless_mut;
mod needless_try_table;
mod new_non_defaultable;
mod omitted_idx_in_instr;
mod packing;
mod plain_instr;
mod shadow;
mod start;
mod subtyping;
mod syntax;
mod table_type;
mod tag_type;
mod type_misuse;
mod typeck;
mod undef;
mod uninit;
mod unreachable;
mod unread;
mod unused;
mod useless_catch;

pub fn check(db: &dyn salsa::Database, uri: &str, document: Document, config: &ServiceConfig) -> Vec<lspt::Diagnostic> {
    let mut bump = Bump::with_capacity(32 * 1024);

    let line_index = document.line_index(db);
    let root = SyntaxNode::new_root(document.root(db));
    let symbol_table = SymbolTable::of(db, document);
    let def_types = get_def_types(db, document);
    let imports = imex::get_imports(db, document);

    let mut diagnostics = Vec::with_capacity(4);
    syntax::check(db, &mut diagnostics, document);
    multi_modules::check(&mut diagnostics, config.lint.multi_modules, root.amber());
    root.children().enumerate().for_each(|(module_id, module)| {
        if let Some(diagnostic) = implicit_module::check(config.lint.implicit_module, module.amber()) {
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
        let mut node_stack = Vec::with_capacity(20);
        node_stack.push((module.amber(), 0));
        while let Some((parent, index)) = node_stack.last_mut() {
            match parent.child_or_token_at(*index) {
                Some(NodeOrToken::Node(node)) => {
                    match node.kind() {
                        SyntaxKind::MODULE_FIELD_FUNC => {
                            typeck::check_func(&mut diagnostics, &mut ctx, node);
                            unreachable::check(&mut diagnostics, &mut ctx, node);
                            if let Some(diagnostic) = import_with_def::check(&mut ctx, node) {
                                diagnostics.push(diagnostic);
                            }
                        }
                        SyntaxKind::MODULE_FIELD_GLOBAL => {
                            typeck::check_global(&mut diagnostics, &mut ctx, node);
                            if let Some(diagnostic) = const_expr::check(node) {
                                diagnostics.push(diagnostic);
                            }
                            if let Some(diagnostic) = import_with_def::check(&mut ctx, node) {
                                diagnostics.push(diagnostic);
                            }
                        }
                        SyntaxKind::PLAIN_INSTR => {
                            if let Some(instr_name) = node.tokens_by_kind(SyntaxKind::INSTR_NAME).next() {
                                plain_instr::check(&mut diagnostics, node, instr_name);
                                br_table_branches::check(&mut diagnostics, &ctx, node, instr_name);
                                if let Some(diagnostic) = packing::check(&ctx, node, instr_name) {
                                    diagnostics.push(diagnostic);
                                }
                                type_misuse::check(&mut diagnostics, &ctx, node, instr_name);
                                if let Some(diagnostic) = new_non_defaultable::check(&ctx, node, instr_name) {
                                    diagnostics.push(diagnostic);
                                }
                                mem_arg::check(&mut diagnostics, &ctx, node, instr_name);
                                lane::check(&mut diagnostics, node, instr_name);
                                if let Some(diagnostic) =
                                    omitted_idx_in_instr::check(ctx.config.lint.omitted_idx_in_instr, node)
                                {
                                    diagnostics.push(diagnostic);
                                }
                            }
                            ctx.bump.reset();
                        }
                        SyntaxKind::BLOCK_BLOCK | SyntaxKind::BLOCK_LOOP | SyntaxKind::BLOCK_IF => {
                            if let Some(diagnostic) = block_type::check(&ctx, node) {
                                diagnostics.push(diagnostic);
                            }
                        }
                        SyntaxKind::MODULE_FIELD_START => {
                            if let Some(diagnostic) = start::check(&ctx, node) {
                                diagnostics.push(diagnostic);
                            }
                        }
                        SyntaxKind::MODULE_FIELD_TABLE => {
                            typeck::check_table(&mut diagnostics, &mut ctx, node);
                            if let Some(diagnostic) = const_expr::check(node) {
                                diagnostics.push(diagnostic);
                            }
                            if let Some(diagnostic) = import_with_def::check(&mut ctx, node) {
                                diagnostics.push(diagnostic);
                            }
                        }
                        SyntaxKind::MODULE_FIELD_ELEM => {
                            if let Some(diagnostic) = elem_type::check(&ctx, node) {
                                diagnostics.push(diagnostic);
                            }
                        }
                        SyntaxKind::MODULE_FIELD_MEMORY => {
                            if let Some(diagnostic) = import_with_def::check(&mut ctx, node) {
                                diagnostics.push(diagnostic);
                            }
                        }
                        SyntaxKind::MEM_TYPE => {
                            mem_type::check(&mut diagnostics, node);
                        }
                        SyntaxKind::TABLE_TYPE => {
                            table_type::check(&mut diagnostics, node);
                        }
                        SyntaxKind::OFFSET => {
                            typeck::check_offset(&mut diagnostics, &mut ctx, node);
                            if let Some(diagnostic) = const_expr::check(node) {
                                diagnostics.push(diagnostic);
                            }
                        }
                        SyntaxKind::ELEM_LIST => {
                            typeck::check_elem_list(&mut diagnostics, &mut ctx, node);
                        }
                        SyntaxKind::ELEM_EXPR => {
                            if let Some(diagnostic) = const_expr::check(node) {
                                diagnostics.push(diagnostic);
                            }
                        }
                        SyntaxKind::MODULE_FIELD_TAG => {
                            tag_type::check(&mut diagnostics, &ctx, node);
                            if let Some(diagnostic) = import_with_def::check(&mut ctx, node) {
                                diagnostics.push(diagnostic);
                            }
                        }
                        SyntaxKind::EXTERN_TYPE_TAG => {
                            tag_type::check(&mut diagnostics, &ctx, node);
                        }
                        SyntaxKind::BLOCK_TRY_TABLE => {
                            if let Some(diagnostic) =
                                needless_try_table::check(ctx.config.lint.needless_try_table, node)
                            {
                                diagnostics.push(diagnostic);
                            }
                            useless_catch::check(&mut diagnostics, &ctx, node);
                            if let Some(diagnostic) = block_type::check(&ctx, node) {
                                diagnostics.push(diagnostic);
                            }
                        }
                        SyntaxKind::CATCH | SyntaxKind::CATCH_ALL => {
                            if let Some(diagnostic) = catch_type::check(&ctx, node) {
                                diagnostics.push(diagnostic);
                            }
                        }
                        SyntaxKind::CONT_TYPE => {
                            if let Some(diagnostic) = cont_type::check(&ctx, node) {
                                diagnostics.push(diagnostic);
                            }
                        }
                        SyntaxKind::IMMEDIATE
                        | SyntaxKind::FUNC_TYPE
                        | SyntaxKind::STRUCT_TYPE
                        | SyntaxKind::ARRAY_TYPE
                        | SyntaxKind::TYPE_USE
                        | SyntaxKind::LOCAL
                        | SyntaxKind::IMPORT
                        | SyntaxKind::EXPORT
                        | SyntaxKind::GLOBAL_TYPE
                        | SyntaxKind::MODULE_FIELD_EXPORT => {
                            *index += 1;
                            continue;
                        }
                        _ => {}
                    }
                    node_stack.push((node, 0));
                }
                Some(NodeOrToken::Token(..)) => {
                    *index += 1;
                }
                None => {
                    node_stack.pop();
                    if let Some((_, index)) = node_stack.last_mut() {
                        *index += 1;
                    }
                }
            }
        }
        multi_starts::check(&mut diagnostics, module.amber());
        import_occur::check(&mut diagnostics, imports, module.amber());
    });
    symbol_table
        .symbols
        .iter()
        .filter(|symbol| symbol.kind == SymbolKind::Local)
        .filter_map(|symbol| {
            symbol_table
                .symbols
                .get(symbol.region)
                .map(|func| (symbol, func.amber()))
        })
        .for_each(|(local, func)| {
            uninit::check(&mut diagnostics, db, symbol_table, func, local, &bump);
            unread::check(
                &mut diagnostics,
                db,
                config.lint.unread,
                symbol_table,
                func,
                local,
                &bump,
            );
            bump.reset();
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
        .filter_map(|diagnostic| {
            Some(lspt::Diagnostic {
                range: line_index.convert(diagnostic.range)?,
                severity: Some(diagnostic.severity),
                code: Some(NumberOrString::String(diagnostic.code)),
                code_description: None,
                source: Some("wat".into()),
                message: StringOrMarkupContent::String(diagnostic.message),
                tags: diagnostic.tags,
                related_information: diagnostic.related_information.map(|related_information| {
                    related_information
                        .into_iter()
                        .filter_map(|info| {
                            line_index
                                .convert(info.range)
                                .map(|range| DiagnosticRelatedInformation {
                                    location: Location {
                                        uri: uri.to_owned(),
                                        range,
                                    },
                                    message: info.message,
                                })
                        })
                        .collect()
                }),
                data: diagnostic.data,
            })
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
    module: &'db SyntaxNode<'db>,
    module_id: u32,
    bump: &'bump mut Bump,
}
