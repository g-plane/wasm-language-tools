use super::{signature::Signature, types::ValType, TypesAnalyzerCtx};
use rowan::{
    ast::{support, AstNode},
    GreenNode, Language, NodeOrToken,
};
use wat_syntax::{
    ast::{Param, Result, ValType as AstValType},
    SyntaxKind, SyntaxNode, WatLanguage,
};

pub(super) fn extract_type(db: &dyn TypesAnalyzerCtx, node: GreenNode) -> Option<ValType> {
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

pub(super) fn extract_global_type(db: &dyn TypesAnalyzerCtx, node: GreenNode) -> Option<ValType> {
    node.children()
        .find_map(|child| match child {
            NodeOrToken::Node(node) if node.kind() == SyntaxKind::GLOBAL_TYPE.into() => Some(node),
            _ => None,
        })
        .and_then(|global_type| db.extract_type(global_type.to_owned()))
}

pub(super) fn extract_sig(db: &dyn TypesAnalyzerCtx, node: GreenNode) -> Signature {
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
