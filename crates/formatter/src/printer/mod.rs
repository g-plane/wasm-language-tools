use crate::config::{FormatOptions, LanguageOptions, MultiLine, WrapBefore};
use rowan::{
    Direction, NodeOrToken,
    ast::{AstChildren, AstNode, support},
};
use std::iter;
use tiny_pretty::Doc;
use wat_syntax::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken, WatLanguage, ast::*};

mod instr;
mod module;
mod ty;

pub(super) struct Ctx<'a> {
    pub indent_width: usize,
    pub options: &'a LanguageOptions,
}
impl<'a> Ctx<'a> {
    pub(crate) fn new(options: &'a FormatOptions) -> Self {
        Self {
            indent_width: options.layout.indent_width,
            options: &options.language,
        }
    }

    pub(crate) fn format_right_paren<N>(&self, node: &N) -> Doc<'static>
    where
        N: AstNode<Language = WatLanguage>,
    {
        let node = node.syntax();
        let docs = if node
            .last_child_or_token()
            .and_then(|node_or_token| match node_or_token {
                NodeOrToken::Token(token) if token.kind() == SyntaxKind::R_PAREN => Some(token),
                _ => None,
            })
            .or_else(|| support::token(node, SyntaxKind::R_PAREN))
            .and_then(|token| {
                token
                    .siblings_with_tokens(Direction::Prev)
                    .skip(1)
                    .map_while(|node_or_token| node_or_token.into_token())
                    .find(|token| token.kind() != SyntaxKind::WHITESPACE)
            })
            .is_some_and(|token| token.kind() == SyntaxKind::LINE_COMMENT)
        {
            vec![Doc::hard_line(), Doc::text(")")]
        } else if self.options.split_closing_parens {
            vec![Doc::line_or_nil(), Doc::text(")")]
        } else {
            vec![Doc::text(")")]
        };
        Doc::list(docs)
    }
}

pub(crate) fn format_node(node: SyntaxNode, ctx: &Ctx) -> Option<Doc<'static>> {
    match node.kind() {
        SyntaxKind::MODULE_NAME => ModuleName::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::NAME => Name::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::NUM_TYPE => NumType::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::VEC_TYPE => VecType::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::REF_TYPE => RefType::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::HEAP_TYPE => HeapType::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::PACKED_TYPE => PackedType::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::FIELD_TYPE => FieldType::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::STRUCT_TYPE => StructType::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::ARRAY_TYPE => ArrayType::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::FUNC_TYPE => FuncType::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::PARAM => Param::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::RESULT => Result::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::FIELD => Field::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::SUB_TYPE => SubType::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::TABLE_TYPE => TableType::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::MEM_TYPE => MemType::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::ADDR_TYPE => AddrType::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::GLOBAL_TYPE => GlobalType::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::BLOCK_TYPE => BlockType::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::PLAIN_INSTR => PlainInstr::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::BLOCK_BLOCK => BlockBlock::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::BLOCK_LOOP => BlockLoop::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::BLOCK_IF => BlockIf::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::BLOCK_IF_THEN => BlockIfThen::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::BLOCK_IF_ELSE => BlockIfElse::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::BLOCK_TRY_TABLE => BlockTryTable::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::CATCH => Catch::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::CATCH_ALL => CatchAll::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::MEM_ARG => MemArg::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::IMMEDIATE => Immediate::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::TYPE_USE => TypeUse::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::LIMITS => Limits::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::IMPORT => Import::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::EXPORT => Export::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::EXTERN_TYPE_FUNC => ExternTypeFunc::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::EXTERN_TYPE_TABLE => ExternTypeTable::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::EXTERN_TYPE_MEMORY => ExternTypeMemory::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::EXTERN_TYPE_GLOBAL => ExternTypeGlobal::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::EXTERN_TYPE_TAG => ExternTypeTag::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::EXTERN_IDX_FUNC => ExternIdxFunc::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::EXTERN_IDX_TABLE => ExternIdxTable::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::EXTERN_IDX_MEMORY => ExternIdxMemory::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::EXTERN_IDX_GLOBAL => ExternIdxGlobal::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::EXTERN_IDX_TAG => ExternIdxTag::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::INDEX => Index::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::LOCAL => Local::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::MEM_PAGE_SIZE => MemPageSize::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::MEM_USE => MemUse::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::OFFSET => Offset::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::ELEM => Elem::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::ELEM_LIST => ElemList::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::ELEM_EXPR => ElemExpr::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::TABLE_USE => TableUse::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::DATA => Data::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::MODULE => Module::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::MODULE_FIELD_DATA => ModuleFieldData::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::MODULE_FIELD_ELEM => ModuleFieldElem::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::MODULE_FIELD_EXPORT => ModuleFieldExport::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::MODULE_FIELD_FUNC => ModuleFieldFunc::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::MODULE_FIELD_GLOBAL => ModuleFieldGlobal::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::MODULE_FIELD_IMPORT => ModuleFieldImport::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::MODULE_FIELD_MEMORY => ModuleFieldMemory::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::MODULE_FIELD_START => ModuleFieldStart::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::MODULE_FIELD_TABLE => ModuleFieldTable::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::MODULE_FIELD_TAG => ModuleFieldTag::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::TYPE_DEF => TypeDef::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::REC_TYPE => RecType::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::ROOT => Root::cast(node).map(|node| node.doc(ctx)),
        _ => None,
    }
}

pub(super) trait DocGen {
    fn doc(&self, ctx: &Ctx) -> Doc<'static>;
}

impl DocGen for Root {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);

        let mut children = self.syntax().children_with_tokens().peekable();
        let mut prev_kind = SyntaxKind::WHITESPACE;
        while let Some(element) = children.next() {
            let kind = element.kind();
            match element {
                SyntaxElement::Node(node) => {
                    if should_ignore(&node, ctx) {
                        reflow(&node.to_string(), &mut docs);
                    } else if let Some(module) = Module::cast(node) {
                        docs.push(module.doc(ctx));
                    }
                }
                SyntaxElement::Token(token) => match token.kind() {
                    SyntaxKind::LINE_COMMENT => {
                        docs.push(format_line_comment(token.text(), ctx));
                    }
                    SyntaxKind::BLOCK_COMMENT => {
                        docs.push(format_block_comment(token.text(), ctx));
                    }
                    SyntaxKind::WHITESPACE => {
                        if token.index() > 0 && children.peek().is_some() {
                            match token.text().chars().filter(|c| *c == '\n').count() {
                                0 => {
                                    if prev_kind == SyntaxKind::LINE_COMMENT {
                                        docs.push(Doc::hard_line());
                                    } else {
                                        docs.push(Doc::space());
                                    }
                                }
                                1 => {
                                    docs.push(Doc::hard_line());
                                }
                                _ => {
                                    docs.push(Doc::empty_line());
                                    docs.push(Doc::hard_line());
                                }
                            }
                        }
                    }
                    _ => docs.push(Doc::text(token.to_string())),
                },
            }
            prev_kind = kind;
        }

        docs.push(Doc::hard_line());
        Doc::list(docs)
    }
}

fn format_trivias_after_node<N>(node: N, ctx: &Ctx) -> Vec<Doc<'static>>
where
    N: AstNode<Language = WatLanguage>,
{
    let trivias = node
        .syntax()
        .siblings_with_tokens(Direction::Next)
        .skip(1)
        .map_while(into_formattable_trivia)
        .collect::<Vec<_>>();
    if trivias
        .iter()
        .all(|token| token.kind() == SyntaxKind::WHITESPACE && token.text().chars().filter(|c| *c == '\n').count() < 2)
    {
        return vec![];
    }
    let mut docs = Vec::with_capacity(trivias.len());
    if trivias.first().is_some_and(|token| token.kind().is_comment()) {
        docs.push(Doc::soft_line());
    }
    trivias.iter().for_each(|token| match token.kind() {
        SyntaxKind::LINE_COMMENT => {
            docs.push(format_line_comment(token.text(), ctx));
        }
        SyntaxKind::BLOCK_COMMENT => {
            docs.push(format_block_comment(token.text(), ctx));
        }
        SyntaxKind::WHITESPACE => match token.text().chars().filter(|c| *c == '\n').count() {
            0 => docs.push(Doc::space()),
            1 => docs.push(Doc::hard_line()),
            _ => {
                docs.push(Doc::empty_line());
                docs.push(Doc::hard_line());
            }
        },
        SyntaxKind::ERROR | SyntaxKind::ANNOT_START | SyntaxKind::ANNOT_ELEM | SyntaxKind::ANNOT_END => {
            docs.push(Doc::text(token.to_string()));
        }
        _ => {}
    });
    docs
}
fn format_trivias_after_token(token: SyntaxToken, ctx: &Ctx) -> Vec<Doc<'static>> {
    let trivias = token
        .siblings_with_tokens(Direction::Next)
        .skip(1)
        .map_while(into_formattable_trivia)
        .skip_while(|current| token.kind() == SyntaxKind::L_PAREN && current.kind() == SyntaxKind::WHITESPACE)
        .collect::<Vec<_>>();
    if trivias.iter().all(|token| token.kind() == SyntaxKind::WHITESPACE) {
        return vec![];
    }
    let mut docs = Vec::with_capacity(trivias.len());
    trivias.iter().for_each(|token| match token.kind() {
        SyntaxKind::LINE_COMMENT => {
            docs.push(format_line_comment(token.text(), ctx));
        }
        SyntaxKind::BLOCK_COMMENT => {
            docs.push(format_block_comment(token.text(), ctx));
        }
        SyntaxKind::WHITESPACE => match token.text().chars().filter(|c| *c == '\n').count() {
            0 => docs.push(Doc::space()),
            1 => docs.push(Doc::hard_line()),
            _ => {
                docs.push(Doc::empty_line());
                docs.push(Doc::hard_line());
            }
        },
        SyntaxKind::ERROR | SyntaxKind::ANNOT_START | SyntaxKind::ANNOT_ELEM | SyntaxKind::ANNOT_END => {
            docs.push(Doc::text(token.to_string()));
        }
        _ => {}
    });
    docs
}

fn into_formattable_trivia(node_or_token: SyntaxElement) -> Option<SyntaxToken> {
    node_or_token.into_token().and_then(|token| match token.kind() {
        SyntaxKind::LINE_COMMENT
        | SyntaxKind::BLOCK_COMMENT
        | SyntaxKind::ERROR
        | SyntaxKind::ANNOT_START
        | SyntaxKind::ANNOT_ELEM
        | SyntaxKind::ANNOT_END => Some(token),
        SyntaxKind::WHITESPACE
            if token.next_token().is_none_or(|token| match token.kind() {
                SyntaxKind::R_PAREN => false,
                SyntaxKind::KEYWORD => token.text() != "end",
                _ => true,
            }) =>
        {
            Some(token)
        }
        _ => None,
    })
}

fn format_line_comment(text: &str, ctx: &Ctx) -> Doc<'static> {
    if ctx.options.format_comments {
        let content = text.strip_prefix(";;").expect("line comment must start with `;;`");
        if content.is_empty() || content.starts_with([' ', '\t']) {
            Doc::text(text.to_owned())
        } else {
            Doc::text(format!(";; {content}"))
        }
    } else {
        Doc::text(text.to_owned())
    }
}

fn format_block_comment(text: &str, ctx: &Ctx) -> Doc<'static> {
    if ctx.options.format_comments {
        let content = text
            .strip_prefix("(;")
            .and_then(|s| s.strip_suffix(";)"))
            .expect("block comment must be wrapped between `(;` and `;)`");
        let has_leading_ws = content.starts_with([' ', '\t']);
        let has_trailing_ws = content.ends_with([' ', '\t']);
        if content.is_empty() || has_leading_ws && has_trailing_ws {
            Doc::text(text.to_owned())
        } else if has_leading_ws {
            Doc::text(format!("(;{content} ;)"))
        } else if has_trailing_ws {
            Doc::text(format!("(; {content};)"))
        } else {
            Doc::text(format!("(; {content} ;)"))
        }
    } else {
        Doc::text(text.to_owned())
    }
}

fn reflow(text: &str, docs: &mut Vec<Doc<'static>>) {
    let mut lines = text.lines();
    if let Some(line) = lines.next() {
        docs.push(Doc::text(line.to_owned()));
    }
    for line in lines {
        docs.push(Doc::empty_line());
        docs.push(Doc::text(line.to_owned()));
    }
}

fn should_ignore(node: &SyntaxNode, ctx: &Ctx) -> bool {
    // for the case that comment comes in the middle of a list of nodes
    node.prev_sibling_or_token()
        .and_then(|element| element.prev_sibling_or_token())
        .or_else(|| {
            // for the case that comment comes at the start or the end of a list of nodes
            node.parent()
                .and_then(|parent| parent.prev_sibling_or_token())
                .and_then(|parent| parent.prev_sibling_or_token())
        })
        .as_ref()
        .and_then(|element| match element {
            SyntaxElement::Token(token) if token.kind() == SyntaxKind::LINE_COMMENT => token
                .text()
                .strip_prefix(";;")
                .and_then(|s| s.trim_start().strip_prefix(&ctx.options.ignore_comment_directive)),
            _ => None,
        })
        .is_some_and(|rest| rest.is_empty() || rest.starts_with(|c: char| c.is_ascii_whitespace()))
}

fn wrap_before<N>(children: &mut iter::Peekable<AstChildren<N>>, option: WrapBefore) -> Doc<'static>
where
    N: AstNode<Language = WatLanguage> + Clone,
{
    match option {
        WrapBefore::Never => Doc::space(),
        WrapBefore::Overflow => Doc::soft_line(),
        WrapBefore::MultiOnly => {
            if children.peek().is_some() {
                Doc::hard_line()
            } else {
                Doc::space()
            }
        }
        WrapBefore::Always => Doc::hard_line(),
    }
}

fn whitespace_of_multi_line<N>(option: MultiLine, first: Option<&N>) -> Doc<'static>
where
    N: AstNode<Language = WatLanguage>,
{
    match option {
        MultiLine::Never => Doc::space(),
        MultiLine::Overflow => Doc::line_or_space(),
        MultiLine::Smart => {
            if first.is_some_and(|first| {
                first
                    .syntax()
                    .siblings_with_tokens(Direction::Next)
                    .skip(1)
                    .map_while(NodeOrToken::into_token)
                    .any(|token| token.text().contains('\n'))
            }) {
                Doc::hard_line()
            } else {
                Doc::line_or_space()
            }
        }
        MultiLine::Wrap => Doc::soft_line(),
        MultiLine::Always => Doc::hard_line(),
    }
}
