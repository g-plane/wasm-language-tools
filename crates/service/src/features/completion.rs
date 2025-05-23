use crate::{
    binder::{SymbolKey, SymbolKind, SymbolTablesCtx},
    data_set, helpers,
    idx::Idx,
    syntax_tree::SyntaxTreeCtx,
    types_analyzer::{self, CompositeType, Fields, OperandType, TypesAnalyzerCtx, ValType},
    uri::{InternUri, UrisCtx},
    LanguageService,
};
use itertools::Itertools;
use line_index::LineIndex;
use lspt::{
    CompletionItem, CompletionItemKind, CompletionItemLabelDetails, CompletionParams,
    MarkupContent, MarkupKind, TextEdit, Union2,
};
use rowan::{
    ast::{support, AstNode},
    Direction,
};
use smallvec::SmallVec;
use wat_syntax::{
    ast::{PlainInstr, TableType},
    SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken,
};

impl LanguageService {
    /// Handler for `textDocument/completion` request.
    pub fn completion(&self, params: CompletionParams) -> Option<Vec<CompletionItem>> {
        let uri = self.uri(params.text_document.uri);
        let line_index = self.line_index(uri);
        let root = SyntaxNode::new_root(self.root(uri));
        let token = helpers::ast::find_token(
            &root,
            helpers::lsp_pos_to_rowan_pos(&line_index, params.position)?,
        )?;

        let cmp_ctx = get_cmp_ctx(&token)?;
        Some(get_cmp_list(self, cmp_ctx, &token, uri, &line_index, &root))
    }
}

fn get_cmp_ctx(token: &SyntaxToken) -> Option<SmallVec<[CmpCtx; 4]>> {
    let mut ctx = SmallVec::with_capacity(2);
    let parent = token.parent()?;
    match parent.kind() {
        SyntaxKind::MODULE_FIELD_FUNC => {
            let prev_node = token
                .siblings_with_tokens(Direction::Prev)
                .skip(1)
                .find(|element| matches!(element, SyntaxElement::Node(..)));
            let next_node = token
                .siblings_with_tokens(Direction::Next)
                .skip(1)
                .find(|element| matches!(element, SyntaxElement::Node(..)))
                .map(|element| element.kind());
            if !matches!(
                prev_node.as_ref().map(|element| element.kind()),
                Some(
                    SyntaxKind::PLAIN_INSTR
                        | SyntaxKind::BLOCK_BLOCK
                        | SyntaxKind::BLOCK_IF
                        | SyntaxKind::BLOCK_LOOP
                )
            ) && find_leading_l_paren(token).is_some()
            {
                ctx.extend([
                    CmpCtx::KeywordImExport,
                    CmpCtx::KeywordType,
                    CmpCtx::KeywordParam,
                    CmpCtx::KeywordResult,
                    CmpCtx::KeywordLocal,
                ]);
            } else if let Some(node) = prev_node.as_ref().and_then(|prev| match prev {
                SyntaxElement::Node(node) if node.kind() == SyntaxKind::PLAIN_INSTR => Some(node),
                _ => None,
            }) {
                if let Some(instr_name) = support::token(node, SyntaxKind::INSTR_NAME) {
                    add_cmp_ctx_for_immediates(
                        instr_name.text(),
                        node,
                        find_leading_l_paren(token).is_some(),
                        &mut ctx,
                    );
                }
            }
            if !token.text().starts_with('$')
                && matches!(
                    next_node,
                    Some(
                        SyntaxKind::PLAIN_INSTR
                            | SyntaxKind::BLOCK_BLOCK
                            | SyntaxKind::BLOCK_IF
                            | SyntaxKind::BLOCK_LOOP
                    ) | None
                )
            {
                ctx.push(CmpCtx::Instr);
            }
        }
        SyntaxKind::TYPE_DEF => {
            if find_leading_l_paren(token).is_some() {
                ctx.extend([CmpCtx::KeywordsCompType, CmpCtx::KeywordSub]);
            }
        }
        SyntaxKind::PLAIN_INSTR => {
            if token.kind() == SyntaxKind::INSTR_NAME {
                ctx.push(CmpCtx::Instr);
                if let (Some(grand), false) = (parent.parent(), token.text().contains('.')) {
                    match grand.kind() {
                        SyntaxKind::MODULE_FIELD_FUNC => {
                            // Given the code below:
                            // (func (export "foo") (par))
                            //                          ^ cursor
                            // User is probably going to type "param",
                            // but parser treat it as a plain instruction,
                            // so we catch this case here, though these keywords aren't instruction names.
                            let prev_node = parent
                                .siblings_with_tokens(Direction::Prev)
                                .skip(1)
                                .find(|element| matches!(element, SyntaxElement::Node(..)))
                                .map(|element| element.kind());
                            if !matches!(
                                prev_node,
                                Some(
                                    SyntaxKind::PLAIN_INSTR
                                        | SyntaxKind::BLOCK_BLOCK
                                        | SyntaxKind::BLOCK_IF
                                        | SyntaxKind::BLOCK_LOOP
                                )
                            ) && find_leading_l_paren(token).is_some()
                            {
                                ctx.extend([
                                    CmpCtx::KeywordImExport,
                                    CmpCtx::KeywordType,
                                    CmpCtx::KeywordParam,
                                    CmpCtx::KeywordResult,
                                    CmpCtx::KeywordLocal,
                                ]);
                            }
                        }
                        SyntaxKind::PLAIN_INSTR => {
                            if let (Some(instr_name), Some(..)) = (
                                support::token(&grand, SyntaxKind::INSTR_NAME),
                                find_leading_l_paren(token),
                            ) {
                                add_cmp_ctx_for_immediates(
                                    instr_name.text(),
                                    &grand,
                                    true,
                                    &mut ctx,
                                );
                            }
                        }
                        SyntaxKind::BLOCK_BLOCK | SyntaxKind::BLOCK_IF | SyntaxKind::BLOCK_LOOP => {
                            let prev_node = parent
                                .siblings_with_tokens(Direction::Prev)
                                .skip(1)
                                .find(|element| matches!(element, SyntaxElement::Node(..)))
                                .map(|element| element.kind());
                            if !matches!(
                                prev_node,
                                Some(
                                    SyntaxKind::PLAIN_INSTR
                                        | SyntaxKind::BLOCK_BLOCK
                                        | SyntaxKind::BLOCK_IF
                                        | SyntaxKind::BLOCK_LOOP
                                )
                            ) && find_leading_l_paren(token).is_some()
                            {
                                ctx.extend([
                                    CmpCtx::KeywordParam,
                                    CmpCtx::KeywordResult,
                                    CmpCtx::KeywordType,
                                ]);
                            }
                        }
                        SyntaxKind::OFFSET => {
                            if support::token(&grand, SyntaxKind::KEYWORD).is_none() {
                                ctx.push(CmpCtx::KeywordOffset);
                                match grand.parent().map(|node| node.kind()) {
                                    Some(SyntaxKind::MODULE_FIELD_DATA) => {
                                        ctx.push(CmpCtx::KeywordMemory);
                                    }
                                    Some(SyntaxKind::MODULE_FIELD_ELEM) => {
                                        if !grand
                                            .siblings(Direction::Prev)
                                            .any(|child| child.kind() == SyntaxKind::TABLE_USE)
                                        {
                                            ctx.push(CmpCtx::KeywordTable);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
            } else if find_leading_l_paren(token).is_some() {
                ctx.push(CmpCtx::Instr);
                if let Some(instr_name) = support::token(&parent, SyntaxKind::INSTR_NAME) {
                    add_cmp_ctx_for_immediates(instr_name.text(), &parent, true, &mut ctx);
                }
            } else {
                let instr_name = support::token(&parent, SyntaxKind::INSTR_NAME)?;
                add_cmp_ctx_for_immediates(instr_name.text(), &parent, false, &mut ctx);
            }
        }
        SyntaxKind::BLOCK_BLOCK | SyntaxKind::BLOCK_IF | SyntaxKind::BLOCK_LOOP => {
            if find_leading_l_paren(token).is_some() {
                if token
                    .siblings_with_tokens(Direction::Prev)
                    .skip(1)
                    .all(|element| !matches!(element, SyntaxElement::Node(..)))
                {
                    ctx.extend([
                        CmpCtx::KeywordParam,
                        CmpCtx::KeywordResult,
                        CmpCtx::KeywordType,
                    ]);
                }
                ctx.push(CmpCtx::Instr);
            }
        }
        SyntaxKind::IMMEDIATE => {
            let instr = parent
                .ancestors()
                .find(|node| node.kind() == SyntaxKind::PLAIN_INSTR)?;
            let instr_name = support::token(&instr, SyntaxKind::INSTR_NAME)?;
            add_cmp_ctx_for_immediates(instr_name.text(), &parent, false, &mut ctx);
        }
        SyntaxKind::PARAM | SyntaxKind::RESULT | SyntaxKind::LOCAL | SyntaxKind::GLOBAL_TYPE => {
            if find_leading_l_paren(token).is_some() {
                ctx.push(CmpCtx::KeywordRef);
            } else if !token.text().starts_with('$') {
                ctx.extend([CmpCtx::NumTypeVecType, CmpCtx::AbbrRefType]);
            }
        }
        SyntaxKind::TYPE_USE => ctx.push(CmpCtx::TypeDef(Some(PreferredType::Func))),
        SyntaxKind::FUNC_TYPE => {
            if find_leading_l_paren(token).is_some() {
                ctx.extend([CmpCtx::KeywordParam, CmpCtx::KeywordResult]);
            }
        }
        SyntaxKind::INDEX => {
            let grand = parent.parent()?;
            match grand.kind() {
                SyntaxKind::MODULE_FIELD_START | SyntaxKind::EXPORT_DESC_FUNC => {
                    ctx.push(CmpCtx::Func);
                }
                SyntaxKind::TYPE_USE => ctx.push(CmpCtx::TypeDef(Some(PreferredType::Func))),
                SyntaxKind::EXPORT_DESC_GLOBAL => ctx.push(CmpCtx::Global),
                SyntaxKind::EXPORT_DESC_MEMORY => ctx.push(CmpCtx::Memory),
                SyntaxKind::EXPORT_DESC_TABLE => ctx.push(CmpCtx::Table),
                _ => {}
            }
        }
        SyntaxKind::MODULE_FIELD_GLOBAL => {
            if parent
                .children()
                .any(|node| node.kind() == SyntaxKind::GLOBAL_TYPE)
            {
                ctx.push(CmpCtx::Instr);
            } else if find_leading_l_paren(token).is_some() {
                ctx.extend([
                    CmpCtx::KeywordMut,
                    CmpCtx::KeywordImExport,
                    CmpCtx::KeywordRef,
                ]);
            } else {
                ctx.extend([CmpCtx::NumTypeVecType, CmpCtx::AbbrRefType]);
            }
        }
        SyntaxKind::MODULE_FIELD_EXPORT | SyntaxKind::MODULE_FIELD_IMPORT => {
            if find_leading_l_paren(token).is_some() {
                ctx.push(CmpCtx::KeywordPortDesc);
            }
        }
        SyntaxKind::MODULE_FIELD_TABLE => {
            if support::child::<TableType>(&parent)
                .and_then(|table_type| table_type.ref_type())
                .is_some()
            {
                ctx.push(CmpCtx::Instr);
            } else if find_leading_l_paren(token).is_some() {
                ctx.extend([CmpCtx::KeywordImExport, CmpCtx::KeywordElem]);
            } else {
                ctx.push(CmpCtx::AbbrRefType);
            }
        }
        SyntaxKind::MODULE_FIELD_START | SyntaxKind::EXPORT_DESC_FUNC => ctx.push(CmpCtx::Func),
        SyntaxKind::EXPORT_DESC_GLOBAL => ctx.push(CmpCtx::Global),
        SyntaxKind::MODULE_FIELD_MEMORY => {
            if find_leading_l_paren(token).is_some() {
                ctx.extend([CmpCtx::KeywordImExport, CmpCtx::KeywordData]);
            }
        }
        SyntaxKind::MODULE_FIELD_DATA => {
            if find_leading_l_paren(token).is_some() {
                ctx.extend([CmpCtx::KeywordMemory, CmpCtx::KeywordOffset, CmpCtx::Instr]);
            }
        }
        SyntaxKind::MODULE_FIELD_ELEM => {
            if find_leading_l_paren(token).is_some() {
                if !parent
                    .children()
                    .any(|child| child.kind() == SyntaxKind::TABLE_USE)
                    && !token
                        .siblings_with_tokens(Direction::Prev)
                        .any(|element| element.kind() == SyntaxKind::OFFSET)
                {
                    ctx.push(CmpCtx::KeywordTable);
                }
                ctx.extend([CmpCtx::KeywordOffset, CmpCtx::KeywordItem, CmpCtx::Instr]);
            } else if parent
                .children()
                .find(|child| child.kind() == SyntaxKind::ELEM_LIST)
                .and_then(|elem_list| support::token(&elem_list, SyntaxKind::KEYWORD))
                .is_some_and(|keyword| keyword.text() == "func")
            {
                ctx.push(CmpCtx::Func);
            } else {
                ctx.extend([CmpCtx::AbbrRefType, CmpCtx::KeywordFunc]);
                if !parent.children_with_tokens().any(|element| {
                    if let SyntaxElement::Token(token) = element {
                        token.text() == "declare"
                    } else {
                        false
                    }
                }) {
                    ctx.push(CmpCtx::KeywordDeclare);
                }
            }
        }
        SyntaxKind::EXPORT_DESC_MEMORY | SyntaxKind::MEM_USE => ctx.push(CmpCtx::Memory),
        SyntaxKind::EXPORT_DESC_TABLE => ctx.push(CmpCtx::Table),
        SyntaxKind::TABLE_TYPE | SyntaxKind::IMPORT_DESC_TABLE_TYPE => {
            if find_leading_l_paren(token).is_some() {
                ctx.push(CmpCtx::KeywordRef);
            } else {
                ctx.push(CmpCtx::AbbrRefType);
            }
        }
        SyntaxKind::IMPORT_DESC_TYPE_USE => {
            if find_leading_l_paren(token).is_some() {
                ctx.extend([
                    CmpCtx::KeywordParam,
                    CmpCtx::KeywordResult,
                    CmpCtx::KeywordType,
                ]);
            }
        }
        SyntaxKind::IMPORT_DESC_GLOBAL_TYPE => {
            if find_leading_l_paren(token).is_some() {
                ctx.extend([CmpCtx::KeywordMut, CmpCtx::KeywordRef]);
            } else {
                ctx.extend([CmpCtx::NumTypeVecType, CmpCtx::AbbrRefType]);
            }
        }
        SyntaxKind::ELEM => {
            if find_leading_l_paren(token).is_some() {
                ctx.extend([CmpCtx::Instr, CmpCtx::KeywordItem]);
            } else {
                ctx.push(CmpCtx::Func);
            }
        }
        SyntaxKind::ELEM_EXPR | SyntaxKind::OFFSET => ctx.push(CmpCtx::Instr),
        SyntaxKind::ELEM_LIST => {
            if find_leading_l_paren(token).is_some() {
                ctx.extend([CmpCtx::KeywordItem, CmpCtx::Instr]);
            } else if token
                .siblings_with_tokens(Direction::Prev)
                .any(|element| match element {
                    SyntaxElement::Token(token) => {
                        token.kind() == SyntaxKind::KEYWORD && token.text() == "func"
                    }
                    _ => false,
                })
            {
                ctx.push(CmpCtx::Func);
            }
        }
        SyntaxKind::TABLE_USE => ctx.push(CmpCtx::Table),
        SyntaxKind::MODULE => {
            if find_leading_l_paren(token).is_some() {
                ctx.push(CmpCtx::KeywordModuleField);
            }
        }
        SyntaxKind::ROOT => {
            if find_leading_l_paren(token).is_some() {
                ctx.push(CmpCtx::KeywordModule);
            }
        }
        SyntaxKind::SUB_TYPE => {
            if find_leading_l_paren(token).is_some() {
                ctx.push(CmpCtx::KeywordsCompType);
            } else {
                ctx.extend([CmpCtx::TypeDef(None), CmpCtx::KeywordFinal]);
            }
        }
        SyntaxKind::STRUCT_TYPE => {
            if find_leading_l_paren(token).is_some() {
                ctx.push(CmpCtx::KeywordField);
            }
        }
        SyntaxKind::FIELD | SyntaxKind::ARRAY_TYPE => {
            if find_leading_l_paren(token).is_some() {
                ctx.extend([CmpCtx::KeywordMut, CmpCtx::KeywordRef]);
            } else {
                ctx.extend([
                    CmpCtx::NumTypeVecType,
                    CmpCtx::AbbrRefType,
                    CmpCtx::PackedType,
                ]);
            }
        }
        SyntaxKind::FIELD_TYPE => {
            if find_leading_l_paren(token).is_some() {
                ctx.push(CmpCtx::KeywordRef);
            } else {
                ctx.extend([
                    CmpCtx::NumTypeVecType,
                    CmpCtx::AbbrRefType,
                    CmpCtx::PackedType,
                ]);
            }
        }
        SyntaxKind::REF_TYPE => {
            ctx.extend([
                CmpCtx::TypeDef(None),
                CmpCtx::KeywordNull,
                CmpCtx::AbsHeapType,
            ]);
        }
        SyntaxKind::REC_TYPE => {
            if find_leading_l_paren(token).is_some() {
                ctx.push(CmpCtx::KeywordType);
            }
        }
        _ => {}
    }
    if ctx.is_empty() {
        None
    } else {
        Some(ctx)
    }
}
fn add_cmp_ctx_for_immediates(
    instr_name: &str,
    node: &SyntaxNode,
    has_leading_l_paren: bool,
    ctx: &mut SmallVec<[CmpCtx; 4]>,
) {
    if has_leading_l_paren {
        match instr_name {
            "select" => ctx.push(CmpCtx::KeywordResult),
            "call_indirect" | "return_call_indirect" => {
                ctx.extend([
                    CmpCtx::KeywordType,
                    CmpCtx::KeywordParam,
                    CmpCtx::KeywordResult,
                ]);
            }
            "ref.test" | "ref.cast" => ctx.push(CmpCtx::KeywordRef),
            _ => {}
        }
    } else {
        let is_current_first_immediate = node.kind() == SyntaxKind::IMMEDIATE
            && node
                .prev_sibling()
                .is_none_or(|prev| prev.kind() != SyntaxKind::IMMEDIATE)
            || PlainInstr::cast(node.clone()).is_some_and(|instr| instr.immediates().count() == 0);
        match instr_name.split_once('.') {
            Some(("local", _)) => ctx.push(CmpCtx::Local),
            Some(("global", _)) => ctx.push(CmpCtx::Global),
            Some(("ref", "func")) => ctx.push(CmpCtx::Func),
            Some(("table", snd)) => {
                if snd == "init"
                    && node.kind() == SyntaxKind::IMMEDIATE
                    && node
                        .prev_sibling()
                        .is_some_and(|prev| prev.kind() == SyntaxKind::IMMEDIATE)
                {
                    // elem id
                } else {
                    ctx.push(CmpCtx::Table);
                }
            }
            Some(("memory", "size" | "grow" | "fill" | "copy")) => ctx.push(CmpCtx::Memory),
            Some(("memory", "init")) => {
                if is_current_first_immediate {
                    ctx.push(CmpCtx::Memory);
                }
            }
            Some((_, snd)) if snd.starts_with("load") || snd.starts_with("store") => {
                if is_current_first_immediate {
                    ctx.push(CmpCtx::Memory);
                }
                ctx.push(CmpCtx::MemArg);
            }
            Some(("struct", snd)) => {
                if is_current_first_immediate {
                    ctx.push(CmpCtx::TypeDef(Some(PreferredType::Struct)));
                }
                if matches!(snd, "get" | "get_s" | "get_u" | "set") {
                    let first_immediate = if node.kind() == SyntaxKind::IMMEDIATE {
                        node.prev_sibling()
                    } else {
                        node.first_child()
                    };
                    if let Some(immediate) = first_immediate {
                        ctx.push(CmpCtx::Field(SymbolKey::new(&immediate)));
                    }
                }
            }
            Some(("array", snd)) if snd != "len" => {
                if is_current_first_immediate {
                    ctx.push(CmpCtx::TypeDef(Some(PreferredType::Array)));
                }
            }
            Some(("ref", "null")) => {
                ctx.extend([CmpCtx::TypeDef(None), CmpCtx::AbsHeapType]);
            }
            None => match instr_name {
                "call" | "return_call" => ctx.push(CmpCtx::Func),
                "br" | "br_if" | "br_table" | "br_on_null" | "br_on_non_null" => {
                    ctx.push(CmpCtx::Block);
                }
                "call_indirect" | "return_call_indirect" => {
                    if is_current_first_immediate {
                        ctx.push(CmpCtx::Table);
                    } else {
                        ctx.push(CmpCtx::TypeDef(Some(PreferredType::Func)));
                    }
                }
                "call_ref" | "return_call_ref" => {
                    ctx.push(CmpCtx::TypeDef(Some(PreferredType::Func)));
                }
                _ => {}
            },
            _ => {}
        }
    }
}

enum CmpCtx {
    Instr,
    NumTypeVecType,
    AbbrRefType,
    Local,
    Func,
    TypeDef(Option<PreferredType>),
    Global,
    MemArg,
    Memory,
    Table,
    Block,
    Field(SymbolKey),
    AbsHeapType,
    PackedType,
    KeywordModule,
    KeywordModuleField,
    KeywordImExport,
    KeywordType,
    KeywordParam,
    KeywordResult,
    KeywordLocal,
    KeywordMut,
    KeywordPortDesc,
    KeywordData,
    KeywordFunc,
    KeywordElem,
    KeywordItem,
    KeywordMemory,
    KeywordOffset,
    KeywordDeclare,
    KeywordTable,
    KeywordSub,
    KeywordFinal,
    KeywordsCompType,
    KeywordField,
    KeywordRef,
    KeywordNull,
}
enum PreferredType {
    Func,
    Array,
    Struct,
}

fn get_cmp_list(
    service: &LanguageService,
    ctx: SmallVec<[CmpCtx; 4]>,
    token: &SyntaxToken,
    uri: InternUri,
    line_index: &LineIndex,
    root: &SyntaxNode,
) -> Vec<CompletionItem> {
    let symbol_table = service.symbol_table(uri);
    ctx.into_iter()
        .fold(Vec::with_capacity(2), |mut items, ctx| {
            match ctx {
                CmpCtx::Instr => {
                    if let Some((left, _)) = token.text().split_once('.') {
                        items.extend(
                            data_set::INSTR_NAMES
                                .iter()
                                .filter_map(|name| {
                                    name.strip_prefix(left).and_then(|s| s.strip_prefix('.'))
                                })
                                .map(|name| CompletionItem {
                                    label: name.to_string(),
                                    kind: Some(CompletionItemKind::Operator),
                                    ..Default::default()
                                }),
                        );
                    } else {
                        items.extend(data_set::INSTR_NAMES.iter().map(|name| CompletionItem {
                            label: name.to_string(),
                            kind: Some(CompletionItemKind::Operator),
                            ..Default::default()
                        }));
                    }
                }
                CmpCtx::NumTypeVecType => {
                    items.extend(["i32", "i64", "f32", "f64", "v128"].into_iter().map(|ty| {
                        CompletionItem {
                            label: ty.to_string(),
                            kind: Some(CompletionItemKind::Class),
                            documentation: data_set::get_value_type_description(ty).map(|desc| {
                                Union2::B(MarkupContent {
                                    kind: MarkupKind::Markdown,
                                    value: desc.into(),
                                })
                            }),
                            ..Default::default()
                        }
                    }));
                }
                CmpCtx::AbbrRefType => {
                    items.extend(
                        [
                            "anyref",
                            "eqref",
                            "i31ref",
                            "structref",
                            "arrayref",
                            "nullref",
                            "funcref",
                            "nullfuncref",
                            "externref",
                            "nullexternref",
                        ]
                        .into_iter()
                        .map(|ty| CompletionItem {
                            label: ty.into(),
                            kind: Some(CompletionItemKind::Class),
                            documentation: data_set::get_value_type_description(ty).map(|desc| {
                                Union2::B(MarkupContent {
                                    kind: MarkupKind::Markdown,
                                    value: desc.into(),
                                })
                            }),
                            ..Default::default()
                        }),
                    );
                }
                CmpCtx::Local => {
                    let Some(func) = token
                        .parent_ancestors()
                        .find(|node| node.kind() == SyntaxKind::MODULE_FIELD_FUNC)
                    else {
                        return items;
                    };
                    let func = SymbolKey::new(&func);
                    let preferred_type = guess_preferred_type(service, uri, token);
                    items.extend(
                        symbol_table
                            .symbols
                            .iter()
                            .filter(|symbol| {
                                matches!(symbol.kind, SymbolKind::Param | SymbolKind::Local)
                                    && symbol.region == func
                            })
                            .map(|symbol| {
                                let label = symbol.idx.render(service).to_string();
                                let ty = service.extract_type(symbol.green.clone());
                                CompletionItem {
                                    label: label.clone(),
                                    kind: Some(CompletionItemKind::Variable),
                                    text_edit: if token.kind().is_trivia() {
                                        None
                                    } else {
                                        Some(Union2::A(TextEdit {
                                            range: helpers::rowan_range_to_lsp_range(
                                                line_index,
                                                token.text_range(),
                                            ),
                                            new_text: label,
                                        }))
                                    },
                                    label_details: ty.map(|ty| CompletionItemLabelDetails {
                                        description: Some(ty.render(service).to_string()),
                                        ..Default::default()
                                    }),
                                    sort_text: preferred_type.zip(ty).map(|(expected, it)| {
                                        if expected == it {
                                            "0".into()
                                        } else {
                                            "1".into()
                                        }
                                    }),
                                    ..Default::default()
                                }
                            }),
                    );
                }
                CmpCtx::Func => {
                    let Some(module) = token
                        .parent_ancestors()
                        .find(|node| node.kind() == SyntaxKind::MODULE)
                    else {
                        return items;
                    };
                    items.extend(symbol_table.get_declared(module, SymbolKind::Func).map(
                        |symbol| {
                            let label = symbol.idx.render(service).to_string();
                            CompletionItem {
                                label: label.clone(),
                                kind: Some(CompletionItemKind::Function),
                                text_edit: if token.kind().is_trivia() {
                                    None
                                } else {
                                    Some(Union2::A(TextEdit {
                                        range: helpers::rowan_range_to_lsp_range(
                                            line_index,
                                            token.text_range(),
                                        ),
                                        new_text: label,
                                    }))
                                },
                                detail: Some(service.render_func_header(
                                    symbol.idx.name,
                                    service.get_func_sig(uri, symbol.key, symbol.green.clone()),
                                )),
                                label_details: Some(CompletionItemLabelDetails {
                                    description: Some(
                                        service.render_compact_sig(
                                            service
                                                .get_func_sig(uri, symbol.key, symbol.green.clone())
                                                .unwrap_or_default(),
                                        ),
                                    ),
                                    ..Default::default()
                                }),
                                documentation: Some(Union2::B(MarkupContent {
                                    kind: MarkupKind::Markdown,
                                    value: helpers::ast::get_doc_comment(&symbol.key.to_node(root)),
                                })),
                                ..Default::default()
                            }
                        },
                    ));
                }
                CmpCtx::TypeDef(preferred_type) => {
                    let Some(module) = token
                        .parent_ancestors()
                        .find(|node| node.kind() == SyntaxKind::MODULE)
                    else {
                        return items;
                    };
                    let def_types = service.def_types(uri);
                    items.extend(symbol_table.get_declared(module, SymbolKind::Type).map(
                        |symbol| {
                            let label = symbol.idx.render(service).to_string();
                            CompletionItem {
                                label: label.clone(),
                                kind: Some(CompletionItemKind::Interface),
                                text_edit: if token.kind().is_trivia() {
                                    None
                                } else {
                                    Some(Union2::A(TextEdit {
                                        range: helpers::rowan_range_to_lsp_range(
                                            line_index,
                                            token.text_range(),
                                        ),
                                        new_text: label,
                                    }))
                                },
                                sort_text: preferred_type.as_ref().and_then(|preferred_type| {
                                    def_types
                                        .iter()
                                        .find(|def_type| def_type.key == symbol.key)
                                        .map(|def_type| {
                                            if let (CompositeType::Func(..), PreferredType::Func)
                                            | (
                                                CompositeType::Array(..),
                                                PreferredType::Array,
                                            )
                                            | (
                                                CompositeType::Struct(..),
                                                PreferredType::Struct,
                                            ) = (&def_type.comp, preferred_type)
                                            {
                                                "0".into()
                                            } else {
                                                "1".into()
                                            }
                                        })
                                }),
                                ..Default::default()
                            }
                        },
                    ));
                }
                CmpCtx::Global => {
                    let Some(module) = token
                        .parent_ancestors()
                        .find(|node| node.kind() == SyntaxKind::MODULE)
                    else {
                        return items;
                    };
                    let preferred_type = guess_preferred_type(service, uri, token);
                    items.extend(
                        symbol_table
                            .get_declared(module, SymbolKind::GlobalDef)
                            .map(|symbol| {
                                let label = symbol.idx.render(service).to_string();
                                let ty = service.extract_global_type(symbol.green.clone());
                                CompletionItem {
                                    label: label.clone(),
                                    kind: Some(CompletionItemKind::Variable),
                                    text_edit: if token.kind().is_trivia() {
                                        None
                                    } else {
                                        Some(Union2::A(TextEdit {
                                            range: helpers::rowan_range_to_lsp_range(
                                                line_index,
                                                token.text_range(),
                                            ),
                                            new_text: label,
                                        }))
                                    },
                                    label_details: ty.map(|ty| CompletionItemLabelDetails {
                                        description: Some(ty.render(service).to_string()),
                                        ..Default::default()
                                    }),
                                    sort_text: preferred_type.zip(ty).map(|(expected, it)| {
                                        if expected == it {
                                            "0".into()
                                        } else {
                                            "1".into()
                                        }
                                    }),
                                    ..Default::default()
                                }
                            }),
                    );
                }
                CmpCtx::MemArg => {
                    items.extend(["offset=", "align="].iter().map(|label| CompletionItem {
                        label: label.to_string(),
                        kind: Some(CompletionItemKind::Snippet),
                        ..Default::default()
                    }));
                }
                CmpCtx::Memory => {
                    let Some(module) = token
                        .parent_ancestors()
                        .find(|node| node.kind() == SyntaxKind::MODULE)
                    else {
                        return items;
                    };
                    items.extend(
                        symbol_table
                            .get_declared(module, SymbolKind::MemoryDef)
                            .map(|symbol| {
                                let label = symbol.idx.render(service).to_string();
                                CompletionItem {
                                    label: label.clone(),
                                    kind: Some(CompletionItemKind::Variable),
                                    text_edit: if token.kind().is_trivia() {
                                        None
                                    } else {
                                        Some(Union2::A(TextEdit {
                                            range: helpers::rowan_range_to_lsp_range(
                                                line_index,
                                                token.text_range(),
                                            ),
                                            new_text: label,
                                        }))
                                    },
                                    ..Default::default()
                                }
                            }),
                    );
                }
                CmpCtx::Table => {
                    let Some(module) = token
                        .parent_ancestors()
                        .find(|node| node.kind() == SyntaxKind::MODULE)
                        .map(|module| SymbolKey::new(&module))
                    else {
                        return items;
                    };
                    items.extend(
                        symbol_table
                            .symbols
                            .iter()
                            .filter(|symbol| {
                                symbol.kind == SymbolKind::TableDef && symbol.region == module
                            })
                            .map(|symbol| {
                                let label = symbol.idx.render(service).to_string();
                                CompletionItem {
                                    label: label.clone(),
                                    kind: Some(CompletionItemKind::Variable),
                                    text_edit: if token.kind().is_trivia() {
                                        None
                                    } else {
                                        Some(Union2::A(TextEdit {
                                            range: helpers::rowan_range_to_lsp_range(
                                                line_index,
                                                token.text_range(),
                                            ),
                                            new_text: label,
                                        }))
                                    },
                                    ..Default::default()
                                }
                            }),
                    );
                }
                CmpCtx::Block => {
                    items.extend(
                        symbol_table
                            .symbols
                            .iter()
                            .filter(|symbol| {
                                symbol.kind == SymbolKind::BlockDef
                                    && symbol.key.text_range().contains_range(token.text_range())
                            })
                            .rev()
                            .enumerate()
                            .map(|(num, symbol)| {
                                let idx = Idx {
                                    num: Some(num as u32),
                                    name: symbol.idx.name,
                                };
                                let label = idx.render(service).to_string();
                                let block_node = symbol.key.to_node(root);
                                let sig = types_analyzer::get_block_sig(service, uri, &block_node);
                                CompletionItem {
                                    label: label.clone(),
                                    kind: Some(CompletionItemKind::Variable),
                                    text_edit: if token.kind().is_trivia() {
                                        None
                                    } else {
                                        Some(Union2::A(TextEdit {
                                            range: helpers::rowan_range_to_lsp_range(
                                                line_index,
                                                token.text_range(),
                                            ),
                                            new_text: label,
                                        }))
                                    },
                                    label_details: Some(CompletionItemLabelDetails {
                                        description: Some(format!(
                                            "[{}]",
                                            sig.as_ref()
                                                .map(|sig| sig
                                                    .results
                                                    .iter()
                                                    .map(|result| result.render(service))
                                                    .join(", "))
                                                .unwrap_or_default()
                                        )),
                                        ..Default::default()
                                    }),
                                    detail: Some(service.render_block_header(
                                        symbol.key.kind(),
                                        idx.name,
                                        sig,
                                    )),
                                    ..Default::default()
                                }
                            }),
                    );
                }
                CmpCtx::Field(struct_ref_key) => {
                    let def_types = service.def_types(uri);
                    if let Some(CompositeType::Struct(Fields(fields))) = symbol_table
                        .find_def(struct_ref_key)
                        .and_then(|symbol| {
                            def_types.iter().find(|def_type| def_type.key == symbol.key)
                        })
                        .map(|def_type| &def_type.comp)
                    {
                        items.extend(fields.iter().map(|(ty, idx)| {
                            let label = idx.render(service).to_string();
                            CompletionItem {
                                label: label.clone(),
                                kind: Some(CompletionItemKind::Field),
                                text_edit: if token.kind().is_trivia() {
                                    None
                                } else {
                                    Some(Union2::A(TextEdit {
                                        range: helpers::rowan_range_to_lsp_range(
                                            line_index,
                                            token.text_range(),
                                        ),
                                        new_text: label,
                                    }))
                                },
                                label_details: Some(CompletionItemLabelDetails {
                                    description: Some(ty.render(service).to_string()),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }
                        }));
                    }
                }
                CmpCtx::AbsHeapType => {
                    items.extend(
                        [
                            "any", "eq", "i31", "struct", "array", "none", "func", "nofunc",
                            "extern", "noextern",
                        ]
                        .into_iter()
                        .map(|ty| CompletionItem {
                            label: ty.into(),
                            kind: Some(CompletionItemKind::Class),
                            ..Default::default()
                        }),
                    );
                }
                CmpCtx::PackedType => {
                    items.extend(["i8", "i16"].into_iter().map(|ty| CompletionItem {
                        label: ty.to_string(),
                        kind: Some(CompletionItemKind::Class),
                        ..Default::default()
                    }));
                }
                CmpCtx::KeywordModule => items.push(CompletionItem {
                    label: "module".to_string(),
                    kind: Some(CompletionItemKind::Keyword),
                    ..Default::default()
                }),
                CmpCtx::KeywordModuleField => {
                    items.extend(data_set::MODULE_FIELDS.iter().map(|ty| CompletionItem {
                        label: ty.to_string(),
                        kind: Some(CompletionItemKind::Keyword),
                        ..Default::default()
                    }));
                }
                CmpCtx::KeywordImExport => {
                    items.extend(["import", "export"].iter().map(|keyword| CompletionItem {
                        label: keyword.to_string(),
                        kind: Some(CompletionItemKind::Keyword),
                        ..Default::default()
                    }));
                }
                CmpCtx::KeywordType => items.push(CompletionItem {
                    label: "type".to_string(),
                    kind: Some(CompletionItemKind::Keyword),
                    ..Default::default()
                }),
                CmpCtx::KeywordParam => items.push(CompletionItem {
                    label: "param".to_string(),
                    kind: Some(CompletionItemKind::Keyword),
                    ..Default::default()
                }),
                CmpCtx::KeywordResult => items.push(CompletionItem {
                    label: "result".to_string(),
                    kind: Some(CompletionItemKind::Keyword),
                    ..Default::default()
                }),
                CmpCtx::KeywordLocal => items.push(CompletionItem {
                    label: "local".to_string(),
                    kind: Some(CompletionItemKind::Keyword),
                    ..Default::default()
                }),
                CmpCtx::KeywordMut => items.push(CompletionItem {
                    label: "mut".to_string(),
                    kind: Some(CompletionItemKind::Keyword),
                    ..Default::default()
                }),
                CmpCtx::KeywordPortDesc => {
                    items.extend(data_set::PORT_DESC.iter().map(|desc| CompletionItem {
                        label: desc.to_string(),
                        kind: Some(CompletionItemKind::Keyword),
                        ..Default::default()
                    }));
                }
                CmpCtx::KeywordData => items.push(CompletionItem {
                    label: "data".to_string(),
                    kind: Some(CompletionItemKind::Keyword),
                    ..Default::default()
                }),
                CmpCtx::KeywordFunc => items.push(CompletionItem {
                    label: "func".to_string(),
                    kind: Some(CompletionItemKind::Keyword),
                    ..Default::default()
                }),
                CmpCtx::KeywordElem => items.push(CompletionItem {
                    label: "elem".to_string(),
                    kind: Some(CompletionItemKind::Keyword),
                    ..Default::default()
                }),
                CmpCtx::KeywordItem => items.push(CompletionItem {
                    label: "item".to_string(),
                    kind: Some(CompletionItemKind::Keyword),
                    ..Default::default()
                }),
                CmpCtx::KeywordMemory => items.push(CompletionItem {
                    label: "memory".to_string(),
                    kind: Some(CompletionItemKind::Keyword),
                    ..Default::default()
                }),
                CmpCtx::KeywordOffset => items.push(CompletionItem {
                    label: "offset".to_string(),
                    kind: Some(CompletionItemKind::Keyword),
                    ..Default::default()
                }),
                CmpCtx::KeywordDeclare => items.push(CompletionItem {
                    label: "declare".to_string(),
                    kind: Some(CompletionItemKind::Keyword),
                    ..Default::default()
                }),
                CmpCtx::KeywordTable => items.push(CompletionItem {
                    label: "table".to_string(),
                    kind: Some(CompletionItemKind::Keyword),
                    ..Default::default()
                }),
                CmpCtx::KeywordSub => items.push(CompletionItem {
                    label: "sub".to_string(),
                    kind: Some(CompletionItemKind::Keyword),
                    ..Default::default()
                }),
                CmpCtx::KeywordFinal => items.push(CompletionItem {
                    label: "final".to_string(),
                    kind: Some(CompletionItemKind::Keyword),
                    ..Default::default()
                }),
                CmpCtx::KeywordsCompType => {
                    items.extend(["func", "struct", "array"].into_iter().map(|keyword| {
                        CompletionItem {
                            label: keyword.to_string(),
                            kind: Some(CompletionItemKind::Keyword),
                            ..Default::default()
                        }
                    }));
                }
                CmpCtx::KeywordField => items.push(CompletionItem {
                    label: "field".to_string(),
                    kind: Some(CompletionItemKind::Keyword),
                    ..Default::default()
                }),
                CmpCtx::KeywordRef => items.push(CompletionItem {
                    label: "ref".to_string(),
                    kind: Some(CompletionItemKind::Keyword),
                    ..Default::default()
                }),
                CmpCtx::KeywordNull => items.push(CompletionItem {
                    label: "null".to_string(),
                    kind: Some(CompletionItemKind::Keyword),
                    ..Default::default()
                }),
            }
            items
        })
}

fn find_leading_l_paren(token: &SyntaxToken) -> Option<SyntaxToken> {
    if is_l_paren(token) {
        Some(token.clone())
    } else {
        token
            .siblings_with_tokens(Direction::Prev)
            .skip(1)
            .skip_while(|element| element.kind().is_trivia())
            .find_map(SyntaxElement::into_token)
            .filter(is_l_paren)
    }
}
fn is_l_paren(token: &SyntaxToken) -> bool {
    let kind = token.kind();
    kind == SyntaxKind::L_PAREN || kind == SyntaxKind::ERROR && token.text() == "("
}

fn guess_preferred_type(
    service: &LanguageService,
    uri: InternUri,
    token: &SyntaxToken,
) -> Option<ValType> {
    token
        .parent_ancestors()
        .find(|node| node.kind() == SyntaxKind::PLAIN_INSTR)
        .and_then(|parent_instr| {
            let grand_instr = parent_instr
                .ancestors()
                .skip(1)
                .find(|node| node.kind() == SyntaxKind::PLAIN_INSTR)?;
            let index = grand_instr
                .children()
                .filter(|child| child.kind() == SyntaxKind::PLAIN_INSTR)
                .position(|instr| instr == parent_instr)?;
            let types = types_analyzer::resolve_param_types(service, uri, &grand_instr)?;
            if let Some(OperandType::Val(val_type)) = types.get(index) {
                Some(*val_type)
            } else {
                None
            }
        })
}
