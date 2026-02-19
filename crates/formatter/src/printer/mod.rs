use self::{instr::*, module::*, ty::*};
use crate::config::{FormatOptions, LanguageOptions, MultiLine, WrapBefore};
use std::iter;
use tiny_pretty::Doc;
use wat_syntax::{AmberNode, AmberToken, NodeOrToken, SyntaxElement, SyntaxKind, SyntaxNode, ast::*};

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

    pub(crate) fn format_right_paren(&self, node: AmberNode) -> Doc<'static> {
        let mut nodes_or_tokens = node.children_with_tokens().rev();
        let docs = if nodes_or_tokens
            .find_map(|node_or_token| match node_or_token {
                NodeOrToken::Token(token) if token.kind() == SyntaxKind::R_PAREN => Some(token),
                _ => None,
            })
            .and_then(|_| {
                nodes_or_tokens
                    .map_while(NodeOrToken::into_token)
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
        SyntaxKind::MODULE_NAME => Some(format_module_name(node.amber())),
        SyntaxKind::NAME => Some(format_name(node.amber())),
        SyntaxKind::NUM_TYPE => Some(format_num_type(node.amber())),
        SyntaxKind::VEC_TYPE => Some(format_vec_type(node.amber())),
        SyntaxKind::REF_TYPE => Some(format_ref_type(node.amber(), ctx)),
        SyntaxKind::HEAP_TYPE => Some(format_heap_type(node.amber())),
        SyntaxKind::PACKED_TYPE => Some(format_packed_type(node.amber())),
        SyntaxKind::FIELD_TYPE => Some(format_field_type(node.amber(), ctx)),
        SyntaxKind::STRUCT_TYPE => Some(format_struct_type(node.amber(), ctx)),
        SyntaxKind::ARRAY_TYPE => Some(format_array_type(node.amber(), ctx)),
        SyntaxKind::FUNC_TYPE => Some(format_func_type(node.amber(), ctx)),
        SyntaxKind::PARAM => Some(format_param(node.amber(), ctx)),
        SyntaxKind::RESULT => Some(format_result(node.amber(), ctx)),
        SyntaxKind::FIELD => Some(format_field(node.amber(), ctx)),
        SyntaxKind::SUB_TYPE => Some(format_sub_type(node.amber(), ctx)),
        SyntaxKind::TABLE_TYPE => Some(format_table_type(node.amber(), ctx)),
        SyntaxKind::MEM_TYPE => Some(format_mem_type(node.amber(), ctx)),
        SyntaxKind::ADDR_TYPE => Some(format_addr_type(node.amber())),
        SyntaxKind::GLOBAL_TYPE => Some(format_global_type(node.amber(), ctx)),
        SyntaxKind::PLAIN_INSTR => Some(format_plain_instr(node.amber(), ctx)),
        SyntaxKind::BLOCK_BLOCK => Some(format_block_block(node.amber(), ctx)),
        SyntaxKind::BLOCK_LOOP => Some(format_block_loop(node.amber(), ctx)),
        SyntaxKind::BLOCK_IF => Some(format_block_if(node.amber(), ctx)),
        SyntaxKind::BLOCK_IF_THEN => Some(format_block_if_then(node.amber(), ctx)),
        SyntaxKind::BLOCK_IF_ELSE => Some(format_block_if_else(node.amber(), ctx)),
        SyntaxKind::BLOCK_TRY_TABLE => Some(format_block_try_table(node.amber(), ctx)),
        SyntaxKind::CATCH => Some(format_catch(node.amber(), ctx)),
        SyntaxKind::CATCH_ALL => Some(format_catch_all(node.amber(), ctx)),
        SyntaxKind::MEM_ARG => Some(format_mem_arg(node.amber())),
        SyntaxKind::IMMEDIATE => Some(format_immediate(node.amber(), ctx)),
        SyntaxKind::TYPE_USE => Some(format_type_use(node.amber(), ctx)),
        SyntaxKind::LIMITS => Some(format_limits(node.amber(), ctx)),
        SyntaxKind::IMPORT => Some(format_import(node.amber(), ctx)),
        SyntaxKind::EXPORT => Some(format_export(node.amber(), ctx)),
        SyntaxKind::EXTERN_TYPE_FUNC => Some(format_extern_type_func(node.amber(), ctx)),
        SyntaxKind::EXTERN_TYPE_TABLE => Some(format_extern_type_table(node.amber(), ctx)),
        SyntaxKind::EXTERN_TYPE_MEMORY => Some(format_extern_type_memory(node.amber(), ctx)),
        SyntaxKind::EXTERN_TYPE_GLOBAL => Some(format_extern_type_global(node.amber(), ctx)),
        SyntaxKind::EXTERN_TYPE_TAG => Some(format_extern_type_tag(node.amber(), ctx)),
        SyntaxKind::EXTERN_IDX_FUNC => Some(format_extern_idx(node.amber(), ctx)),
        SyntaxKind::EXTERN_IDX_TABLE => Some(format_extern_idx(node.amber(), ctx)),
        SyntaxKind::EXTERN_IDX_MEMORY => Some(format_extern_idx(node.amber(), ctx)),
        SyntaxKind::EXTERN_IDX_GLOBAL => Some(format_extern_idx(node.amber(), ctx)),
        SyntaxKind::EXTERN_IDX_TAG => Some(format_extern_idx(node.amber(), ctx)),
        SyntaxKind::INDEX => Some(format_index(node.amber())),
        SyntaxKind::LOCAL => Some(format_local(node.amber(), ctx)),
        SyntaxKind::MEM_PAGE_SIZE => Some(format_mem_page_size(node.amber(), ctx)),
        SyntaxKind::MEM_USE => Some(format_mem_use(node.amber(), ctx)),
        SyntaxKind::OFFSET => Some(format_offset(node.amber(), ctx)),
        SyntaxKind::ELEM => Some(format_elem(node.amber(), ctx)),
        SyntaxKind::ELEM_LIST => Some(format_elem_list(node.amber(), ctx)),
        SyntaxKind::ELEM_EXPR => Some(format_elem_expr(node.amber(), ctx)),
        SyntaxKind::TABLE_USE => Some(format_table_use(node.amber(), ctx)),
        SyntaxKind::DATA => Some(format_data(node.amber(), ctx)),
        SyntaxKind::MODULE => Module::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::MODULE_FIELD_DATA => Some(format_module_field_data(node.amber(), ctx)),
        SyntaxKind::MODULE_FIELD_ELEM => Some(format_module_field_elem(node.amber(), ctx)),
        SyntaxKind::MODULE_FIELD_EXPORT => Some(format_module_field_export(node.amber(), ctx)),
        SyntaxKind::MODULE_FIELD_FUNC => Some(format_module_field_func(node.amber(), ctx)),
        SyntaxKind::MODULE_FIELD_GLOBAL => Some(format_module_field_global(node.amber(), ctx)),
        SyntaxKind::MODULE_FIELD_IMPORT => Some(format_module_field_import(node.amber(), ctx)),
        SyntaxKind::MODULE_FIELD_MEMORY => Some(format_module_field_memory(node.amber(), ctx)),
        SyntaxKind::MODULE_FIELD_START => Some(format_module_field_start(node.amber(), ctx)),
        SyntaxKind::MODULE_FIELD_TABLE => Some(format_module_field_table(node.amber(), ctx)),
        SyntaxKind::MODULE_FIELD_TAG => Some(format_module_field_tag(node.amber(), ctx)),
        SyntaxKind::TYPE_DEF => Some(format_type_def(node.amber(), ctx)),
        SyntaxKind::REC_TYPE => Some(format_rec_type(node.amber(), ctx)),
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
                        if token.has_prev_sibling_or_token() && children.peek().is_some() {
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

fn format_trivias_after_node(node: AmberNode, parent: AmberNode, ctx: &Ctx) -> Vec<Doc<'static>> {
    let mut tokens = parent
        .children_with_tokens()
        .skip_while(|node_or_token| node_or_token.text_range().start() <= node.text_range().start())
        .map_while(NodeOrToken::into_token)
        .peekable();
    let mut trivias = Vec::with_capacity(1);
    while let Some(token) = tokens.next() {
        match token.kind() {
            SyntaxKind::LINE_COMMENT
            | SyntaxKind::BLOCK_COMMENT
            | SyntaxKind::ERROR
            | SyntaxKind::ANNOT_START
            | SyntaxKind::ANNOT_ELEM
            | SyntaxKind::ANNOT_END => trivias.push(token),
            SyntaxKind::WHITESPACE
                if tokens.peek().is_none_or(|token| match token.kind() {
                    SyntaxKind::R_PAREN => false,
                    SyntaxKind::KEYWORD => token.text() != "end",
                    _ => true,
                }) =>
            {
                trivias.push(token);
            }
            _ => break,
        }
    }
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
            docs.push(Doc::text(token.text().to_string()));
        }
        _ => {}
    });
    docs
}
fn format_trivias_after_token(token: AmberToken, parent: AmberNode, ctx: &Ctx) -> Vec<Doc<'static>> {
    let mut tokens = parent
        .children_with_tokens()
        .skip_while(|node_or_token| node_or_token.text_range().start() <= token.text_range().start())
        .map_while(NodeOrToken::into_token)
        .peekable();
    let mut trivias = Vec::with_capacity(1);
    while let Some(current) = tokens.next() {
        match current.kind() {
            SyntaxKind::LINE_COMMENT
            | SyntaxKind::BLOCK_COMMENT
            | SyntaxKind::ERROR
            | SyntaxKind::ANNOT_START
            | SyntaxKind::ANNOT_ELEM
            | SyntaxKind::ANNOT_END => trivias.push(current),
            SyntaxKind::WHITESPACE
                if tokens.peek().is_none_or(|token| match token.kind() {
                    SyntaxKind::R_PAREN => false,
                    SyntaxKind::KEYWORD => token.text() != "end",
                    _ => true,
                }) =>
            {
                if !(token.kind() == SyntaxKind::L_PAREN
                    && current.kind() == SyntaxKind::WHITESPACE
                    && trivias.is_empty())
                {
                    trivias.push(current);
                }
            }
            _ => break,
        }
    }
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
            docs.push(Doc::text(token.text().to_string()));
        }
        _ => {}
    });
    docs
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
    node.prev_consecutive_tokens()
        .nth(1)
        .or_else(|| {
            // for the case that comment comes at the start or the end of a list of nodes
            node.parent()
                .and_then(|parent| parent.prev_sibling_or_token())
                .and_then(|parent| parent.prev_sibling_or_token())
                .and_then(NodeOrToken::into_token)
        })
        .as_ref()
        .and_then(|token| {
            if token.kind() == SyntaxKind::LINE_COMMENT {
                token
                    .text()
                    .strip_prefix(";;")
                    .and_then(|s| s.trim_start().strip_prefix(&ctx.options.ignore_comment_directive))
            } else {
                None
            }
        })
        .is_some_and(|rest| rest.is_empty() || rest.starts_with(|c: char| c.is_ascii_whitespace()))
}

fn wrap_before<I>(children: &mut iter::Peekable<I>, option: WrapBefore) -> Doc<'static>
where
    I: Iterator,
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

fn whitespace_of_multi_line(option: MultiLine, first: Option<AmberNode>, parent: AmberNode) -> Doc<'static> {
    match option {
        MultiLine::Never => Doc::space(),
        MultiLine::Overflow => Doc::line_or_space(),
        MultiLine::Smart => {
            if first.is_some_and(|first| {
                parent
                    .children_with_tokens()
                    .skip_while(|node_or_token| node_or_token.text_range().start() <= first.text_range().start())
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
