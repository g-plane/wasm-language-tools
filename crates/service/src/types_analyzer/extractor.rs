use super::{
    signature::Signature,
    types::{FieldType, Fields, ValType},
};
use crate::idx::{Idx, IdxGen, InternIdent};
use rowan::{GreenNodeData, Language, NodeOrToken, ast::AstNode};
use std::ops::ControlFlow;
use wat_syntax::{
    SyntaxKind, WatLanguage,
    ast::{StructType, ValType as AstValType},
};

pub(crate) fn extract_type<'db>(db: &'db dyn salsa::Database, green: &GreenNodeData) -> Option<ValType<'db>> {
    ValType::from_green(green, db).or_else(|| {
        green.children().find_map(|child| match child {
            NodeOrToken::Node(node) if AstValType::can_cast(WatLanguage::kind_from_raw(node.kind())) => {
                ValType::from_green(node, db)
            }
            _ => None,
        })
    })
}

pub(crate) fn extract_global_type<'db>(db: &'db dyn salsa::Database, green: &GreenNodeData) -> Option<ValType<'db>> {
    green
        .children()
        .find_map(|child| match child {
            NodeOrToken::Node(node) if node.kind() == SyntaxKind::GLOBAL_TYPE.into() => Some(node),
            _ => None,
        })
        .and_then(|global_type| extract_type(db, global_type))
}

pub(crate) fn extract_sig<'db>(db: &'db dyn salsa::Database, node: &GreenNodeData) -> Signature<'db> {
    let mut params = Vec::with_capacity(1);
    let mut results = Vec::with_capacity(1);
    node.children().filter_map(|child| child.into_node()).for_each(|node| {
        match WatLanguage::kind_from_raw(node.kind()) {
            SyntaxKind::PARAM => {
                let mut ident = None;
                let _ = node.children().try_for_each(|child| match child {
                    NodeOrToken::Node(node) => {
                        if let Some(ty) = ValType::from_green(node, db) {
                            params.push((ty, ident));
                            if ident.is_some() {
                                ControlFlow::Break(())
                            } else {
                                ControlFlow::Continue(())
                            }
                        } else {
                            ControlFlow::Continue(())
                        }
                    }
                    NodeOrToken::Token(token) if token.kind() == SyntaxKind::IDENT.into() => {
                        ident = Some(InternIdent::new(db, token.text()));
                        ControlFlow::Continue(())
                    }
                    _ => ControlFlow::Continue(()),
                });
            }
            SyntaxKind::RESULT => results.extend(
                node.children()
                    .filter_map(|child| child.into_node().and_then(|node| ValType::from_green(node, db))),
            ),
            _ => {}
        }
    });
    Signature { params, results }
}

pub(super) fn extract_fields<'db>(db: &'db dyn salsa::Database, struct_ty: &StructType) -> Fields<'db> {
    let mut field_idx_gen = IdxGen::default();
    Fields(struct_ty.fields().fold(vec![], |mut acc, field| {
        if let Some((ty, ident)) = field
            .field_types()
            .next()
            .and_then(|ty| FieldType::from_ast(&ty, db))
            .zip(field.ident_token())
        {
            acc.push((
                ty,
                Idx {
                    num: Some(field_idx_gen.pull()),
                    name: Some(InternIdent::new(db, ident.text())),
                },
            ));
        } else {
            acc.extend(
                field
                    .field_types()
                    .filter_map(|ty| FieldType::from_ast(&ty, db))
                    .map(|ty| {
                        (
                            ty,
                            Idx {
                                num: Some(field_idx_gen.pull()),
                                name: None,
                            },
                        )
                    }),
            );
        }
        acc
    }))
}
