use super::{
    signature::Signature,
    types::{FieldType, Fields, ValType},
};
use crate::idx::{Idx, IdxGen, InternIdent};
use rowan::{
    GreenNode, Language, NodeOrToken,
    ast::{AstNode, support},
};
use wat_syntax::{
    SyntaxKind, SyntaxNode, WatLanguage,
    ast::{Param, Result, StructType, ValType as AstValType},
};

pub(crate) fn extract_type<'db>(
    db: &'db dyn salsa::Database,
    node: GreenNode,
) -> Option<ValType<'db>> {
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

pub(crate) fn extract_global_type<'db>(
    db: &'db dyn salsa::Database,
    node: GreenNode,
) -> Option<ValType<'db>> {
    node.children()
        .find_map(|child| match child {
            NodeOrToken::Node(node) if node.kind() == SyntaxKind::GLOBAL_TYPE.into() => Some(node),
            _ => None,
        })
        .and_then(|global_type| extract_type(db, global_type.to_owned()))
}

pub(crate) fn extract_sig<'db>(db: &'db dyn salsa::Database, node: GreenNode) -> Signature<'db> {
    let root = SyntaxNode::new_root(node);
    let params = support::children::<Param>(&root).fold(vec![], |mut acc, param| {
        if let Some((ty, ident)) = param
            .val_types()
            .next()
            .and_then(|ty| ValType::from_ast(&ty, db))
            .zip(param.ident_token())
        {
            acc.push((ty, Some(InternIdent::new(db, ident.text()))));
        } else {
            acc.extend(
                param
                    .val_types()
                    .filter_map(|ty| ValType::from_ast(&ty, db))
                    .map(|ty| (ty, None)),
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

pub(super) fn extract_fields<'db>(
    db: &'db dyn salsa::Database,
    struct_ty: &StructType,
) -> Fields<'db> {
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
