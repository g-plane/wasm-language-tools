use super::TypesAnalyzerCtx;
use crate::idx::Idx;
use rowan::{ast::AstNode, GreenNodeData, Language, NodeOrToken};
use wat_syntax::{ast::ValType as AstValType, SyntaxKind, WatLanguage};

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
