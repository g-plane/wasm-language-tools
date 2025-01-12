use crate::{
    binder::{SymbolItem, SymbolItemKey, SymbolTablesCtx},
    data_set::INSTR_SIG,
    helpers,
    idx::InternIdent,
    syntax_tree::SyntaxTreeCtx,
    uri::InternUri,
    LanguageService,
};
use itertools::Itertools;
use rowan::{
    ast::{
        support::{child, children, token},
        AstNode,
    },
    GreenNode, GreenNodeData, Language, NodeOrToken,
};
use std::{
    fmt::{self, Debug, Display},
    hash::Hash,
    ops::Deref,
};
use wat_syntax::{
    ast::{Param, Result, TypeUse, ValType as AstValType},
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
fn extract_type(_: &dyn TypesAnalyzerCtx, node: GreenNode) -> Option<ValType> {
    (&*node).try_into().ok().or_else(|| {
        node.children().find_map(|child| match child {
            NodeOrToken::Node(node) if node.kind() == SyntaxKind::VAL_TYPE.into() => {
                node.try_into().ok()
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
    let params = children::<Param>(&root).fold(vec![], |mut acc, param| {
        if let Some((ident, ty)) = param.ident_token().zip(param.val_types().next()) {
            acc.push((ValType::from(ty), Some(db.ident(ident.text().to_string()))));
        } else {
            acc.extend(
                param
                    .val_types()
                    .map(|val_type| (ValType::from(val_type), None)),
            );
        }
        acc
    });
    let results = children::<Result>(&root)
        .flat_map(|result| result.val_types())
        .map(ValType::from)
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
                child::<TypeUse>(&node)
                    .and_then(|type_use| type_use.index())
                    .and_then(|idx| symbol_table.find_defs(SymbolItemKey::new(idx.syntax())))
                    .and_then(|mut symbols| symbols.next())
                    .and_then(|symbol| helpers::ast::find_func_type_of_type_def(&symbol.green))
                    .map(|func_type| db.extract_sig(func_type))
            }
        })
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
    node.children()
        .find(|child| child.kind() == SyntaxKind::BLOCK_TYPE)
        .and_then(|block_type| {
            service.get_func_sig(
                uri,
                SyntaxNodePtr::new(&block_type),
                block_type.green().into(),
            )
        })
}

fn render_sig(db: &dyn TypesAnalyzerCtx, signature: Signature) -> String {
    let mut ret = String::with_capacity(signature.params.len() * 9 + signature.results.len() * 10);
    let params = signature
        .params
        .into_iter()
        .map(|(ty, name)| {
            if let Some(name) = name {
                format!("(param {} {ty})", db.lookup_ident(name))
            } else {
                format!("(param {ty})")
            }
        })
        .join(" ");
    ret.push_str(&params);
    let results = signature
        .results
        .into_iter()
        .map(|ty| format!("(result {ty})"))
        .join(" ");
    if !params.is_empty() && !results.is_empty() {
        ret.push(' ');
    }
    ret.push_str(&results);
    ret
}

fn render_compact_sig(_: &dyn TypesAnalyzerCtx, signature: Signature) -> String {
    let params = signature
        .params
        .iter()
        .map(|(ty, _)| ty.to_string())
        .join(", ");
    let results = signature.results.iter().map(ValType::to_string).join(", ");
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
    let instr_name = token(instr, SyntaxKind::INSTR_NAME)?;
    let instr_name = instr_name.text();
    if matches!(instr_name, "call" | "return_call") {
        let symbol_table = service.symbol_table(uri);
        let idx = instr
            .children()
            .find(|child| child.kind() == SyntaxKind::IMMEDIATE)?;
        let func = symbol_table
            .find_defs(SymbolItemKey::new(&idx))
            .into_iter()
            .flatten()
            .next()?;
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum ValType {
    I32,
    I64,
    F32,
    F64,
    V128,
    FuncRef,
    ExternRef,
}
impl Display for ValType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValType::I32 => write!(f, "i32"),
            ValType::I64 => write!(f, "i64"),
            ValType::F32 => write!(f, "f32"),
            ValType::F64 => write!(f, "f64"),
            ValType::V128 => write!(f, "v128"),
            ValType::FuncRef => write!(f, "funcref"),
            ValType::ExternRef => write!(f, "externref"),
        }
    }
}

impl From<AstValType> for ValType {
    fn from(value: AstValType) -> Self {
        if let Some(num_type) = value.num_type() {
            match num_type.text() {
                "i32" => ValType::I32,
                "i64" => ValType::I64,
                "f32" => ValType::F32,
                "f64" => ValType::F64,
                _ => unreachable!("unsupported numtype"),
            }
        } else if value.vec_type().is_some() {
            ValType::V128
        } else if let Some(ref_type) = value.ref_type() {
            match ref_type.text() {
                "funcref" => ValType::FuncRef,
                "externref" => ValType::ExternRef,
                _ => unreachable!("unsupported reftype"),
            }
        } else {
            unreachable!("unsupported valtype");
        }
    }
}

impl TryFrom<&GreenNodeData> for ValType {
    type Error = ();
    fn try_from(node: &GreenNodeData) -> std::result::Result<Self, Self::Error> {
        node.children()
            .find_map(|child| {
                if let NodeOrToken::Token(token) = child {
                    match WatLanguage::kind_from_raw(token.kind()) {
                        SyntaxKind::NUM_TYPE => match token.text() {
                            "i32" => Some(ValType::I32),
                            "i64" => Some(ValType::I64),
                            "f32" => Some(ValType::F32),
                            "f64" => Some(ValType::F64),
                            _ => None,
                        },
                        SyntaxKind::VEC_TYPE => Some(ValType::V128),
                        SyntaxKind::REF_TYPE => match token.text() {
                            "funcref" => Some(ValType::FuncRef),
                            "externref" => Some(ValType::ExternRef),
                            _ => None,
                        },
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .ok_or(())
    }
}

#[derive(Clone, Debug)]
pub(crate) enum OperandType {
    Val(ValType),
    Any,
}
impl Display for OperandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperandType::Val(ty) => Display::fmt(ty, f),
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
pub(crate) struct SymbolItemWithGreenEq(SymbolItem);
impl PartialEq for SymbolItemWithGreenEq {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.0.green == other.0.green
    }
}
impl Eq for SymbolItemWithGreenEq {}
impl Hash for SymbolItemWithGreenEq {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.0.green.hash(state);
    }
}
impl From<SymbolItem> for SymbolItemWithGreenEq {
    fn from(symbol: SymbolItem) -> Self {
        SymbolItemWithGreenEq(symbol)
    }
}
impl Deref for SymbolItemWithGreenEq {
    type Target = SymbolItem;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Debug for SymbolItemWithGreenEq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
