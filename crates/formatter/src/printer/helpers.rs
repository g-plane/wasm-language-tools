use crate::config::WrapBefore;
use rowan::ast::{AstChildren, AstNode};
use tiny_pretty::Doc;
use wat_syntax::WatLanguage;

pub fn wrap_before<N>(children: &AstChildren<N>, option: WrapBefore) -> Doc<'static>
where
    N: AstNode<Language = WatLanguage> + Clone,
{
    match option {
        WrapBefore::Never => Doc::space(),
        WrapBefore::Overflow => Doc::soft_line(),
        WrapBefore::MultiOnly => {
            if children.clone().peekable().peek().is_some() {
                Doc::hard_line()
            } else {
                Doc::space()
            }
        }
        WrapBefore::Always => Doc::hard_line(),
    }
}
