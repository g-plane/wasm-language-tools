use crate::{
    document::Document,
    helpers,
    idx::{Idx, IdxGen, InternIdent},
};
use indexmap::IndexMap;
use rowan::{
    GreenNode, NodeOrToken, TextRange,
    ast::{AstNode, support},
};
use rustc_hash::{FxBuildHasher, FxHashMap};
use std::{fmt, hash::Hash, ops::Deref};
use wat_syntax::{SyntaxKind, SyntaxNode, SyntaxNodePtr, ast::ValType};

#[derive(Clone, Debug, PartialEq, Eq, salsa::Update)]
pub(crate) struct SymbolTable<'db> {
    pub symbols: Symbols<'db>,
    pub resolved: FxHashMap<SymbolKey, SymbolKey>,
    pub def_poi: FxHashMap<SymbolKey, TextRange>,
}
fn create_symbol_table<'db>(db: &'db dyn salsa::Database, document: Document) -> SymbolTable<'db> {
    fn create_module_level_symbol<'db>(
        db: &'db dyn salsa::Database,
        node: &SyntaxNode,
        id: u32,
        kind: SymbolKind,
        module: &SyntaxNode,
    ) -> Symbol<'db> {
        Symbol {
            key: SymbolKey::new(node),
            green: node.green().into(),
            region: SymbolKey::new(module),
            kind,
            idx: Idx {
                num: Some(id),
                name: support::token(node, SyntaxKind::IDENT).map(|token| InternIdent::new(db, token.text())),
            },
            idx_kind: kind.into(),
        }
    }
    fn create_ref_symbol<'db>(
        db: &'db dyn salsa::Database,
        node: &SyntaxNode,
        region: SymbolKey,
        kind: SymbolKind,
    ) -> Option<Symbol<'db>> {
        node.children_with_tokens()
            .find_map(|node_or_token| match node_or_token {
                NodeOrToken::Token(token) => match token.kind() {
                    SyntaxKind::IDENT => Some(Idx {
                        num: None,
                        name: Some(InternIdent::new(db, token.text())),
                    }),
                    SyntaxKind::UNSIGNED_INT | SyntaxKind::INT => token.text().parse().ok().map(|num| Idx {
                        num: Some(num),
                        name: None,
                    }),
                    _ => None,
                },
                _ => None,
            })
            .map(|idx| Symbol {
                key: SymbolKey::new(node),
                green: node.green().into(),
                region,
                kind,
                idx,
                idx_kind: kind.into(),
            })
    }
    fn create_first_optional_ref_symbol<'db>(
        db: &'db dyn salsa::Database,
        node: &SyntaxNode,
        kind: SymbolKind,
    ) -> Option<Symbol<'db>> {
        node.ancestors()
            .find(|node| node.kind() == SyntaxKind::MODULE)
            .map(|region| {
                let region = SymbolKey::new(&region);
                node.first_child_by_kind(&|kind| kind == SyntaxKind::IMMEDIATE)
                    .and_then(|immediate| create_ref_symbol(db, &immediate, region, kind))
                    .unwrap_or_else(|| Symbol {
                        green: node.green().into(),
                        key: SymbolKey::new(node),
                        region,
                        kind,
                        idx: Idx {
                            num: Some(0),
                            name: None,
                        },
                        idx_kind: kind.into(),
                    })
            })
    }
    fn create_extern_type_symbol<'db>(
        db: &'db dyn salsa::Database,
        node: &SyntaxNode,
        id: u32,
        kind: SymbolKind,
        module: &SyntaxNode,
    ) -> Symbol<'db> {
        Symbol {
            key: SymbolKey::new(node),
            green: node.green().into(),
            region: SymbolKey::new(module),
            kind,
            idx: Idx {
                num: Some(id),
                name: support::token(node, SyntaxKind::IDENT).map(|token| InternIdent::new(db, token.text())),
            },
            idx_kind: kind.into(),
        }
    }
    fn search_def<'a, 'db>(
        defs: &'a [(SymbolKey, Option<InternIdent<'db>>)],
        idx: Idx,
    ) -> Option<&'a (SymbolKey, Option<InternIdent<'db>>)> {
        idx.num.and_then(|num| defs.get(num as usize)).or_else(|| {
            idx.name.and_then(|name| {
                defs.iter()
                    .find(|(_, def_name)| def_name.is_some_and(|def_name| def_name == name))
            })
        })
    }
    fn find_up_block(node: &SyntaxNode) -> Option<SyntaxNode> {
        node.ancestors().skip(1).find(|node| {
            matches!(
                node.kind(),
                SyntaxKind::BLOCK_BLOCK
                    | SyntaxKind::BLOCK_LOOP
                    | SyntaxKind::BLOCK_IF
                    | SyntaxKind::BLOCK_TRY_TABLE
                    | SyntaxKind::MODULE_FIELD_FUNC
            )
        })
    }
    fn resolve_block_def(symbol: &Symbol, symbols: &Symbols) -> Option<SymbolKey> {
        let mut current = symbol;
        let mut levels = 0;
        while let Some(parent) = symbols
            .values()
            .find(|sym| sym.kind == SymbolKind::BlockDef && sym.key == current.region)
        {
            if symbol.idx.is_defined_by(&Idx {
                num: Some(levels),
                name: parent.idx.name,
            }) {
                return Some(parent.key);
            }
            current = parent;
            levels += 1;
        }
        if symbol.idx.is_defined_by(&Idx {
            num: Some(levels),
            name: None,
        }) {
            symbols
                .values()
                .find(|symbol| symbol.kind == SymbolKind::Func && symbol.key == current.region)
                .map(|func_symbol| func_symbol.key)
        } else {
            None
        }
    }
    fn infer_def_poi(node: &SyntaxNode) -> TextRange {
        support::token(node, SyntaxKind::IDENT)
            .or_else(|| support::token(node, SyntaxKind::KEYWORD))
            .map(|token| token.text_range())
            .unwrap_or_else(|| node.text_range())
    }

    let root = document.root_tree(db);
    let mut symbols = Symbols::with_capacity_and_hasher(8, FxBuildHasher);
    let mut resolved = FxHashMap::default();
    let mut def_poi = FxHashMap::default();
    root.children().enumerate().for_each(|(module_id, module)| {
        let module_key = SymbolKey::new(&module);
        symbols.insert(
            module_key,
            Symbol {
                green: module.green().into(),
                key: module_key,
                region: SymbolKey::new(&root),
                kind: SymbolKind::Module,
                idx: Idx {
                    num: Some(module_id as u32),
                    name: None,
                },
                idx_kind: IdxKind::Module,
            },
        );
        def_poi.insert(module_key, infer_def_poi(&module));
        let mut func_idx_gen = IdxGen::default();
        let mut local_idx_gen = IdxGen::default();
        let mut type_idx_gen = IdxGen::default();
        let mut global_idx_gen = IdxGen::default();
        let mut mem_idx_gen = IdxGen::default();
        let mut table_idx_gen = IdxGen::default();
        let mut field_idx_gen = IdxGen::default();
        let mut tag_idx_gen = IdxGen::default();
        let mut data_idx_gen = IdxGen::default();

        let mut funcs = Vec::new();
        let mut locals = Vec::new();
        let mut types = Vec::new();
        let mut globals = Vec::new();
        let mut memories = Vec::new();
        let mut tables = Vec::new();
        let mut fields = FxHashMap::default();
        let mut tags = Vec::new();
        let mut datas = Vec::new();
        let mut indirect_params = Vec::new();

        module.descendants().for_each(|node| match node.kind() {
            SyntaxKind::MODULE_FIELD_FUNC => {
                let func_idx = func_idx_gen.pull();
                let symbol = create_module_level_symbol(db, &node, func_idx, SymbolKind::Func, &module);
                let func_key = symbol.key;
                funcs.push((func_key, symbol.idx.name));
                symbols.insert(func_key, symbol);
                def_poi.insert(func_key, infer_def_poi(&node));
                locals.clear();
                local_idx_gen.reset();
            }
            SyntaxKind::PARAM => {
                let region = if let Some(node) =
                    node.parent()
                        .and_then(|node| node.parent())
                        .and_then(|node| match node.kind() {
                            SyntaxKind::MODULE_FIELD_FUNC => Some(node),
                            SyntaxKind::SUB_TYPE => node.parent(),
                            _ => None,
                        }) {
                    SymbolKey::new(&node)
                } else {
                    return;
                };
                if let Some(ident) = support::token(&node, SyntaxKind::IDENT) {
                    let key = SymbolKey::new(&node);
                    let idx = local_idx_gen.pull();
                    let name = InternIdent::new(db, ident.text());
                    locals.push((key, Some(name)));
                    symbols.insert(
                        key,
                        Symbol {
                            key,
                            green: node.green().into(),
                            region,
                            kind: SymbolKind::Param,
                            idx: Idx {
                                num: Some(idx),
                                name: if region.kind() == SyntaxKind::TYPE_DEF {
                                    None
                                } else {
                                    Some(name)
                                },
                            },
                            idx_kind: IdxKind::Local,
                        },
                    );
                    def_poi.insert(key, ident.text_range());
                } else {
                    symbols.extend(
                        node.children()
                            .filter(|child| ValType::can_cast(child.kind()))
                            .map(|val_type| {
                                let key = SymbolKey::new(&val_type);
                                locals.push((key, None));
                                def_poi.insert(key, val_type.text_range());
                                (
                                    key,
                                    Symbol {
                                        key,
                                        green: val_type.green().into(),
                                        region,
                                        kind: SymbolKind::Param,
                                        idx: Idx {
                                            num: Some(local_idx_gen.pull()),
                                            name: None,
                                        },
                                        idx_kind: IdxKind::Local,
                                    },
                                )
                            }),
                    );
                }
            }
            SyntaxKind::LOCAL => {
                let func_key = if let Some(func) = node.parent() {
                    SymbolKey::new(&func)
                } else {
                    return;
                };
                if let Some(ident) = support::token(&node, SyntaxKind::IDENT) {
                    let key = SymbolKey::new(&node);
                    let idx = local_idx_gen.pull();
                    let name = InternIdent::new(db, ident.text());
                    locals.push((key, Some(name)));
                    symbols.insert(
                        key,
                        Symbol {
                            key,
                            green: node.green().into(),
                            region: func_key,
                            kind: SymbolKind::Local,
                            idx: Idx {
                                num: Some(idx),
                                name: Some(name),
                            },
                            idx_kind: IdxKind::Local,
                        },
                    );
                    def_poi.insert(key, ident.text_range());
                } else {
                    symbols.extend(
                        node.children()
                            .filter(|child| ValType::can_cast(child.kind()))
                            .map(|val_type| {
                                let key = SymbolKey::new(&val_type);
                                locals.push((key, None));
                                def_poi.insert(key, val_type.text_range());
                                (
                                    key,
                                    Symbol {
                                        key,
                                        green: val_type.green().into(),
                                        region: func_key,
                                        kind: SymbolKind::Local,
                                        idx: Idx {
                                            num: Some(local_idx_gen.pull()),
                                            name: None,
                                        },
                                        idx_kind: IdxKind::Local,
                                    },
                                )
                            }),
                    );
                }
            }
            SyntaxKind::TYPE_DEF => {
                let type_idx = type_idx_gen.pull();
                let symbol = create_module_level_symbol(db, &node, type_idx, SymbolKind::Type, &module);
                let type_def_key = symbol.key;
                types.push((type_def_key, symbol.idx.name));
                symbols.insert(type_def_key, symbol);
                def_poi.insert(type_def_key, infer_def_poi(&node));
            }
            SyntaxKind::FUNC_TYPE => {
                locals.clear();
                local_idx_gen.reset();
            }
            SyntaxKind::STRUCT_TYPE => {
                field_idx_gen.reset();
            }
            SyntaxKind::FIELD => {
                let type_def_key = if let Some(type_def) = node
                    .ancestors()
                    .find(|ancestor| ancestor.kind() == SyntaxKind::TYPE_DEF)
                {
                    SymbolKey::new(&type_def)
                } else {
                    return;
                };
                let fields = fields.entry(type_def_key).or_insert_with(|| Vec::with_capacity(1));
                if let Some(ident) = support::token(&node, SyntaxKind::IDENT) {
                    let key = SymbolKey::new(&node);
                    let idx = field_idx_gen.pull();
                    let name = InternIdent::new(db, ident.text());
                    fields.push((key, Some(name)));
                    symbols.insert(
                        key,
                        Symbol {
                            key,
                            green: node.green().into(),
                            region: type_def_key,
                            kind: SymbolKind::FieldDef,
                            idx: Idx {
                                num: Some(idx),
                                name: Some(name),
                            },
                            idx_kind: IdxKind::Field,
                        },
                    );
                    def_poi.insert(key, ident.text_range());
                } else {
                    symbols.extend(
                        node.children()
                            .filter(|child| child.kind() == SyntaxKind::FIELD_TYPE)
                            .map(|field_type| {
                                let key = SymbolKey::new(&field_type);
                                fields.push((key, None));
                                def_poi.insert(key, field_type.text_range());
                                (
                                    key,
                                    Symbol {
                                        key,
                                        green: field_type.green().into(),
                                        region: type_def_key,
                                        kind: SymbolKind::FieldDef,
                                        idx: Idx {
                                            num: Some(field_idx_gen.pull()),
                                            name: None,
                                        },
                                        idx_kind: IdxKind::Field,
                                    },
                                )
                            }),
                    );
                }
            }
            SyntaxKind::MODULE_FIELD_GLOBAL => {
                let idx = global_idx_gen.pull();
                let symbol = create_module_level_symbol(db, &node, idx, SymbolKind::GlobalDef, &module);
                globals.push((symbol.key, symbol.idx.name));
                def_poi.insert(symbol.key, infer_def_poi(&node));
                symbols.insert(symbol.key, symbol);
            }
            SyntaxKind::PLAIN_INSTR => {
                match node
                    .children_with_tokens()
                    .find_map(|node_or_token| match node_or_token {
                        NodeOrToken::Token(token) if token.kind() == SyntaxKind::INSTR_NAME => Some(token),
                        _ => None,
                    })
                    .as_ref()
                    .map(|token| token.text())
                {
                    Some("call" | "ref.func" | "return_call") => {
                        symbols.extend(node.children().filter_map(|node| {
                            create_ref_symbol(db, &node, module_key, SymbolKind::Call)
                                .map(|symbol| (symbol.key, symbol))
                        }));
                    }
                    Some("local.get" | "local.set" | "local.tee") => {
                        let Some(func) = node
                            .ancestors()
                            .find(|node| node.kind() == SyntaxKind::MODULE_FIELD_FUNC)
                        else {
                            return;
                        };
                        let region = SymbolKey::new(&func);
                        symbols.extend(node.children().filter_map(|node| {
                            create_ref_symbol(db, &node, region, SymbolKind::LocalRef)
                                .inspect(|symbol| {
                                    if let Some((def_key, _)) = search_def(&locals, symbol.idx) {
                                        resolved.insert(symbol.key, *def_key);
                                    } else if let Some(num) = symbol.idx.num
                                        && let Some(idx) = helpers::syntax::pick_type_idx_from_func(&func)
                                    {
                                        indirect_params.push((SymbolKey::new(&idx), symbol.key, num));
                                    }
                                })
                                .map(|symbol| (symbol.key, symbol))
                        }));
                    }
                    Some("global.get" | "global.set") => {
                        symbols.extend(node.children().filter_map(|node| {
                            create_ref_symbol(db, &node, module_key, SymbolKind::GlobalRef)
                                .map(|symbol| (symbol.key, symbol))
                        }));
                    }
                    Some(
                        "br" | "br_if" | "br_table" | "br_on_null" | "br_on_non_null" | "br_on_cast"
                        | "br_on_cast_fail",
                    ) => {
                        if let Some(region) = find_up_block(&node).map(|node| SymbolKey::new(&node)) {
                            node.children()
                                .filter_map(|node| create_ref_symbol(db, &node, region, SymbolKind::BlockRef))
                                .for_each(|symbol| {
                                    if let Some(def_key) = resolve_block_def(&symbol, &symbols) {
                                        resolved.insert(symbol.key, def_key);
                                    }
                                    symbols.insert(symbol.key, symbol);
                                });
                        }
                    }
                    Some("call_indirect" | "return_call_indirect") => {
                        if let Some(symbol) = create_first_optional_ref_symbol(db, &node, SymbolKind::TableRef) {
                            symbols.insert(symbol.key, symbol);
                        }
                    }
                    Some("table.get" | "table.set" | "table.size" | "table.grow" | "table.fill" | "table.copy") => {
                        symbols.extend(node.children().filter_map(|node| {
                            create_ref_symbol(db, &node, module_key, SymbolKind::TableRef)
                                .map(|symbol| (symbol.key, symbol))
                        }));
                    }
                    Some("table.init") => {
                        if let Some(symbol) = node
                            .children()
                            .next()
                            .and_then(|node| create_ref_symbol(db, &node, module_key, SymbolKind::TableRef))
                        {
                            symbols.insert(symbol.key, symbol);
                        }
                    }
                    Some(
                        "memory.size" | "memory.grow" | "memory.fill" | "i32.load" | "i64.load" | "f32.load"
                        | "f64.load" | "i32.load8_s" | "i32.load8_u" | "i32.load16_s" | "i32.load16_u" | "i64.load8_s"
                        | "i64.load8_u" | "i64.load16_s" | "i64.load16_u" | "i64.load32_s" | "i64.load32_u"
                        | "i32.store" | "i64.store" | "f32.store" | "f64.store" | "i32.store8" | "i32.store16"
                        | "i64.store8" | "i64.store16" | "i64.store32" | "v128.load" | "v128.load8x8_s"
                        | "v128.load8x8_u" | "v128.load16x4_s" | "v128.load16x4_u" | "v128.load32x2_s"
                        | "v128.load32x2_u" | "v128.load8_splat" | "v128.load16_splat" | "v128.load32_splat"
                        | "v128.load64_splat" | "v128.load32_zero" | "v128.load64_zero" | "v128.store"
                        | "v128.load8_lane" | "v128.load16_lane" | "v128.load32_lane" | "v128.load64_lane"
                        | "v128.store8_lane" | "v128.store16_lane" | "v128.store32_lane" | "v128.store64_lane",
                    ) => {
                        if let Some(symbol) = create_first_optional_ref_symbol(db, &node, SymbolKind::MemoryRef) {
                            symbols.insert(symbol.key, symbol);
                        }
                    }
                    Some("memory.init") => {
                        let mut immediates = node.children().filter(|child| child.kind() == SyntaxKind::IMMEDIATE);
                        if let Some(first) = immediates.next() {
                            if let Some(second) = immediates.next() {
                                if let Some(symbol) = create_ref_symbol(db, &first, module_key, SymbolKind::MemoryRef) {
                                    symbols.insert(symbol.key, symbol);
                                }
                                if let Some(symbol) = create_ref_symbol(db, &second, module_key, SymbolKind::DataRef) {
                                    symbols.insert(symbol.key, symbol);
                                }
                            } else {
                                let key = SymbolKey::new(&node);
                                symbols.insert(
                                    key,
                                    Symbol {
                                        key,
                                        green: node.green().into_owned(),
                                        region: module_key,
                                        kind: SymbolKind::MemoryRef,
                                        idx: Idx {
                                            num: Some(0),
                                            name: None,
                                        },
                                        idx_kind: IdxKind::Memory,
                                    },
                                );
                                if let Some(symbol) = create_ref_symbol(db, &first, module_key, SymbolKind::DataRef) {
                                    symbols.insert(symbol.key, symbol);
                                }
                            }
                        }
                    }
                    Some("memory.copy") => {
                        if let Some(symbol) = node
                            .children()
                            .next()
                            .and_then(|node| create_ref_symbol(db, &node, module_key, SymbolKind::MemoryRef))
                        {
                            symbols.insert(symbol.key, symbol);
                        }
                    }
                    Some("data.drop") => {
                        if let Some(symbol) = node
                            .first_child_by_kind(&|kind| kind == SyntaxKind::IMMEDIATE)
                            .and_then(|node| create_ref_symbol(db, &node, module_key, SymbolKind::DataRef))
                        {
                            symbols.insert(symbol.key, symbol);
                        }
                    }
                    Some(
                        "struct.new" | "struct.new_default" | "array.new" | "array.new_default" | "array.new_fixed"
                        | "array.new_elem" | "array.get" | "array.get_u" | "array.get_s" | "array.set" | "array.fill"
                        | "array.init_elem" | "call_ref" | "return_call_ref" | "ref.null",
                    ) => {
                        if let Some(symbol) = node
                            .first_child_by_kind(&|kind| kind == SyntaxKind::IMMEDIATE)
                            .and_then(|node| create_ref_symbol(db, &node, module_key, SymbolKind::TypeUse))
                        {
                            symbols.insert(symbol.key, symbol);
                        }
                    }
                    Some("array.copy") => {
                        symbols.extend(node.children().filter_map(|node| {
                            create_ref_symbol(db, &node, module_key, SymbolKind::TypeUse)
                                .map(|symbol| (symbol.key, symbol))
                        }));
                    }
                    Some("array.new_data" | "array.init_data") => {
                        let mut immediates = node.children().filter(|child| child.kind() == SyntaxKind::IMMEDIATE);
                        if let Some(symbol) = immediates
                            .next()
                            .and_then(|node| create_ref_symbol(db, &node, module_key, SymbolKind::TypeUse))
                        {
                            symbols.insert(symbol.key, symbol);
                        }
                        if let Some(symbol) = immediates
                            .next()
                            .and_then(|node| create_ref_symbol(db, &node, module_key, SymbolKind::DataRef))
                        {
                            symbols.insert(symbol.key, symbol);
                        }
                    }
                    Some("struct.get" | "struct.get_s" | "struct.get_u" | "struct.set") => {
                        let mut children = node.children();
                        if let Some(symbol) = children
                            .next()
                            .and_then(|node| create_ref_symbol(db, &node, module_key, SymbolKind::TypeUse))
                        {
                            let key = symbol.key;
                            symbols.insert(key, symbol);
                            if let Some(symbol) = children.next().and_then(|node| {
                                // The region here is temporary.
                                // It's used for tracking which struct it belongs to,
                                // and it will be replaced with the actual region later.
                                // If the struct it belongs to isn't defined, nothing will happen.
                                create_ref_symbol(db, &node, key, SymbolKind::FieldRef)
                            }) {
                                symbols.insert(symbol.key, symbol);
                            }
                        }
                    }
                    Some("throw") => {
                        if let Some(symbol) = node
                            .first_child_by_kind(&|kind| kind == SyntaxKind::IMMEDIATE)
                            .and_then(|node| create_ref_symbol(db, &node, module_key, SymbolKind::TagRef))
                        {
                            symbols.insert(symbol.key, symbol);
                        }
                    }
                    _ => {}
                }
            }
            SyntaxKind::BLOCK_BLOCK | SyntaxKind::BLOCK_IF | SyntaxKind::BLOCK_LOOP | SyntaxKind::BLOCK_TRY_TABLE => {
                if let Some(symbol) = find_up_block(&node).map(|region| Symbol {
                    key: SymbolKey::new(&node),
                    green: node.green().into(),
                    region: SymbolKey::new(&region),
                    kind: SymbolKind::BlockDef,
                    idx: Idx {
                        num: Some(0), // fake ID
                        name: support::token(&node, SyntaxKind::IDENT).map(|token| InternIdent::new(db, token.text())),
                    },
                    idx_kind: IdxKind::Block,
                }) {
                    def_poi.insert(symbol.key, infer_def_poi(&node));
                    symbols.insert(symbol.key, symbol);
                }
            }
            SyntaxKind::MODULE_FIELD_START | SyntaxKind::EXTERN_IDX_FUNC => {
                if let Some(symbol) = node
                    .first_child_by_kind(&|kind| kind == SyntaxKind::INDEX)
                    .and_then(|index| create_ref_symbol(db, &index, module_key, SymbolKind::Call))
                {
                    symbols.insert(symbol.key, symbol);
                }
            }
            SyntaxKind::TYPE_USE | SyntaxKind::HEAP_TYPE | SyntaxKind::SUB_TYPE => {
                if let Some(symbol) = node
                    .first_child_by_kind(&|kind| kind == SyntaxKind::INDEX)
                    .and_then(|index| create_ref_symbol(db, &index, module_key, SymbolKind::TypeUse))
                {
                    symbols.insert(symbol.key, symbol);
                }
            }
            SyntaxKind::MODULE_FIELD_MEMORY => {
                let idx = mem_idx_gen.pull();
                let symbol = create_module_level_symbol(db, &node, idx, SymbolKind::MemoryDef, &module);
                memories.push((symbol.key, symbol.idx.name));
                def_poi.insert(symbol.key, infer_def_poi(&node));
                symbols.insert(symbol.key, symbol);
            }
            SyntaxKind::MODULE_FIELD_TABLE => {
                let idx = table_idx_gen.pull();
                let symbol = create_module_level_symbol(db, &node, idx, SymbolKind::TableDef, &module);
                tables.push((symbol.key, symbol.idx.name));
                def_poi.insert(symbol.key, infer_def_poi(&node));
                symbols.insert(symbol.key, symbol);
            }
            SyntaxKind::MODULE_FIELD_TAG => {
                let idx = tag_idx_gen.pull();
                let symbol = create_module_level_symbol(db, &node, idx, SymbolKind::TagDef, &module);
                tags.push((symbol.key, symbol.idx.name));
                def_poi.insert(symbol.key, infer_def_poi(&node));
                symbols.insert(symbol.key, symbol);
            }
            SyntaxKind::EXTERN_IDX_GLOBAL => {
                if let Some(symbol) = node
                    .first_child_by_kind(&|kind| kind == SyntaxKind::INDEX)
                    .and_then(|index| create_ref_symbol(db, &index, module_key, SymbolKind::GlobalRef))
                {
                    symbols.insert(symbol.key, symbol);
                }
            }
            SyntaxKind::EXTERN_IDX_MEMORY => {
                if let Some(symbol) = node
                    .first_child_by_kind(&|kind| kind == SyntaxKind::INDEX)
                    .and_then(|index| create_ref_symbol(db, &index, module_key, SymbolKind::MemoryRef))
                {
                    symbols.insert(symbol.key, symbol);
                }
            }
            SyntaxKind::EXTERN_IDX_TABLE | SyntaxKind::TABLE_USE => {
                if let Some(symbol) = node
                    .first_child_by_kind(&|kind| kind == SyntaxKind::INDEX)
                    .and_then(|index| create_ref_symbol(db, &index, module_key, SymbolKind::TableRef))
                {
                    symbols.insert(symbol.key, symbol);
                }
            }
            SyntaxKind::EXTERN_IDX_TAG => {
                if let Some(symbol) = node
                    .first_child_by_kind(&|kind| kind == SyntaxKind::INDEX)
                    .and_then(|index| create_ref_symbol(db, &index, module_key, SymbolKind::TagRef))
                {
                    symbols.insert(symbol.key, symbol);
                }
            }
            SyntaxKind::EXTERN_TYPE_FUNC => {
                let idx = func_idx_gen.pull();
                let symbol = create_extern_type_symbol(db, &node, idx, SymbolKind::Func, &module);
                funcs.push((symbol.key, symbol.idx.name));
                def_poi.insert(symbol.key, infer_def_poi(&node));
                symbols.insert(symbol.key, symbol);
            }
            SyntaxKind::EXTERN_TYPE_TABLE => {
                let idx = table_idx_gen.pull();
                let symbol = create_extern_type_symbol(db, &node, idx, SymbolKind::TableDef, &module);
                tables.push((symbol.key, symbol.idx.name));
                def_poi.insert(symbol.key, infer_def_poi(&node));
                symbols.insert(symbol.key, symbol);
            }
            SyntaxKind::EXTERN_TYPE_MEMORY => {
                let idx = mem_idx_gen.pull();
                let symbol = create_extern_type_symbol(db, &node, idx, SymbolKind::MemoryDef, &module);
                memories.push((symbol.key, symbol.idx.name));
                def_poi.insert(symbol.key, infer_def_poi(&node));
                symbols.insert(symbol.key, symbol);
            }
            SyntaxKind::EXTERN_TYPE_GLOBAL => {
                let idx = global_idx_gen.pull();
                let symbol = create_extern_type_symbol(db, &node, idx, SymbolKind::GlobalDef, &module);
                globals.push((symbol.key, symbol.idx.name));
                def_poi.insert(symbol.key, infer_def_poi(&node));
                symbols.insert(symbol.key, symbol);
            }
            SyntaxKind::EXTERN_TYPE_TAG => {
                let idx = tag_idx_gen.pull();
                let symbol = create_extern_type_symbol(db, &node, idx, SymbolKind::TagDef, &module);
                tags.push((symbol.key, symbol.idx.name));
                def_poi.insert(symbol.key, infer_def_poi(&node));
                symbols.insert(symbol.key, symbol);
            }
            SyntaxKind::MODULE_FIELD_DATA => {
                let idx = data_idx_gen.pull();
                let symbol = create_module_level_symbol(db, &node, idx, SymbolKind::DataDef, &module);
                datas.push((symbol.key, symbol.idx.name));
                def_poi.insert(symbol.key, infer_def_poi(&node));
                symbols.insert(symbol.key, symbol);
            }
            SyntaxKind::MEM_USE => {
                if let Some(symbol) = node
                    .first_child_by_kind(&|kind| kind == SyntaxKind::INDEX)
                    .and_then(|index| create_ref_symbol(db, &index, module_key, SymbolKind::MemoryRef))
                {
                    symbols.insert(symbol.key, symbol);
                }
            }
            SyntaxKind::ELEM_LIST => {
                symbols.extend(
                    node.children()
                        .filter(|child| child.kind() == SyntaxKind::INDEX)
                        .filter_map(|index| {
                            create_ref_symbol(db, &index, module_key, SymbolKind::Call)
                                .map(|symbol| (symbol.key, symbol))
                        }),
                );
            }
            SyntaxKind::CATCH => {
                let mut children = node.children();
                if let Some(symbol) = children
                    .next()
                    .and_then(|node| create_ref_symbol(db, &node, module_key, SymbolKind::TagRef))
                {
                    symbols.insert(symbol.key, symbol);
                }
                if let Some(region) = node
                    .parent()
                    .and_then(|parent| find_up_block(&parent))
                    .map(|node| SymbolKey::new(&node))
                    && let Some(symbol) = children
                        .next()
                        .and_then(|node| create_ref_symbol(db, &node, region, SymbolKind::BlockRef))
                {
                    if let Some(def_key) = resolve_block_def(&symbol, &symbols) {
                        resolved.insert(symbol.key, def_key);
                    }
                    symbols.insert(symbol.key, symbol);
                }
            }
            SyntaxKind::CATCH_ALL => {
                if let Some(region) = node
                    .parent()
                    .and_then(|parent| find_up_block(&parent))
                    .map(|node| SymbolKey::new(&node))
                    && let Some(symbol) = node
                        .first_child_by_kind(&|kind| kind == SyntaxKind::INDEX)
                        .and_then(|node| create_ref_symbol(db, &node, region, SymbolKind::BlockRef))
                {
                    if let Some(def_key) = resolve_block_def(&symbol, &symbols) {
                        resolved.insert(symbol.key, def_key);
                    }
                    symbols.insert(symbol.key, symbol);
                }
            }
            _ => {}
        });

        resolved.extend(symbols.values().filter_map(|symbol| {
            let defs = match symbol.kind {
                SymbolKind::Call => {
                    if symbol.region != module_key {
                        return None;
                    }
                    &funcs
                }
                SymbolKind::TypeUse => {
                    if symbol.region != module_key {
                        return None;
                    }
                    &types
                }
                SymbolKind::GlobalRef => {
                    if symbol.region != module_key {
                        return None;
                    }
                    &globals
                }
                SymbolKind::MemoryRef => {
                    if symbol.region != module_key {
                        return None;
                    }
                    &memories
                }
                SymbolKind::TableRef => {
                    if symbol.region != module_key {
                        return None;
                    }
                    &tables
                }
                SymbolKind::FieldRef => {
                    let type_use = symbols.get(&symbol.region)?;
                    if type_use.region != module_key {
                        return None;
                    }
                    let (struct_def_key, _) = search_def(&types, type_use.idx)?;
                    fields.get(struct_def_key)?
                }
                SymbolKind::TagRef => {
                    if symbol.region != module_key {
                        return None;
                    }
                    &tags
                }
                SymbolKind::DataRef => {
                    if symbol.region != module_key {
                        return None;
                    }
                    &datas
                }
                _ => return None,
            };
            search_def(defs, symbol.idx).map(|(key, _)| (symbol.key, *key))
        }));

        // replace struct fields' region with their actual region
        symbols.values_mut().for_each(|symbol| {
            if symbol.kind == SymbolKind::FieldRef
                && let Some(def_key) = resolved.get(&symbol.region)
            {
                symbol.region = *def_key;
            }
        });

        // bind parameters that are defined via type use like `(type 0)`
        resolved.reserve(indirect_params.len());
        indirect_params
            .into_iter()
            .for_each(|(type_use_key, param_ref_key, idx)| {
                if let Some(def_symbol) = resolved.get(&type_use_key).and_then(|type_def_key| {
                    symbols
                        .values()
                        .filter(|symbol| symbol.kind == SymbolKind::Param && &symbol.region == type_def_key)
                        .nth(idx as usize)
                }) {
                    resolved.insert(param_ref_key, def_symbol.key);
                }
            });
    });

    SymbolTable {
        symbols,
        resolved,
        def_poi,
    }
}

impl<'db> SymbolTable<'db> {
    pub fn find_def(&'db self, key: SymbolKey) -> Option<&'db Symbol<'db>> {
        self.resolved.get(&key).and_then(|def_key| self.symbols.get(def_key))
    }

    pub fn get_declared(&self, node: SyntaxNode, kind: SymbolKind) -> impl Iterator<Item = &Symbol<'db>> {
        let key = SymbolKey::new(&node);
        self.symbols
            .values()
            .filter(move |symbol| symbol.kind == kind && symbol.region == key)
    }

    pub fn find_references_on_def(
        &self,
        def_symbol: &Symbol<'db>,
        with_decl: bool,
    ) -> impl Iterator<Item = &Symbol<'db>> {
        debug_assert_ne!(def_symbol.kind, SymbolKind::BlockDef);
        self.symbols.values().filter(move |symbol| {
            if symbol.kind == def_symbol.kind {
                with_decl && symbol == &def_symbol
            } else if symbol.idx_kind == def_symbol.idx_kind {
                self.resolved
                    .get(&symbol.key)
                    .is_some_and(|def_key| def_key == &def_symbol.key)
            } else {
                false
            }
        })
    }

    pub fn find_references_on_ref(
        &self,
        ref_symbol: &Symbol<'db>,
        with_decl: bool,
    ) -> impl Iterator<Item = &Symbol<'db>> {
        debug_assert_ne!(ref_symbol.kind, SymbolKind::BlockRef);
        let def_key = self.resolved.get(&ref_symbol.key);
        self.symbols.values().filter(move |symbol| {
            if symbol.kind == ref_symbol.kind {
                symbol.region == ref_symbol.region
                    && self.resolved.get(&symbol.key).zip(def_key).is_some_and(|(a, b)| a == b)
            } else if symbol.idx_kind == ref_symbol.idx_kind {
                with_decl && def_key.is_some_and(|def_key| def_key == &symbol.key)
            } else {
                false
            }
        })
    }

    pub fn find_block_references(&self, def_key: SymbolKey, with_decl: bool) -> impl Iterator<Item = &Symbol<'db>> {
        if with_decl { self.symbols.get(&def_key) } else { None }
            .into_iter()
            .chain(
                self.resolved
                    .iter()
                    .filter(move |(_, key)| *key == &def_key)
                    .filter_map(|(key, _)| self.symbols.get(key)),
            )
    }

    pub fn find_module(&self, module_id: u32) -> Option<&Symbol<'db>> {
        self.symbols
            .values()
            .find(|symbol| symbol.kind == SymbolKind::Module && symbol.idx.num == Some(module_id))
    }
}
#[salsa::tracked]
impl<'db> SymbolTable<'db> {
    #[salsa::tracked(returns(ref))]
    pub(crate) fn of(db: &'db dyn salsa::Database, document: Document) -> Self {
        create_symbol_table(db, document)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, salsa::Update)]
/// Wrapper type for allowing `SyntaxNodePtr` to be stored in Salsa database.
pub struct SymbolKey(SyntaxNodePtr);
impl SymbolKey {
    pub fn new(node: &SyntaxNode) -> Self {
        SymbolKey(SyntaxNodePtr::new(node))
    }
}
impl From<SyntaxNodePtr> for SymbolKey {
    fn from(ptr: SyntaxNodePtr) -> Self {
        SymbolKey(ptr)
    }
}
impl Deref for SymbolKey {
    type Target = SyntaxNodePtr;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug, salsa::Update)]
pub struct Symbol<'db> {
    pub key: SymbolKey,
    pub green: GreenNode,
    pub region: SymbolKey,
    pub kind: SymbolKind,
    pub idx: Idx<'db>,
    pub idx_kind: IdxKind,
}
impl PartialEq for Symbol<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
impl Eq for Symbol<'_> {}
impl Hash for Symbol<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SymbolKind {
    Module,
    Func,
    Param,
    Local,
    Call,
    LocalRef,
    Type,
    TypeUse,
    GlobalDef,
    GlobalRef,
    MemoryDef,
    MemoryRef,
    TableDef,
    TableRef,
    BlockDef,
    BlockRef,
    FieldDef,
    FieldRef,
    TagDef,
    TagRef,
    DataDef,
    DataRef,
}
impl fmt::Display for SymbolKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SymbolKind::Module => write!(f, "module"),
            SymbolKind::Func | SymbolKind::Call => write!(f, "func"),
            SymbolKind::Param => write!(f, "param"),
            SymbolKind::Local => write!(f, "local"),
            SymbolKind::LocalRef => write!(f, "param or local"),
            SymbolKind::Type | SymbolKind::TypeUse => write!(f, "type"),
            SymbolKind::GlobalDef | SymbolKind::GlobalRef => write!(f, "global"),
            SymbolKind::MemoryDef | SymbolKind::MemoryRef => write!(f, "memory"),
            SymbolKind::TableDef | SymbolKind::TableRef => write!(f, "table"),
            SymbolKind::BlockDef | SymbolKind::BlockRef => write!(f, "label"),
            SymbolKind::FieldDef | SymbolKind::FieldRef => write!(f, "field"),
            SymbolKind::TagDef | SymbolKind::TagRef => write!(f, "tag"),
            SymbolKind::DataDef | SymbolKind::DataRef => write!(f, "data segment"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum IdxKind {
    Module,
    Func,
    Local,
    Type,
    Global,
    Memory,
    Table,
    Block,
    Field,
    Tag,
    Data,
}
impl From<SymbolKind> for IdxKind {
    fn from(value: SymbolKind) -> Self {
        match value {
            SymbolKind::Module => IdxKind::Module,
            SymbolKind::Func | SymbolKind::Call => IdxKind::Func,
            SymbolKind::Param | SymbolKind::Local | SymbolKind::LocalRef => IdxKind::Local,
            SymbolKind::Type | SymbolKind::TypeUse => IdxKind::Type,
            SymbolKind::GlobalDef | SymbolKind::GlobalRef => IdxKind::Global,
            SymbolKind::MemoryDef | SymbolKind::MemoryRef => IdxKind::Memory,
            SymbolKind::TableDef | SymbolKind::TableRef => IdxKind::Table,
            SymbolKind::BlockDef | SymbolKind::BlockRef => IdxKind::Block,
            SymbolKind::FieldDef | SymbolKind::FieldRef => IdxKind::Field,
            SymbolKind::TagDef | SymbolKind::TagRef => IdxKind::Tag,
            SymbolKind::DataDef | SymbolKind::DataRef => IdxKind::Data,
        }
    }
}
impl fmt::Display for IdxKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdxKind::Module => write!(f, "module"),
            IdxKind::Func => write!(f, "func"),
            IdxKind::Local => write!(f, "param or local"),
            IdxKind::Type => write!(f, "type"),
            IdxKind::Global => write!(f, "global"),
            IdxKind::Memory => write!(f, "memory"),
            IdxKind::Table => write!(f, "table"),
            IdxKind::Block => write!(f, "label"),
            IdxKind::Field => write!(f, "field"),
            IdxKind::Tag => write!(f, "tag"),
            IdxKind::Data => write!(f, "data segment"),
        }
    }
}

type Symbols<'db> = IndexMap<SymbolKey, Symbol<'db>, FxBuildHasher>;
