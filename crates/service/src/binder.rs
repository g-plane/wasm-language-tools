use crate::{
    document::Document,
    helpers::{self, BumpCollectionsExt, BumpHashMap},
    idx::{Idx, IdxGen, InternIdent},
};
use bumpalo::{Bump, collections::Vec as BumpVec};
use hashbrown::HashTable;
use rustc_hash::{FxBuildHasher, FxHashMap};
use smallvec::SmallVec;
use std::{
    borrow::Borrow,
    fmt,
    hash::{BuildHasher, Hash},
    num::NonZeroU32,
    ops::Deref,
    slice,
};
use wat_syntax::{
    AmberNode, GreenNode, NodeOrToken, SyntaxKind, SyntaxNode, SyntaxNodePtr, TextRange,
    ast::{AstNode, ExternType, ValType},
};

#[derive(Clone, Debug, PartialEq, Eq, salsa::SalsaValue)]
pub(crate) struct SymbolTable<'db> {
    pub symbols: Symbols<'db>,
    resolved: Resolved,
    pub modules: FxHashMap<SymbolKey, ModuleDefSymbols>,
    type_nodes: FxHashMap<SymbolKey, (GreenNode, TextRange)>,
}
fn create_symbol_table<'db>(db: &'db dyn salsa::Database, document: Document) -> SymbolTable<'db> {
    fn create_module_level_symbol<'db>(
        db: &'db dyn salsa::Database,
        node: AmberNode,
        id: u32,
        kind: SymbolKind,
        module_key: SymbolKey,
    ) -> Symbol<'db> {
        Symbol {
            key: node.into(),
            green: node.green().clone(),
            region: module_key,
            kind,
            idx: Idx {
                num: Some(id),
                name: node
                    .tokens_by_kind(SyntaxKind::IDENT)
                    .next()
                    .map(|token| InternIdent::new(db, token.text())),
            },
        }
    }
    fn create_ref_symbol<'db>(
        db: &'db dyn salsa::Database,
        node: AmberNode,
        region: SymbolKey,
        kind: SymbolKind,
    ) -> Option<Symbol<'db>> {
        node.green()
            .children()
            .find_map(|node_or_token| match node_or_token {
                NodeOrToken::Token(token) => match token.kind() {
                    SyntaxKind::IDENT => Some(Idx {
                        num: None,
                        name: Some(InternIdent::new(db, token.text())),
                    }),
                    SyntaxKind::UNSIGNED_INT | SyntaxKind::INT => {
                        helpers::parse_u32(token.text()).ok().map(|num| Idx {
                            num: Some(num),
                            name: None,
                        })
                    }
                    _ => None,
                },
                _ => None,
            })
            .map(|idx| Symbol {
                key: node.into(),
                green: node.green().clone(),
                region,
                kind,
                idx,
            })
    }
    fn create_optional_ref_symbol<'db>(
        db: &'db dyn salsa::Database,
        node: Option<AmberNode>,
        fallback_node: AmberNode,
        region: SymbolKey,
        kind: SymbolKind,
    ) -> Symbol<'db> {
        node.and_then(|node| create_ref_symbol(db, node, region, kind))
            .unwrap_or_else(|| Symbol {
                green: fallback_node.green().clone(),
                key: fallback_node.into(),
                region,
                kind,
                idx: Idx {
                    num: Some(0),
                    name: None,
                },
            })
    }
    fn create_extern_type_symbol<'db>(
        db: &'db dyn salsa::Database,
        node: AmberNode,
        id: u32,
        kind: SymbolKind,
        module_key: SymbolKey,
        ty: AmberNode,
    ) -> Symbol<'db> {
        Symbol {
            key: node.into(),
            green: node.green().clone(),
            region: module_key,
            kind,
            idx: Idx {
                num: Some(id),
                name: ty
                    .tokens_by_kind(SyntaxKind::IDENT)
                    .next()
                    .map(|token| InternIdent::new(db, token.text())),
            },
        }
    }
    fn search_def<'a, 'db>(
        defs: &'a [(SymbolKey, Option<InternIdent<'db>>, u32)],
        idx: Idx,
    ) -> Option<&'a (SymbolKey, Option<InternIdent<'db>>, u32)> {
        idx.num.and_then(|num| defs.get(num as usize)).or_else(|| {
            idx.name.and_then(|name| {
                defs.iter()
                    .find(|(_, def_name, _)| def_name.is_some_and(|def_name| def_name == name))
            })
        })
    }
    fn find_up_blocks<'a>(stack: &[(AmberNode<'a>, usize)]) -> impl Iterator<Item = AmberNode<'a>> {
        stack
            .iter()
            .rev()
            .filter(|(node, _)| {
                matches!(
                    node.kind(),
                    SyntaxKind::BLOCK_BLOCK
                        | SyntaxKind::BLOCK_LOOP
                        | SyntaxKind::BLOCK_IF
                        | SyntaxKind::BLOCK_TRY_TABLE
                        | SyntaxKind::MODULE_FIELD_FUNC
                )
            })
            .map(|(node, _)| *node)
    }
    fn resolve_block_def(
        symbol: &Symbol,
        symbols: &Symbols,
        node_stack: &[(AmberNode, usize)],
        skip_current: bool,
    ) -> Option<SymbolKey> {
        let mut blocks = find_up_blocks(node_stack);
        if skip_current {
            blocks.next();
        }
        if let Some(num) = symbol.idx.num {
            blocks.nth(num as usize).map(|block| block.into())
        } else if let Some(name) = symbol.idx.name {
            blocks.find_map(|block| {
                let key = SymbolKey::from(block);
                if matches!(
                    key.kind(),
                    SyntaxKind::BLOCK_BLOCK
                        | SyntaxKind::BLOCK_LOOP
                        | SyntaxKind::BLOCK_IF
                        | SyntaxKind::BLOCK_TRY_TABLE
                ) && symbols
                    .get(key)
                    .and_then(|other| other.idx.name)
                    .is_some_and(|other| other == name)
                {
                    Some(key)
                } else {
                    None
                }
            })
        } else {
            None
        }
    }

    let root = AmberNode::new_root(document.root(db));
    let mut symbols = Symbols {
        values: Vec::with_capacity(usize::max(64, usize::from(root.text_range().len()) / 80)),
        indices: HashTable::with_capacity(usize::max(64, usize::from(root.text_range().len()) / 128)),
        build_hasher: FxBuildHasher,
    };
    let mut resolved = Vec::new();
    let mut modules = FxHashMap::with_capacity_and_hasher(1, FxBuildHasher);
    let mut type_nodes = FxHashMap::default();
    let bump = Bump::new();
    let mut pre_resolved = BumpHashMap::new_in(&bump);
    root.children().enumerate().for_each(|(module_id, module)| {
        let module_key = module.into();
        let module_index = symbols.values.len() as u32;
        symbols.insert(Symbol {
            green: module.green().clone(),
            key: module_key,
            region: root.into(),
            kind: SymbolKind::Module,
            idx: Idx {
                num: Some(module_id as u32),
                name: module
                    .tokens_by_kind(SyntaxKind::IDENT)
                    .next()
                    .map(|token| InternIdent::new(db, token.text())),
            },
        });
        let mut func_idx_gen = IdxGen::default();
        let mut local_idx_gen = IdxGen::default();
        let mut type_idx_gen = IdxGen::default();
        let mut global_idx_gen = IdxGen::default();
        let mut mem_idx_gen = IdxGen::default();
        let mut table_idx_gen = IdxGen::default();
        let mut field_idx_gen = IdxGen::default();
        let mut tag_idx_gen = IdxGen::default();
        let mut data_idx_gen = IdxGen::default();
        let mut elem_idx_gen = IdxGen::default();

        let mut funcs = BumpVec::new_in(&bump);
        let mut locals = BumpVec::new_in(&bump);
        let mut types = BumpVec::new_in(&bump);
        let mut globals = BumpVec::new_in(&bump);
        let mut memories = BumpVec::new_in(&bump);
        let mut tables = BumpVec::new_in(&bump);
        let mut fields = BumpHashMap::new_in(&bump);
        let mut tags = BumpVec::new_in(&bump);
        let mut datas = BumpVec::new_in(&bump);
        let mut elems = BumpVec::new_in(&bump);
        let mut indirect_params = BumpVec::<(SymbolKey, _, _)>::new_in(&bump);

        let mut node_stack = BumpVec::with_capacity_in(8, &bump);
        node_stack.push((module, 0));
        while let Some((parent, index)) = node_stack.last_mut() {
            match parent.child_or_token_at(*index) {
                Some(NodeOrToken::Node(node)) => {
                    match node.kind() {
                        SyntaxKind::MODULE_FIELD_FUNC => {
                            let func_idx = func_idx_gen.pull();
                            let symbol = create_module_level_symbol(db, node, func_idx, SymbolKind::Func, module_key);
                            let func_key = symbol.key;
                            funcs.push((func_key, symbol.idx.name, symbols.values.len() as u32));
                            symbols.insert(symbol);
                            locals.clear();
                            local_idx_gen.reset();
                        }
                        SyntaxKind::PARAM => 'param: {
                            let region = if let Some(node) = node_stack
                                .len()
                                .checked_sub(2)
                                .and_then(|i| node_stack.get(i))
                                .and_then(|(node, _)| match node.kind() {
                                    SyntaxKind::MODULE_FIELD_FUNC => Some(node),
                                    SyntaxKind::SUB_TYPE => node_stack
                                        .len()
                                        .checked_sub(3)
                                        .and_then(|i| node_stack.get(i))
                                        .map(|(node, _)| node),
                                    _ => None,
                                }) {
                                (*node).into()
                            } else {
                                break 'param;
                            };
                            if let Some(ident) = node.tokens_by_kind(SyntaxKind::IDENT).next() {
                                let key = node.into();
                                let idx = local_idx_gen.pull();
                                let name = InternIdent::new(db, ident.text());
                                locals.push((key, Some(name), symbols.values.len() as u32));
                                symbols.insert(Symbol {
                                    key,
                                    green: node.green().clone(),
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
                                });
                            } else {
                                node.children_by_kind(ValType::can_cast).for_each(|val_type| {
                                    let key = val_type.into();
                                    locals.push((key, None, symbols.values.len() as u32));
                                    symbols.insert(Symbol {
                                        key,
                                        green: val_type.green().clone(),
                                        region,
                                        kind: SymbolKind::Param,
                                        idx: Idx {
                                            num: Some(local_idx_gen.pull()),
                                            name: None,
                                        },
                                    });
                                });
                            }
                        }
                        SyntaxKind::LOCAL => {
                            let func_key = (*parent).into();
                            if let Some(ident) = node.tokens_by_kind(SyntaxKind::IDENT).next() {
                                let key = node.into();
                                let idx = local_idx_gen.pull();
                                let name = InternIdent::new(db, ident.text());
                                locals.push((key, Some(name), symbols.values.len() as u32));
                                symbols.insert(Symbol {
                                    key,
                                    green: node.green().clone(),
                                    region: func_key,
                                    kind: SymbolKind::Local,
                                    idx: Idx {
                                        num: Some(idx),
                                        name: Some(name),
                                    },
                                });
                            } else {
                                node.children_by_kind(ValType::can_cast).for_each(|val_type| {
                                    let key = val_type.into();
                                    locals.push((key, None, symbols.values.len() as u32));
                                    symbols.insert(Symbol {
                                        key,
                                        green: val_type.green().clone(),
                                        region: func_key,
                                        kind: SymbolKind::Local,
                                        idx: Idx {
                                            num: Some(local_idx_gen.pull()),
                                            name: None,
                                        },
                                    })
                                });
                            }
                        }
                        SyntaxKind::TYPE_DEF => {
                            let type_idx = type_idx_gen.pull();
                            let symbol = create_module_level_symbol(db, node, type_idx, SymbolKind::Type, module_key);
                            let type_def_key = symbol.key;
                            types.push((type_def_key, symbol.idx.name, symbols.values.len() as u32));
                            symbols.insert(symbol);
                        }
                        SyntaxKind::FUNC_TYPE => {
                            locals.clear();
                            local_idx_gen.reset();
                        }
                        SyntaxKind::STRUCT_TYPE => {
                            field_idx_gen.reset();
                        }
                        SyntaxKind::FIELD => 'field: {
                            let type_def_key = if let Some((type_def, _)) = node_stack
                                .iter()
                                .find(|(ancestor, _)| ancestor.kind() == SyntaxKind::TYPE_DEF)
                            {
                                (*type_def).into()
                            } else {
                                break 'field;
                            };
                            let fields = fields
                                .entry(type_def_key)
                                .or_insert_with(|| BumpVec::with_capacity_in(1, &bump));
                            if let Some(ident) = node.tokens_by_kind(SyntaxKind::IDENT).next() {
                                let key = node.into();
                                let idx = field_idx_gen.pull();
                                let name = InternIdent::new(db, ident.text());
                                fields.push((key, Some(name), symbols.values.len() as u32));
                                symbols.insert(Symbol {
                                    key,
                                    green: node.green().clone(),
                                    region: type_def_key,
                                    kind: SymbolKind::FieldDef,
                                    idx: Idx {
                                        num: Some(idx),
                                        name: Some(name),
                                    },
                                });
                            } else {
                                node.children_by_kind(SyntaxKind::FIELD_TYPE).for_each(|field_type| {
                                    let key = field_type.into();
                                    fields.push((key, None, symbols.values.len() as u32));
                                    symbols.insert(Symbol {
                                        key,
                                        green: field_type.green().clone(),
                                        region: type_def_key,
                                        kind: SymbolKind::FieldDef,
                                        idx: Idx {
                                            num: Some(field_idx_gen.pull()),
                                            name: None,
                                        },
                                    });
                                });
                            }
                        }
                        SyntaxKind::MODULE_FIELD_GLOBAL => {
                            let idx = global_idx_gen.pull();
                            let symbol = create_module_level_symbol(db, node, idx, SymbolKind::GlobalDef, module_key);
                            globals.push((symbol.key, symbol.idx.name, symbols.values.len() as u32));
                            symbols.insert(symbol);
                        }
                        SyntaxKind::PLAIN_INSTR => 'instr: {
                            match node
                                .tokens_by_kind(SyntaxKind::INSTR_NAME)
                                .next()
                                .map(|token| token.text())
                            {
                                Some("call" | "ref.func" | "return_call") => {
                                    symbols.extend(
                                        node.children().filter_map(|node| {
                                            create_ref_symbol(db, node, module_key, SymbolKind::Call)
                                        }),
                                    );
                                }
                                Some("local.get" | "local.set" | "local.tee") => {
                                    // invariant: node stack is [module, module field, ..]
                                    // but someone can put `local.*` in global initialization expr
                                    let Some((func, _)) = node_stack
                                        .get(1)
                                        .filter(|(node, _)| node.kind() == SyntaxKind::MODULE_FIELD_FUNC)
                                    else {
                                        break 'instr;
                                    };
                                    let region = (*func).into();
                                    node.children()
                                        .filter_map(|node| create_ref_symbol(db, node, region, SymbolKind::LocalRef))
                                        .for_each(|symbol| {
                                            let index = symbols.values.len() as u32;
                                            if let Some((def_key, ..)) = search_def(&locals, symbol.idx) {
                                                pre_resolved.insert(index, *def_key);
                                            } else if let Some(num) = symbol.idx.num
                                                && let Some(idx) = helpers::syntax::pick_type_idx_from_func(*func)
                                            {
                                                indirect_params.push((idx.into(), index, num));
                                            }
                                            symbols.insert(symbol);
                                        });
                                }
                                Some("global.get" | "global.set") => {
                                    symbols.extend(node.children().filter_map(|node| {
                                        create_ref_symbol(db, node, module_key, SymbolKind::GlobalRef)
                                    }));
                                }
                                Some(
                                    "br" | "br_if" | "br_table" | "br_on_null" | "br_on_non_null" | "br_on_cast"
                                    | "br_on_cast_fail",
                                ) => {
                                    if let Some(region) = find_up_blocks(&node_stack).next().map(|node| node.into()) {
                                        node.children()
                                            .filter_map(|node| {
                                                create_ref_symbol(db, node, region, SymbolKind::BlockRef)
                                            })
                                            .for_each(|symbol| {
                                                if let Some(def_key) =
                                                    resolve_block_def(&symbol, &symbols, &node_stack, false)
                                                {
                                                    pre_resolved.insert(symbols.values.len() as u32, def_key);
                                                }
                                                symbols.insert(symbol);
                                            });
                                    }
                                }
                                Some("call_indirect" | "return_call_indirect") => {
                                    let immediate = node.children_by_kind(SyntaxKind::IMMEDIATE).next();
                                    let symbol = create_optional_ref_symbol(
                                        db,
                                        immediate,
                                        node,
                                        module_key,
                                        SymbolKind::TableRef,
                                    );
                                    symbols.insert(symbol);
                                }
                                Some("table.get" | "table.set" | "table.size" | "table.grow" | "table.fill") => {
                                    let immediate = node.children_by_kind(SyntaxKind::IMMEDIATE).next();
                                    let symbol = create_optional_ref_symbol(
                                        db,
                                        immediate,
                                        node,
                                        module_key,
                                        SymbolKind::TableRef,
                                    );
                                    symbols.insert(symbol);
                                }
                                Some("table.copy") => {
                                    let mut immediates = node.children_by_kind(SyntaxKind::IMMEDIATE);
                                    let dst = create_optional_ref_symbol(
                                        db,
                                        immediates.next(),
                                        node,
                                        module_key,
                                        SymbolKind::TableRef,
                                    );
                                    symbols.insert(dst);
                                    let src = create_optional_ref_symbol(
                                        db,
                                        immediates.next(),
                                        node,
                                        module_key,
                                        SymbolKind::TableRef,
                                    );
                                    symbols.insert(src);
                                }
                                Some("table.init") => {
                                    let mut immediates = node.children_by_kind(SyntaxKind::IMMEDIATE);
                                    let mut first = immediates.next();
                                    if let Some(elem_ref) = immediates.next().or_else(|| first.take())
                                        && let Some(symbol) =
                                            create_ref_symbol(db, elem_ref, module_key, SymbolKind::ElemRef)
                                    {
                                        symbols.insert(symbol);
                                    }
                                    let table_symbol =
                                        create_optional_ref_symbol(db, first, node, module_key, SymbolKind::TableRef);
                                    symbols.insert(table_symbol);
                                }
                                Some("elem.drop") => {
                                    if let Some(symbol) = node
                                        .children_by_kind(SyntaxKind::IMMEDIATE)
                                        .next()
                                        .and_then(|node| create_ref_symbol(db, node, module_key, SymbolKind::ElemRef))
                                    {
                                        symbols.insert(symbol);
                                    }
                                }
                                Some(
                                    "memory.size" | "memory.grow" | "memory.fill" | "i32.load" | "i64.load"
                                    | "f32.load" | "f64.load" | "i32.load8_s" | "i32.load8_u" | "i32.load16_s"
                                    | "i32.load16_u" | "i64.load8_s" | "i64.load8_u" | "i64.load16_s" | "i64.load16_u"
                                    | "i64.load32_s" | "i64.load32_u" | "i32.store" | "i64.store" | "f32.store"
                                    | "f64.store" | "i32.store8" | "i32.store16" | "i64.store8" | "i64.store16"
                                    | "i64.store32" | "v128.load" | "v128.load8x8_s" | "v128.load8x8_u"
                                    | "v128.load16x4_s" | "v128.load16x4_u" | "v128.load32x2_s" | "v128.load32x2_u"
                                    | "v128.load8_splat" | "v128.load16_splat" | "v128.load32_splat"
                                    | "v128.load64_splat" | "v128.load32_zero" | "v128.load64_zero" | "v128.store"
                                    | "v128.load8_lane" | "v128.load16_lane" | "v128.load32_lane" | "v128.load64_lane"
                                    | "v128.store8_lane" | "v128.store16_lane" | "v128.store32_lane"
                                    | "v128.store64_lane",
                                ) => {
                                    let immediate = node.children_by_kind(SyntaxKind::IMMEDIATE).next();
                                    let symbol = create_optional_ref_symbol(
                                        db,
                                        immediate,
                                        node,
                                        module_key,
                                        SymbolKind::MemoryRef,
                                    );
                                    symbols.insert(symbol);
                                }
                                Some("memory.init") => {
                                    let mut immediates = node.children_by_kind(SyntaxKind::IMMEDIATE);
                                    let mut first = immediates.next();
                                    if let Some(data_ref) = immediates.next().or_else(|| first.take())
                                        && let Some(symbol) =
                                            create_ref_symbol(db, data_ref, module_key, SymbolKind::DataRef)
                                    {
                                        symbols.insert(symbol);
                                    }
                                    let mem_symbol =
                                        create_optional_ref_symbol(db, first, node, module_key, SymbolKind::MemoryRef);
                                    symbols.insert(mem_symbol);
                                }
                                Some("memory.copy") => {
                                    let mut immediates = node.children_by_kind(SyntaxKind::IMMEDIATE);
                                    let dst = create_optional_ref_symbol(
                                        db,
                                        immediates.next(),
                                        node,
                                        module_key,
                                        SymbolKind::MemoryRef,
                                    );
                                    symbols.insert(dst);
                                    let src = create_optional_ref_symbol(
                                        db,
                                        immediates.next(),
                                        node,
                                        module_key,
                                        SymbolKind::MemoryRef,
                                    );
                                    symbols.insert(src);
                                }
                                Some("data.drop") => {
                                    if let Some(symbol) = node
                                        .children_by_kind(SyntaxKind::IMMEDIATE)
                                        .next()
                                        .and_then(|node| create_ref_symbol(db, node, module_key, SymbolKind::DataRef))
                                    {
                                        symbols.insert(symbol);
                                    }
                                }
                                Some(
                                    "struct.new" | "struct.new_default" | "array.new" | "array.new_default"
                                    | "array.new_fixed" | "array.get" | "array.get_u" | "array.get_s" | "array.set"
                                    | "array.fill" | "call_ref" | "return_call_ref" | "ref.null" | "cont.new"
                                    | "resume" | "resume_throw_ref",
                                ) => {
                                    if let Some(symbol) = node
                                        .children_by_kind(SyntaxKind::IMMEDIATE)
                                        .next()
                                        .and_then(|node| create_ref_symbol(db, node, module_key, SymbolKind::TypeUse))
                                    {
                                        symbols.insert(symbol);
                                    }
                                }
                                Some("array.copy" | "cont.bind") => {
                                    symbols.extend(node.children().filter_map(|node| {
                                        create_ref_symbol(db, node, module_key, SymbolKind::TypeUse)
                                    }));
                                }
                                Some("array.new_data" | "array.init_data") => {
                                    let mut immediates = node.children_by_kind(SyntaxKind::IMMEDIATE);
                                    if let Some(symbol) = immediates
                                        .next()
                                        .and_then(|node| create_ref_symbol(db, node, module_key, SymbolKind::TypeUse))
                                    {
                                        symbols.insert(symbol);
                                    }
                                    if let Some(symbol) = immediates
                                        .next()
                                        .and_then(|node| create_ref_symbol(db, node, module_key, SymbolKind::DataRef))
                                    {
                                        symbols.insert(symbol);
                                    }
                                }
                                Some("array.new_elem" | "array.init_elem") => {
                                    let mut immediates = node.children_by_kind(SyntaxKind::IMMEDIATE);
                                    if let Some(symbol) = immediates
                                        .next()
                                        .and_then(|node| create_ref_symbol(db, node, module_key, SymbolKind::TypeUse))
                                    {
                                        symbols.insert(symbol);
                                    }
                                    if let Some(symbol) = immediates
                                        .next()
                                        .and_then(|node| create_ref_symbol(db, node, module_key, SymbolKind::ElemRef))
                                    {
                                        symbols.insert(symbol);
                                    }
                                }
                                Some("struct.get" | "struct.get_s" | "struct.get_u" | "struct.set") => {
                                    let mut children = node.children();
                                    if let Some(symbol) = children
                                        .next()
                                        .and_then(|node| create_ref_symbol(db, node, module_key, SymbolKind::TypeUse))
                                    {
                                        let key = symbol.key;
                                        symbols.insert(symbol);
                                        if let Some(symbol) = children.next().and_then(|node| {
                                            // The region here is temporary.
                                            // It's used for tracking which struct it belongs to,
                                            // and it will be replaced with the actual region later.
                                            // If the struct it belongs to isn't defined, nothing will happen.
                                            create_ref_symbol(db, node, key, SymbolKind::FieldRef)
                                        }) {
                                            symbols.insert(symbol);
                                        }
                                    }
                                }
                                Some("throw" | "suspend") => {
                                    if let Some(symbol) = node
                                        .children_by_kind(SyntaxKind::IMMEDIATE)
                                        .next()
                                        .and_then(|node| create_ref_symbol(db, node, module_key, SymbolKind::TagRef))
                                    {
                                        symbols.insert(symbol);
                                    }
                                }
                                Some("resume_throw" | "switch") => {
                                    let mut immediates = node.children_by_kind(SyntaxKind::IMMEDIATE);
                                    if let Some(symbol) = immediates
                                        .next()
                                        .and_then(|node| create_ref_symbol(db, node, module_key, SymbolKind::TypeUse))
                                    {
                                        symbols.insert(symbol);
                                    }
                                    if let Some(symbol) = immediates
                                        .next()
                                        .and_then(|node| create_ref_symbol(db, node, module_key, SymbolKind::TagRef))
                                    {
                                        symbols.insert(symbol);
                                    }
                                }
                                _ => {}
                            }
                        }
                        SyntaxKind::BLOCK_BLOCK
                        | SyntaxKind::BLOCK_IF
                        | SyntaxKind::BLOCK_LOOP
                        | SyntaxKind::BLOCK_TRY_TABLE => {
                            if let Some(symbol) = find_up_blocks(&node_stack).next().map(|region| Symbol {
                                key: node.into(),
                                green: node.green().clone(),
                                region: region.into(),
                                kind: SymbolKind::BlockDef,
                                idx: Idx {
                                    num: Some(0), // fake ID
                                    name: node
                                        .tokens_by_kind(SyntaxKind::IDENT)
                                        .next()
                                        .map(|token| InternIdent::new(db, token.text())),
                                },
                            }) {
                                symbols.insert(symbol);
                            }
                        }
                        SyntaxKind::MODULE_FIELD_START | SyntaxKind::EXTERN_IDX_FUNC => {
                            if let Some(symbol) = node
                                .children_by_kind(SyntaxKind::INDEX)
                                .next()
                                .and_then(|index| create_ref_symbol(db, index, module_key, SymbolKind::Call))
                            {
                                symbols.insert(symbol);
                            }
                        }
                        SyntaxKind::TYPE_USE | SyntaxKind::HEAP_TYPE | SyntaxKind::SUB_TYPE | SyntaxKind::CONT_TYPE => {
                            if let Some(symbol) = node
                                .children_by_kind(SyntaxKind::INDEX)
                                .next()
                                .and_then(|index| create_ref_symbol(db, index, module_key, SymbolKind::TypeUse))
                            {
                                symbols.insert(symbol);
                            }
                        }
                        SyntaxKind::MODULE_FIELD_MEMORY => {
                            let idx = mem_idx_gen.pull();
                            let symbol = create_module_level_symbol(db, node, idx, SymbolKind::MemoryDef, module_key);
                            memories.push((symbol.key, symbol.idx.name, symbols.values.len() as u32));
                            symbols.insert(symbol);
                        }
                        SyntaxKind::MODULE_FIELD_TABLE => {
                            let idx = table_idx_gen.pull();
                            let symbol = create_module_level_symbol(db, node, idx, SymbolKind::TableDef, module_key);
                            tables.push((symbol.key, symbol.idx.name, symbols.values.len() as u32));
                            symbols.insert(symbol);
                        }
                        SyntaxKind::MODULE_FIELD_TAG => {
                            let idx = tag_idx_gen.pull();
                            let symbol = create_module_level_symbol(db, node, idx, SymbolKind::TagDef, module_key);
                            tags.push((symbol.key, symbol.idx.name, symbols.values.len() as u32));
                            symbols.insert(symbol);
                        }
                        SyntaxKind::EXTERN_IDX_GLOBAL => {
                            if let Some(symbol) = node
                                .children_by_kind(SyntaxKind::INDEX)
                                .next()
                                .and_then(|index| create_ref_symbol(db, index, module_key, SymbolKind::GlobalRef))
                            {
                                symbols.insert(symbol);
                            }
                        }
                        SyntaxKind::EXTERN_IDX_MEMORY => {
                            if let Some(symbol) = node
                                .children_by_kind(SyntaxKind::INDEX)
                                .next()
                                .and_then(|index| create_ref_symbol(db, index, module_key, SymbolKind::MemoryRef))
                            {
                                symbols.insert(symbol);
                            }
                        }
                        SyntaxKind::EXTERN_IDX_TABLE | SyntaxKind::TABLE_USE => {
                            if let Some(symbol) = node
                                .children_by_kind(SyntaxKind::INDEX)
                                .next()
                                .and_then(|index| create_ref_symbol(db, index, module_key, SymbolKind::TableRef))
                            {
                                symbols.insert(symbol);
                            }
                        }
                        SyntaxKind::EXTERN_IDX_TAG => {
                            if let Some(symbol) = node
                                .children_by_kind(SyntaxKind::INDEX)
                                .next()
                                .and_then(|index| create_ref_symbol(db, index, module_key, SymbolKind::TagRef))
                            {
                                symbols.insert(symbol);
                            }
                        }
                        SyntaxKind::MODULE_FIELD_IMPORT => {
                            let extern_type = node.children_by_kind(ExternType::can_cast).next();
                            node.children_by_kind(SyntaxKind::IMPORT_ITEM)
                                .chain(node.children_by_kind(SyntaxKind::NAME).next().map(|_| node))
                                .filter_map(|node| {
                                    let extern_type =
                                        node.children_by_kind(ExternType::can_cast).next().or(extern_type)?;
                                    Some((node, extern_type))
                                })
                                .for_each(|(node, ty)| match ty.kind() {
                                    SyntaxKind::EXTERN_TYPE_FUNC => {
                                        let idx = func_idx_gen.pull();
                                        let symbol =
                                            create_extern_type_symbol(db, node, idx, SymbolKind::Func, module_key, ty);
                                        funcs.push((symbol.key, symbol.idx.name, symbols.values.len() as u32));
                                        type_nodes.insert(symbol.key, (ty.green().clone(), ty.text_range()));
                                        symbols.insert(symbol);
                                    }
                                    SyntaxKind::EXTERN_TYPE_GLOBAL => {
                                        let idx = global_idx_gen.pull();
                                        let symbol = create_extern_type_symbol(
                                            db,
                                            node,
                                            idx,
                                            SymbolKind::GlobalDef,
                                            module_key,
                                            ty,
                                        );
                                        globals.push((symbol.key, symbol.idx.name, symbols.values.len() as u32));
                                        type_nodes.insert(symbol.key, (ty.green().clone(), ty.text_range()));
                                        symbols.insert(symbol);
                                    }
                                    SyntaxKind::EXTERN_TYPE_MEMORY => {
                                        let idx = mem_idx_gen.pull();
                                        let symbol = create_extern_type_symbol(
                                            db,
                                            node,
                                            idx,
                                            SymbolKind::MemoryDef,
                                            module_key,
                                            ty,
                                        );
                                        memories.push((symbol.key, symbol.idx.name, symbols.values.len() as u32));
                                        type_nodes.insert(symbol.key, (ty.green().clone(), ty.text_range()));
                                        symbols.insert(symbol);
                                    }
                                    SyntaxKind::EXTERN_TYPE_TABLE => {
                                        let idx = table_idx_gen.pull();
                                        let symbol = create_extern_type_symbol(
                                            db,
                                            node,
                                            idx,
                                            SymbolKind::TableDef,
                                            module_key,
                                            ty,
                                        );
                                        tables.push((symbol.key, symbol.idx.name, symbols.values.len() as u32));
                                        type_nodes.insert(symbol.key, (ty.green().clone(), ty.text_range()));
                                        symbols.insert(symbol);
                                    }
                                    SyntaxKind::EXTERN_TYPE_TAG => {
                                        let idx = tag_idx_gen.pull();
                                        let symbol = create_extern_type_symbol(
                                            db,
                                            node,
                                            idx,
                                            SymbolKind::TagDef,
                                            module_key,
                                            ty,
                                        );
                                        tags.push((symbol.key, symbol.idx.name, symbols.values.len() as u32));
                                        type_nodes.insert(symbol.key, (ty.green().clone(), ty.text_range()));
                                        symbols.insert(symbol);
                                    }
                                    _ => {}
                                });
                        }
                        SyntaxKind::MODULE_FIELD_DATA => {
                            let idx = data_idx_gen.pull();
                            let symbol = create_module_level_symbol(db, node, idx, SymbolKind::DataDef, module_key);
                            datas.push((symbol.key, symbol.idx.name, symbols.values.len() as u32));
                            symbols.insert(symbol);
                        }
                        SyntaxKind::MODULE_FIELD_ELEM
                            if node.tokens_by_kind(SyntaxKind::MODIFIER_KEYWORD).next().is_none() =>
                        {
                            let idx = elem_idx_gen.pull();
                            let symbol = create_module_level_symbol(db, node, idx, SymbolKind::ElemDef, module_key);
                            elems.push((symbol.key, symbol.idx.name, symbols.values.len() as u32));
                            symbols.insert(symbol);
                        }
                        SyntaxKind::MEM_USE => {
                            if let Some(symbol) = node
                                .children_by_kind(SyntaxKind::INDEX)
                                .next()
                                .and_then(|index| create_ref_symbol(db, index, module_key, SymbolKind::MemoryRef))
                            {
                                symbols.insert(symbol);
                            }
                        }
                        SyntaxKind::ELEM_LIST => {
                            symbols.extend(
                                node.children_by_kind(SyntaxKind::INDEX)
                                    .filter_map(|index| create_ref_symbol(db, index, module_key, SymbolKind::Call)),
                            );
                        }
                        SyntaxKind::CATCH => {
                            let mut children = node.children();
                            if let Some(symbol) = children
                                .next()
                                .and_then(|node| create_ref_symbol(db, node, module_key, SymbolKind::TagRef))
                            {
                                symbols.insert(symbol);
                            }
                            if let Some(region) = find_up_blocks(&node_stack).nth(1).map(|node| node.into())
                                && let Some(symbol) = children
                                    .next()
                                    .and_then(|node| create_ref_symbol(db, node, region, SymbolKind::BlockRef))
                            {
                                if let Some(def_key) = resolve_block_def(&symbol, &symbols, &node_stack, true) {
                                    pre_resolved.insert(symbols.values.len() as u32, def_key);
                                }
                                symbols.insert(symbol);
                            }
                        }
                        SyntaxKind::CATCH_ALL => {
                            if let Some(region) = find_up_blocks(&node_stack).nth(1).map(|node| node.into())
                                && let Some(symbol) = node
                                    .children_by_kind(SyntaxKind::INDEX)
                                    .next()
                                    .and_then(|node| create_ref_symbol(db, node, region, SymbolKind::BlockRef))
                            {
                                if let Some(def_key) = resolve_block_def(&symbol, &symbols, &node_stack, true) {
                                    pre_resolved.insert(symbols.values.len() as u32, def_key);
                                }
                                symbols.insert(symbol);
                            }
                        }
                        SyntaxKind::ON_CLAUSE => {
                            let mut indexes = node.children_by_kind(SyntaxKind::INDEX);
                            if let Some(symbol) = indexes
                                .next()
                                .and_then(|node| create_ref_symbol(db, node, module_key, SymbolKind::TagRef))
                            {
                                symbols.insert(symbol);
                            }
                            if let Some(index) = indexes.next()
                                && let Some(region) = find_up_blocks(&node_stack).next().map(|node| node.into())
                                && let Some(symbol) = create_ref_symbol(db, index, region, SymbolKind::BlockRef)
                            {
                                if let Some(def_key) = resolve_block_def(&symbol, &symbols, &node_stack, false) {
                                    pre_resolved.insert(symbols.values.len() as u32, def_key);
                                }
                                symbols.insert(symbol);
                            }
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

        resolved.reserve(symbols.values.len() - module_index as usize);
        resolved.extend(
            symbols
                .values
                .get(module_index as usize..)
                .into_iter()
                .flatten()
                .map(|symbol| match symbol.kind {
                    SymbolKind::Call => search_def(&funcs, symbol.idx).and_then(|(.., index)| NonZeroU32::new(*index)),
                    SymbolKind::LocalRef | SymbolKind::BlockRef => symbols
                        .get_index_of(symbol.key)
                        .and_then(|index| pre_resolved.get(&index))
                        .and_then(|key| symbols.get_index_of(key))
                        .and_then(NonZeroU32::new),
                    SymbolKind::TypeUse => {
                        search_def(&types, symbol.idx).and_then(|(.., index)| NonZeroU32::new(*index))
                    }
                    SymbolKind::GlobalRef => {
                        search_def(&globals, symbol.idx).and_then(|(.., index)| NonZeroU32::new(*index))
                    }
                    SymbolKind::MemoryRef => {
                        search_def(&memories, symbol.idx).and_then(|(.., index)| NonZeroU32::new(*index))
                    }
                    SymbolKind::TableRef => {
                        search_def(&tables, symbol.idx).and_then(|(.., index)| NonZeroU32::new(*index))
                    }
                    SymbolKind::FieldRef => symbols
                        .get(symbol.region)
                        .and_then(|type_use| search_def(&types, type_use.idx))
                        .and_then(|(struct_def_key, ..)| fields.get(struct_def_key))
                        .and_then(|fields| search_def(fields, symbol.idx))
                        .and_then(|(.., index)| NonZeroU32::new(*index)),
                    SymbolKind::TagRef => search_def(&tags, symbol.idx).and_then(|(.., index)| NonZeroU32::new(*index)),
                    SymbolKind::DataRef => {
                        search_def(&datas, symbol.idx).and_then(|(.., index)| NonZeroU32::new(*index))
                    }
                    SymbolKind::ElemRef => {
                        search_def(&elems, symbol.idx).and_then(|(.., index)| NonZeroU32::new(*index))
                    }
                    _ => None,
                }),
        );

        // replace struct fields' region with their actual region
        BumpVec::from_iter_in(
            symbols.values.iter().enumerate().filter_map(|(i, symbol)| {
                if symbol.kind == SymbolKind::FieldRef
                    && let Some(struct_ref_index) = symbols.get_index_of(symbol.region)
                    && let Some(struct_def_index) = resolved.get(struct_ref_index as usize).and_then(|index| *index)
                    && let Some(struct_def) = symbols.values.get(struct_def_index.get() as usize)
                {
                    Some((i, struct_def.key))
                } else {
                    None
                }
            }),
            &bump,
        )
        .into_iter()
        .for_each(|(i, key)| {
            if let Some(symbol) = symbols.values.get_mut(i) {
                symbol.region = key;
            }
        });

        // bind parameters that are defined via type use like `(type 0)`
        indirect_params.into_iter().for_each(|(type_use_key, param_ref, idx)| {
            if let Some((index, _)) = symbols
                .get_index_of(type_use_key)
                .and_then(|index| resolved.get(index as usize))
                .and_then(|index| *index)
                .and_then(|index| symbols.values.get(index.get() as usize))
                .and_then(|type_def| {
                    symbols
                        .iter()
                        .enumerate()
                        .filter(|(_, symbol)| symbol.kind == SymbolKind::Param && symbol.region == type_def.key)
                        .nth(idx as usize)
                })
                && let Some(param_def) = resolved.get_mut(param_ref as usize)
            {
                *param_def = NonZeroU32::new(index as u32);
            }
        });

        modules.insert(
            module_key,
            ModuleDefSymbols {
                funcs: funcs.into_iter().map(|(key, ..)| key).collect(),
                types: types.into_iter().map(|(key, ..)| key).collect(),
                globals: globals.into_iter().map(|(key, ..)| key).collect(),
                memories: memories.into_iter().map(|(key, ..)| key).collect(),
                tables: tables.into_iter().map(|(key, ..)| key).collect(),
                tags: tags.into_iter().map(|(key, ..)| key).collect(),
                datas: datas.into_iter().map(|(key, ..)| key).collect(),
                elems: elems.into_iter().map(|(key, ..)| key).collect(),
            },
        );
    });

    symbols.values.shrink_to_fit();
    SymbolTable {
        symbols,
        resolved: resolved.into_boxed_slice(),
        modules,
        type_nodes,
    }
}

impl<'db> SymbolTable<'db> {
    pub fn find_def(&'db self, key: SymbolKey) -> Option<&'db Symbol<'db>> {
        self.symbols
            .get_index_of(key)
            .and_then(|index| self.find_def_index(index as usize))
            .and_then(|index| self.symbols.values.get(index as usize))
    }
    fn find_def_index(&self, ref_index: usize) -> Option<u32> {
        self.resolved
            .get(ref_index)
            .and_then(|index| *index)
            .map(|index| index.get())
    }

    pub fn find_def_by_idx(&'db self, idx: Idx<'db>, kind: SymbolKind, module: SymbolKey) -> Option<&'db Symbol<'db>> {
        std::debug_assert_matches!(kind, SymbolKind::Type | SymbolKind::Func);
        let module = self.modules.get(&module)?;
        let declared = match kind {
            SymbolKind::Type => &module.types,
            SymbolKind::Func => &module.funcs,
            _ => return None,
        };
        if let Some(num) = idx.num {
            declared.get(num as usize).and_then(|key| self.symbols.get(key))
        } else if let Some(name) = idx.name {
            declared
                .iter()
                .find_map(|key| self.symbols.get(key).filter(|symbol| symbol.idx.name == Some(name)))
        } else {
            None
        }
    }

    pub fn get_declared(&self, module: &SyntaxNode, kind: SymbolKind) -> impl Iterator<Item = &Symbol<'db>> {
        self.modules
            .get(&SymbolKey::from(module))
            .into_iter()
            .flat_map(move |module| match kind {
                SymbolKind::Func => &*module.funcs,
                SymbolKind::Type => &*module.types,
                SymbolKind::GlobalDef => &*module.globals,
                SymbolKind::MemoryDef => &*module.memories,
                SymbolKind::TableDef => &*module.tables,
                SymbolKind::TagDef => &*module.tags,
                SymbolKind::DataDef => &*module.datas,
                SymbolKind::ElemDef => &*module.elems,
                _ => &[],
            })
            .filter_map(|key| self.symbols.get(key))
    }

    pub fn find_references_on_def(
        &self,
        def_symbol: &Symbol<'db>,
        with_decl: bool,
    ) -> impl Iterator<Item = &Symbol<'db>> {
        self.find_references(self.symbols.get_index_of(def_symbol.key), with_decl)
    }
    pub fn find_references_on_ref(
        &self,
        ref_symbol: &Symbol<'db>,
        with_decl: bool,
    ) -> impl Iterator<Item = &Symbol<'db>> {
        let def_index = self
            .symbols
            .get_index_of(ref_symbol.key)
            .and_then(|index| self.find_def_index(index as usize));
        self.find_references(def_index, with_decl).filter(|symbol| {
            if symbol.kind == SymbolKind::LocalRef {
                // Special case for params defined in type definition, not function.
                // Only consider params in a same function.
                symbol.region == ref_symbol.region
            } else {
                true
            }
        })
    }
    fn find_references(&self, def_index: Option<u32>, with_decl: bool) -> impl Iterator<Item = &Symbol<'db>> {
        self.resolved
            .iter()
            .enumerate()
            .filter(move |(symbol_index, resolved_def_index)| {
                if let Some(def_index) = def_index {
                    let is_def = with_decl && *symbol_index == def_index as usize;
                    let is_ref =
                        resolved_def_index.is_some_and(|resolved_def_index| resolved_def_index.get() == def_index);
                    is_def || is_ref
                } else {
                    false
                }
            })
            .filter_map(|(symbol_index, _)| self.symbols.values.get(symbol_index))
    }

    pub fn find_module(&self, module_id: u32) -> Option<&Symbol<'db>> {
        self.symbols
            .iter()
            .find(|symbol| symbol.kind == SymbolKind::Module && symbol.idx.num == Some(module_id))
    }

    pub fn get_type_node_of(&'db self, symbol: &'db Symbol) -> AmberNode<'db> {
        std::debug_assert_matches!(
            symbol.kind,
            SymbolKind::Func
                | SymbolKind::GlobalDef
                | SymbolKind::MemoryDef
                | SymbolKind::TableDef
                | SymbolKind::TagDef
        );
        self.type_nodes
            .get(&symbol.key)
            .map(|(green, range)| AmberNode::new(green, range.start()))
            .unwrap_or(symbol.amber())
    }

    pub fn iter_may_resolved(&self) -> slice::Iter<'_, Option<NonZeroU32>> {
        self.resolved.iter()
    }
    pub fn iter_resolved(&self) -> impl Iterator<Item = (u32, u32)> {
        self.resolved
            .iter()
            .enumerate()
            .filter_map(|(i, index)| index.map(|index| (i as u32, index.get())))
    }
}
#[salsa::tracked]
impl<'db> SymbolTable<'db> {
    #[salsa::tracked]
    pub(crate) fn of(db: &'db dyn salsa::Database, document: Document) -> Self {
        create_symbol_table(db, document)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, salsa::SalsaValue)]
/// Wrapper type for allowing `SyntaxNodePtr` to be stored in Salsa database.
pub struct SymbolKey(SyntaxNodePtr);
impl From<&SyntaxNode<'_>> for SymbolKey {
    #[inline]
    fn from(node: &SyntaxNode) -> Self {
        SymbolKey(SyntaxNodePtr::new(node))
    }
}
impl From<SyntaxNodePtr> for SymbolKey {
    #[inline]
    fn from(ptr: SyntaxNodePtr) -> Self {
        SymbolKey(ptr)
    }
}
impl From<AmberNode<'_>> for SymbolKey {
    #[inline]
    fn from(node: AmberNode<'_>) -> Self {
        SymbolKey(node.to_ptr())
    }
}
impl Deref for SymbolKey {
    type Target = SyntaxNodePtr;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug, salsa::SalsaValue)]
pub struct Symbol<'db> {
    pub key: SymbolKey,
    pub green: GreenNode,
    pub region: SymbolKey,
    pub kind: SymbolKind,
    pub idx: Idx<'db>,
}
impl Symbol<'_> {
    pub fn amber(&self) -> AmberNode<'_> {
        AmberNode::new(&self.green, self.key.0.text_range().start())
    }
}
impl PartialEq for Symbol<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.green == other.green
    }
}
impl Eq for Symbol<'_> {}
impl Hash for Symbol<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state);
        self.green.hash(state);
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
    ElemDef,
    ElemRef,
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
            SymbolKind::ElemDef | SymbolKind::ElemRef => write!(f, "elem segment"),
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
    Elem,
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
            SymbolKind::ElemDef | SymbolKind::ElemRef => IdxKind::Elem,
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
            IdxKind::Elem => write!(f, "elem segment"),
        }
    }
}

#[derive(Clone, salsa::SalsaValue)]
pub struct Symbols<'db> {
    values: Vec<Symbol<'db>>,
    indices: HashTable<u32>,
    build_hasher: FxBuildHasher,
}
impl<'db> Symbols<'db> {
    pub fn get<Q>(&self, key: Q) -> Option<&Symbol<'db>>
    where
        Q: Borrow<SymbolKey>,
    {
        self.get_index_of(key).and_then(|i| self.values.get(i as usize))
    }
    pub fn get_index(&self, index: u32) -> Option<&Symbol<'db>> {
        self.values.get(index as usize)
    }
    fn get_index_of<Q>(&self, key: Q) -> Option<u32>
    where
        Q: Borrow<SymbolKey>,
    {
        let key = *key.borrow();
        self.indices
            .find(self.build_hasher.hash_one(key), |i| {
                self.values.get(*i as usize).is_some_and(|symbol| symbol.key == key)
            })
            .copied()
    }
    pub fn iter(&self) -> slice::Iter<'_, Symbol<'db>> {
        self.values.iter()
    }
    fn insert(&mut self, symbol: Symbol<'db>) {
        let i = self.values.len() as u32;
        let key = symbol.key;
        self.values.push(symbol);
        self.indices.insert_unique(self.build_hasher.hash_one(key), i, |i| {
            self.build_hasher.hash_one(self.values[*i as usize].key)
        });
    }
}
impl PartialEq for Symbols<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.values.eq(&other.values)
    }
}
impl Eq for Symbols<'_> {}
impl<'db> Extend<Symbol<'db>> for Symbols<'db> {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = Symbol<'db>>,
    {
        let iter = iter.into_iter();
        let (len, _) = iter.size_hint();
        self.values.reserve(len);
        iter.for_each(|symbol| self.insert(symbol));
    }
}
impl fmt::Debug for Symbols<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Symbols")
            .field("values", &self.values)
            .field("indices", &self.indices)
            .finish()
    }
}

/// Slice that records about resolving "use" to "definition".
///
/// Items are in the same order of symbols, so an index in this slice is equivalent to the index in symbols.
///
/// Item type of this slice is `Option`, and there're three kinds of semantics.
/// - `Some(def_index)`: a use can be resolved to a definition;
/// - `None` for a ref-kind symbol: a use can't be resolved to a definition, a.k.a. undefined;
/// - `None` for a def-kind symbol: not applicable.
///
/// Invariant: index of a definition can never be zero, since `symbols[0]` always corresponds to the first module.
/// Leveraging this, we can use `NonZeroU32` to get smaller type size.
type Resolved = Box<[Option<NonZeroU32>]>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModuleDefSymbols {
    pub funcs: Vec<SymbolKey>,
    pub types: Vec<SymbolKey>,
    pub globals: Vec<SymbolKey>,
    pub memories: SmallVec<[SymbolKey; 1]>,
    pub tables: SmallVec<[SymbolKey; 1]>,
    pub tags: Vec<SymbolKey>,
    pub datas: Vec<SymbolKey>,
    pub elems: Vec<SymbolKey>,
}
