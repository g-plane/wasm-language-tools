use crate::{
    binder::{Symbol, SymbolKey, SymbolTable, SymbolTablesCtx},
    data_set::INSTR_SIG,
    helpers,
    idx::{Idx, InternIdent},
    syntax_tree::SyntaxTreeCtx,
    uri::InternUri,
    LanguageService,
};
use itertools::Itertools;
use rowan::{
    ast::{support, AstNode},
    GreenNode, GreenNodeData, Language, NodeOrToken,
};
use std::{
    fmt::{self, Debug, Display},
    hash::Hash,
    ops::Deref,
};
use wat_syntax::{
    ast::{BlockType, Immediate, Param, Result, TypeUse, ValType as AstValType},
    SyntaxKind, SyntaxNode, SyntaxNodePtr, WatLanguage,
};

#[salsa::query_group(TypesAnalyzer)]
pub(crate) trait TypesAnalyzerCtx: SyntaxTreeCtx + SymbolTablesCtx {
    #[salsa::memoized]
    fn extract_type(&self, node: GreenNode) -> Option<ValType>;
    #[salsa::memoized]
    fn extract_global_type(&self, node: GreenNode) -> Option<ValType>;
    #[salsa::memoized]
    fn extract_sig(&self, node: GreenNode) -> Signature;

    #[salsa::memoized]
    fn get_func_sig(
        &self,
        uri: InternUri,
        ptr: SyntaxNodePtr,
        green: GreenNode,
    ) -> Option<Signature>;
    #[salsa::memoized]
    fn get_type_use_sig(
        &self,
        uri: InternUri,
        ptr: SyntaxNodePtr,
        type_use: GreenNode,
    ) -> Option<Signature>;
    #[salsa::memoized]
    fn render_sig(&self, signature: Signature) -> String;
    #[salsa::memoized]
    fn render_compact_sig(&self, signature: Signature) -> String;
    #[salsa::memoized]
    fn render_func_header(&self, name: Option<InternIdent>, signature: Option<Signature>)
        -> String;
    #[salsa::memoized]
    fn render_block_header(
        &self,
        kind: SyntaxKind,
        name: Option<InternIdent>,
        signature: Option<Signature>,
    ) -> String;
}
fn extract_type(db: &dyn TypesAnalyzerCtx, node: GreenNode) -> Option<ValType> {
    ValType::from_green(&node, db).or_else(|| {
        node.children().find_map(|child| match child {
            NodeOrToken::Node(node)
                if AstValType::can_cast(WatLanguage::kind_from_raw(node.kind())) =>
            {
                ValType::from_green(node, db)
            }
            _ => None,
        })
    })
}

fn extract_global_type(db: &dyn TypesAnalyzerCtx, node: GreenNode) -> Option<ValType> {
    node.children()
        .find_map(|child| match child {
            NodeOrToken::Node(node) if node.kind() == SyntaxKind::GLOBAL_TYPE.into() => Some(node),
            _ => None,
        })
        .and_then(|global_type| db.extract_type(global_type.to_owned()))
}

fn extract_sig(db: &dyn TypesAnalyzerCtx, node: GreenNode) -> Signature {
    let root = SyntaxNode::new_root(node);
    let params = support::children::<Param>(&root).fold(vec![], |mut acc, param| {
        if let Some((ident, ty)) = param.ident_token().zip(
            param
                .val_types()
                .next()
                .and_then(|ty| ValType::from_ast(&ty, db)),
        ) {
            acc.push((ty, Some(db.ident(ident.text().into()))));
        } else {
            acc.extend(
                param
                    .val_types()
                    .filter_map(|ty| ValType::from_ast(&ty, db))
                    .map(|val_type| (val_type, None)),
            );
        }
        acc
    });
    let results = support::children::<Result>(&root)
        .flat_map(|result| result.val_types())
        .filter_map(|ty| ValType::from_ast(&ty, db))
        .collect();
    Signature { params, results }
}

fn get_func_sig(
    db: &dyn TypesAnalyzerCtx,
    uri: InternUri,
    ptr: SyntaxNodePtr,
    green: GreenNode,
) -> Option<Signature> {
    green
        .children()
        .find_map(|child| match child {
            NodeOrToken::Node(node) if node.kind() == SyntaxKind::TYPE_USE.into() => Some(node),
            _ => None,
        })
        .and_then(|type_use| {
            if type_use.children().any(|child| {
                let kind = child.kind();
                kind == SyntaxKind::PARAM.into() || kind == SyntaxKind::RESULT.into()
            }) {
                Some(db.extract_sig(type_use.to_owned()))
            } else {
                let node = ptr.to_node(&SyntaxNode::new_root(db.root(uri)));
                let symbol_table = db.symbol_table(uri);
                support::child::<TypeUse>(&node)
                    .and_then(|type_use| type_use.index())
                    .and_then(|idx| symbol_table.find_def(SymbolKey::new(idx.syntax())))
                    .and_then(|symbol| helpers::ast::find_func_type_of_type_def(&symbol.green))
                    .map(|func_type| db.extract_sig(func_type))
            }
        })
}

fn get_type_use_sig(
    db: &dyn TypesAnalyzerCtx,
    uri: InternUri,
    ptr: SyntaxNodePtr,
    type_use: GreenNode,
) -> Option<Signature> {
    if type_use.children().any(|child| {
        let kind = child.kind();
        kind == SyntaxKind::PARAM.into() || kind == SyntaxKind::RESULT.into()
    }) {
        Some(db.extract_sig(type_use.to_owned()))
    } else {
        let symbol_table = db.symbol_table(uri);
        TypeUse::cast(ptr.to_node(&SyntaxNode::new_root(db.root(uri))))
            .and_then(|type_use| type_use.index())
            .and_then(|idx| symbol_table.find_def(SymbolKey::new(idx.syntax())))
            .and_then(|symbol| helpers::ast::find_func_type_of_type_def(&symbol.green))
            .map(|func_type| db.extract_sig(func_type))
    }
}

// The reason why we don't put this function to Salsa is because
// the block node comes with block body and can be huge.
// Once the body changed (even block type is unchanged), memoization will be skipped.
// Also, Salsa requires the ownership of GreenNode,
// which means we must clone the whole huge block green node.
pub fn get_block_sig(
    service: &LanguageService,
    uri: InternUri,
    node: &SyntaxNode,
) -> Option<Signature> {
    support::child::<BlockType>(node)
        .and_then(|block_type| block_type.type_use())
        .and_then(|type_use| {
            let node = type_use.syntax();
            service.get_type_use_sig(uri, SyntaxNodePtr::new(node), node.green().into())
        })
}

fn render_sig(db: &dyn TypesAnalyzerCtx, signature: Signature) -> String {
    let mut ret = String::with_capacity(signature.params.len() * 9 + signature.results.len() * 10);
    let params = signature
        .params
        .into_iter()
        .map(|(ty, name)| {
            if let Some(name) = name {
                format!("(param {} {})", db.lookup_ident(name), ty.render(db))
            } else {
                format!("(param {})", ty.render(db))
            }
        })
        .join(" ");
    ret.push_str(&params);
    let results = signature
        .results
        .into_iter()
        .map(|ty| format!("(result {})", ty.render(db)))
        .join(" ");
    if !params.is_empty() && !results.is_empty() {
        ret.push(' ');
    }
    ret.push_str(&results);
    ret
}

fn render_compact_sig(db: &dyn TypesAnalyzerCtx, signature: Signature) -> String {
    let params = signature
        .params
        .iter()
        .map(|(ty, _)| ty.render_compact(db))
        .join(", ");
    let results = signature
        .results
        .iter()
        .map(|ty| ty.render_compact(db))
        .join(", ");
    format!("[{params}] -> [{results}]")
}

fn render_func_header(
    db: &dyn TypesAnalyzerCtx,
    name: Option<InternIdent>,
    signature: Option<Signature>,
) -> String {
    let mut content = "(func".to_string();
    if let Some(name) = name {
        content.push(' ');
        content.push_str(&db.lookup_ident(name));
    }
    if let Some(sig) = signature {
        if !sig.params.is_empty() || !sig.results.is_empty() {
            content.push(' ');
            content.push_str(&db.render_sig(sig));
        }
    }
    content.push(')');
    content
}

fn render_block_header(
    db: &dyn TypesAnalyzerCtx,
    kind: SyntaxKind,
    name: Option<InternIdent>,
    signature: Option<Signature>,
) -> String {
    let mut content = format!(
        "({}",
        match kind {
            SyntaxKind::BLOCK_IF => "if",
            SyntaxKind::BLOCK_LOOP => "loop",
            _ => "block",
        }
    );
    if let Some(name) = name {
        content.push(' ');
        content.push_str(&db.lookup_ident(name));
    }
    if let Some(sig) = signature {
        if !sig.params.is_empty() || !sig.results.is_empty() {
            content.push(' ');
            content.push_str(&db.render_sig(sig));
        }
    }
    content.push(')');
    content
}

pub(crate) fn resolve_param_types(
    service: &LanguageService,
    uri: InternUri,
    instr: &SyntaxNode,
) -> Option<Vec<OperandType>> {
    debug_assert!(instr.kind() == SyntaxKind::PLAIN_INSTR);
    let instr_name = support::token(instr, SyntaxKind::INSTR_NAME)?;
    let instr_name = instr_name.text();
    if matches!(instr_name, "call" | "return_call") {
        let symbol_table = service.symbol_table(uri);
        let idx = instr
            .children()
            .find(|child| child.kind() == SyntaxKind::IMMEDIATE)?;
        let func = symbol_table.find_def(SymbolKey::new(&idx))?;
        service
            .get_func_sig(uri, func.key, func.green.clone())
            .map(|sig| {
                sig.params
                    .iter()
                    .map(|(ty, ..)| OperandType::Val(*ty))
                    .collect()
            })
    } else {
        INSTR_SIG.get(instr_name).map(|sig| sig.params.clone())
    }
}

pub(crate) fn resolve_br_types(
    service: &LanguageService,
    uri: InternUri,
    symbol_table: &SymbolTable,
    immediate: &Immediate,
) -> Vec<OperandType> {
    let key = SymbolKey::new(immediate.syntax());
    symbol_table
        .blocks
        .iter()
        .find(|block| block.ref_key == key)
        .and_then(|block| {
            get_block_sig(
                service,
                uri,
                &block
                    .def_key
                    .to_node(&SyntaxNode::new_root(service.root(uri))),
            )
        })
        .map(|sig| sig.results.into_iter().map(OperandType::Val).collect())
        .unwrap_or_default()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum ValType {
    I32,
    I64,
    F32,
    F64,
    V128,
    Ref(RefType),
}
impl ValType {
    pub(crate) fn from_ast(node: &AstValType, db: &dyn TypesAnalyzerCtx) -> Option<Self> {
        Self::from_green(&node.syntax().green(), db)
    }

    pub(crate) fn from_green(node: &GreenNodeData, db: &dyn TypesAnalyzerCtx) -> Option<Self> {
        match WatLanguage::kind_from_raw(node.kind()) {
            SyntaxKind::NUM_TYPE => match node
                .children()
                .next()
                .and_then(|child| child.into_token())?
                .text()
            {
                "i32" => Some(ValType::I32),
                "i64" => Some(ValType::I64),
                "f32" => Some(ValType::F32),
                "f64" => Some(ValType::F64),
                _ => None,
            },
            SyntaxKind::VEC_TYPE => Some(ValType::V128),
            SyntaxKind::REF_TYPE => {
                let mut children = node.children();
                match children.next().and_then(|child| child.into_token())?.text() {
                    "anyref" => {
                        return Some(ValType::Ref(RefType {
                            heap_ty: HeapType::Any,
                            nullable: true,
                        }));
                    }
                    "eqref" => {
                        return Some(ValType::Ref(RefType {
                            heap_ty: HeapType::Eq,
                            nullable: true,
                        }));
                    }
                    "i31ref" => {
                        return Some(ValType::Ref(RefType {
                            heap_ty: HeapType::I31,
                            nullable: true,
                        }));
                    }
                    "structref" => {
                        return Some(ValType::Ref(RefType {
                            heap_ty: HeapType::Struct,
                            nullable: true,
                        }));
                    }
                    "arrayref" => {
                        return Some(ValType::Ref(RefType {
                            heap_ty: HeapType::Array,
                            nullable: true,
                        }));
                    }
                    "nullref" => {
                        return Some(ValType::Ref(RefType {
                            heap_ty: HeapType::None,
                            nullable: true,
                        }));
                    }
                    "funcref" => {
                        return Some(ValType::Ref(RefType {
                            heap_ty: HeapType::Func,
                            nullable: true,
                        }));
                    }
                    "nullfuncref" => {
                        return Some(ValType::Ref(RefType {
                            heap_ty: HeapType::NoFunc,
                            nullable: true,
                        }));
                    }
                    "externref" => {
                        return Some(ValType::Ref(RefType {
                            heap_ty: HeapType::Extern,
                            nullable: true,
                        }));
                    }
                    "nullexternref" => {
                        return Some(ValType::Ref(RefType {
                            heap_ty: HeapType::NoExtern,
                            nullable: false,
                        }));
                    }
                    _ => {}
                }
                let mut nullable = false;
                for node_or_token in children {
                    match node_or_token {
                        NodeOrToken::Node(node) if node.kind() == SyntaxKind::HEAP_TYPE.into() => {
                            return match node.children().next() {
                                Some(NodeOrToken::Node(node))
                                    if node.kind() == SyntaxKind::INDEX.into() =>
                                {
                                    let token = node.children().next()?.into_token()?;
                                    match WatLanguage::kind_from_raw(token.kind()) {
                                        SyntaxKind::UNSIGNED_INT => Some(ValType::Ref(RefType {
                                            heap_ty: HeapType::Type(Idx {
                                                num: token.text().parse().ok(),
                                                name: None,
                                            }),
                                            nullable,
                                        })),
                                        SyntaxKind::IDENT => Some(ValType::Ref(RefType {
                                            heap_ty: HeapType::Type(Idx {
                                                num: None,
                                                name: Some(db.ident(token.text().into())),
                                            }),
                                            nullable,
                                        })),
                                        _ => None,
                                    }
                                }
                                Some(NodeOrToken::Token(token))
                                    if token.kind() == SyntaxKind::TYPE_KEYWORD.into() =>
                                {
                                    match token.text() {
                                        "any" => Some(ValType::Ref(RefType {
                                            heap_ty: HeapType::Any,
                                            nullable,
                                        })),
                                        "eq" => Some(ValType::Ref(RefType {
                                            heap_ty: HeapType::Eq,
                                            nullable,
                                        })),
                                        "i31" => Some(ValType::Ref(RefType {
                                            heap_ty: HeapType::I31,
                                            nullable,
                                        })),
                                        "struct" => Some(ValType::Ref(RefType {
                                            heap_ty: HeapType::Struct,
                                            nullable,
                                        })),
                                        "array" => Some(ValType::Ref(RefType {
                                            heap_ty: HeapType::Array,
                                            nullable,
                                        })),
                                        "none" => Some(ValType::Ref(RefType {
                                            heap_ty: HeapType::None,
                                            nullable,
                                        })),
                                        "func" => Some(ValType::Ref(RefType {
                                            heap_ty: HeapType::Func,
                                            nullable,
                                        })),
                                        "nofunc" => Some(ValType::Ref(RefType {
                                            heap_ty: HeapType::NoFunc,
                                            nullable,
                                        })),
                                        "extern" => Some(ValType::Ref(RefType {
                                            heap_ty: HeapType::Extern,
                                            nullable,
                                        })),
                                        "noextern" => Some(ValType::Ref(RefType {
                                            heap_ty: HeapType::NoExtern,
                                            nullable,
                                        })),
                                        _ => None,
                                    }
                                }
                                _ => None,
                            };
                        }
                        NodeOrToken::Token(token)
                            if token.kind() == SyntaxKind::KEYWORD.into()
                                && token.text() == "null" =>
                        {
                            nullable = true;
                        }
                        _ => {}
                    }
                }
                None
            }
            _ => None,
        }
    }

    pub(crate) fn render<'a>(&'a self, db: &'a dyn TypesAnalyzerCtx) -> ValTypeRender<'a, false> {
        ValTypeRender { ty: self, db }
    }

    pub(crate) fn render_compact<'a>(
        &'a self,
        db: &'a dyn TypesAnalyzerCtx,
    ) -> ValTypeRender<'a, true> {
        ValTypeRender { ty: self, db }
    }
}
pub(crate) struct ValTypeRender<'a, const COMPACT: bool> {
    ty: &'a ValType,
    db: &'a dyn TypesAnalyzerCtx,
}
impl Display for ValTypeRender<'_, false> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.ty {
            ValType::I32 => write!(f, "i32"),
            ValType::I64 => write!(f, "i64"),
            ValType::F32 => write!(f, "f32"),
            ValType::F64 => write!(f, "f64"),
            ValType::V128 => write!(f, "v128"),
            ValType::Ref(ty) => {
                write!(f, "(ref ")?;
                if ty.nullable {
                    write!(f, "null ")?;
                }
                match ty.heap_ty {
                    HeapType::Type(idx) => {
                        if let Some(name) = idx.name {
                            write!(f, "{}", self.db.lookup_ident(name))?;
                        } else if let Some(num) = idx.num {
                            write!(f, "{num}")?;
                        }
                    }
                    HeapType::Any => write!(f, "any")?,
                    HeapType::Eq => write!(f, "eq")?,
                    HeapType::I31 => write!(f, "i31")?,
                    HeapType::Struct => write!(f, "struct")?,
                    HeapType::Array => write!(f, "array")?,
                    HeapType::None => write!(f, "none")?,
                    HeapType::Func => write!(f, "func")?,
                    HeapType::NoFunc => write!(f, "nofunc")?,
                    HeapType::Extern => write!(f, "extern")?,
                    HeapType::NoExtern => write!(f, "noextern")?,
                }
                write!(f, ")")
            }
        }
    }
}
impl Display for ValTypeRender<'_, true> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.ty {
            ValType::I32 => write!(f, "i32"),
            ValType::I64 => write!(f, "i64"),
            ValType::F32 => write!(f, "f32"),
            ValType::F64 => write!(f, "f64"),
            ValType::V128 => write!(f, "v128"),
            ValType::Ref(ty) => {
                if ty.nullable {
                    write!(f, "nullable ")?;
                }
                match ty.heap_ty {
                    HeapType::Type(idx) => {
                        if let Some(name) = idx.name {
                            write!(f, "type `{}`", self.db.lookup_ident(name))?;
                        } else if let Some(num) = idx.num {
                            write!(f, "type `{num}`")?;
                        }
                    }
                    HeapType::Any => write!(f, "any")?,
                    HeapType::Eq => write!(f, "eq")?,
                    HeapType::I31 => write!(f, "i31")?,
                    HeapType::Struct => write!(f, "struct")?,
                    HeapType::Array => write!(f, "array")?,
                    HeapType::None => write!(f, "none")?,
                    HeapType::Func => write!(f, "func")?,
                    HeapType::NoFunc => write!(f, "nofunc")?,
                    HeapType::Extern => write!(f, "extern")?,
                    HeapType::NoExtern => write!(f, "noextern")?,
                }
                write!(f, " ref")
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct RefType {
    pub heap_ty: HeapType,
    pub nullable: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum HeapType {
    Type(Idx),
    Any,
    Eq,
    I31,
    Struct,
    Array,
    None,
    Func,
    NoFunc,
    Extern,
    NoExtern,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum OperandType {
    Val(ValType),
    Any,
}
impl OperandType {
    pub(crate) fn render<'a>(&'a self, db: &'a dyn TypesAnalyzerCtx) -> OperandTypeRender<'a> {
        OperandTypeRender { ty: self, db }
    }
}
pub(crate) struct OperandTypeRender<'a> {
    ty: &'a OperandType,
    db: &'a dyn TypesAnalyzerCtx,
}
impl Display for OperandTypeRender<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.ty {
            OperandType::Val(ty) => write!(f, "{}", ty.render_compact(self.db)),
            OperandType::Any => write!(f, "any"),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub(crate) struct Signature {
    pub(crate) params: Vec<(ValType, Option<InternIdent>)>,
    pub(crate) results: Vec<ValType>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ResolvedSig {
    pub(crate) params: Vec<OperandType>,
    pub(crate) results: Vec<OperandType>,
}

impl From<Signature> for ResolvedSig {
    fn from(sig: Signature) -> Self {
        ResolvedSig {
            params: sig
                .params
                .into_iter()
                .map(|(ty, _)| OperandType::Val(ty))
                .collect(),
            results: sig.results.into_iter().map(OperandType::Val).collect(),
        }
    }
}

#[derive(Clone)]
pub(crate) struct SymbolWithGreenEq(Symbol);
impl PartialEq for SymbolWithGreenEq {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.0.green == other.0.green
    }
}
impl Eq for SymbolWithGreenEq {}
impl Hash for SymbolWithGreenEq {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.0.green.hash(state);
    }
}
impl From<Symbol> for SymbolWithGreenEq {
    fn from(symbol: Symbol) -> Self {
        SymbolWithGreenEq(symbol)
    }
}
impl Deref for SymbolWithGreenEq {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Debug for SymbolWithGreenEq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
