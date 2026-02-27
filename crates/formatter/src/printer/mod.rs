use self::{instr::*, module::*, ty::*};
use crate::config::{FormatOptions, LanguageOptions, MultiLine, WrapBefore};
use std::iter;
use tiny_pretty::Doc;
use wat_syntax::{AmberNode, AmberToken, NodeOrToken, SyntaxKind, ast::*};

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

pub(crate) fn format_node<'a>(node: AmberNode<'a>, ctx: &Ctx) -> Option<Doc<'a>> {
    match node.kind() {
        SyntaxKind::MODULE_NAME => Some(format_module_name(node)),
        SyntaxKind::NAME => Some(format_name(node)),
        SyntaxKind::NUM_TYPE => Some(format_num_type(node)),
        SyntaxKind::VEC_TYPE => Some(format_vec_type(node)),
        SyntaxKind::REF_TYPE => Some(format_ref_type(node, ctx)),
        SyntaxKind::HEAP_TYPE => Some(format_heap_type(node)),
        SyntaxKind::PACKED_TYPE => Some(format_packed_type(node)),
        SyntaxKind::FIELD_TYPE => Some(format_field_type(node, ctx)),
        SyntaxKind::STRUCT_TYPE => Some(format_struct_type(node, ctx)),
        SyntaxKind::ARRAY_TYPE => Some(format_array_type(node, ctx)),
        SyntaxKind::FUNC_TYPE => Some(format_func_type(node, ctx)),
        SyntaxKind::CONT_TYPE => Some(format_cont_type(node, ctx)),
        SyntaxKind::PARAM => Some(format_param(node, ctx)),
        SyntaxKind::RESULT => Some(format_result(node, ctx)),
        SyntaxKind::FIELD => Some(format_field(node, ctx)),
        SyntaxKind::SUB_TYPE => Some(format_sub_type(node, ctx)),
        SyntaxKind::TABLE_TYPE => Some(format_table_type(node, ctx)),
        SyntaxKind::MEM_TYPE => Some(format_mem_type(node, ctx)),
        SyntaxKind::ADDR_TYPE => Some(format_addr_type(node)),
        SyntaxKind::GLOBAL_TYPE => Some(format_global_type(node, ctx)),
        SyntaxKind::PLAIN_INSTR => Some(format_plain_instr(node, ctx)),
        SyntaxKind::BLOCK_BLOCK => Some(format_block_block(node, ctx)),
        SyntaxKind::BLOCK_LOOP => Some(format_block_loop(node, ctx)),
        SyntaxKind::BLOCK_IF => Some(format_block_if(node, ctx)),
        SyntaxKind::BLOCK_IF_THEN => Some(format_block_if_then(node, ctx)),
        SyntaxKind::BLOCK_IF_ELSE => Some(format_block_if_else(node, ctx)),
        SyntaxKind::BLOCK_TRY_TABLE => Some(format_block_try_table(node, ctx)),
        SyntaxKind::CATCH => Some(format_catch(node, ctx)),
        SyntaxKind::CATCH_ALL => Some(format_catch_all(node, ctx)),
        SyntaxKind::MEM_ARG => Some(format_mem_arg(node)),
        SyntaxKind::ON_CLAUSE => Some(format_on_clause(node, ctx)),
        SyntaxKind::IMMEDIATE => Some(format_immediate(node, ctx)),
        SyntaxKind::TYPE_USE => Some(format_type_use(node, ctx)),
        SyntaxKind::LIMITS => Some(format_limits(node, ctx)),
        SyntaxKind::IMPORT => Some(format_import(node, ctx)),
        SyntaxKind::EXPORT => Some(format_export(node, ctx)),
        SyntaxKind::EXTERN_TYPE_FUNC => Some(format_extern_type_func(node, ctx)),
        SyntaxKind::EXTERN_TYPE_TABLE => Some(format_extern_type_table(node, ctx)),
        SyntaxKind::EXTERN_TYPE_MEMORY => Some(format_extern_type_memory(node, ctx)),
        SyntaxKind::EXTERN_TYPE_GLOBAL => Some(format_extern_type_global(node, ctx)),
        SyntaxKind::EXTERN_TYPE_TAG => Some(format_extern_type_tag(node, ctx)),
        SyntaxKind::EXTERN_IDX_FUNC => Some(format_extern_idx(node, ctx)),
        SyntaxKind::EXTERN_IDX_TABLE => Some(format_extern_idx(node, ctx)),
        SyntaxKind::EXTERN_IDX_MEMORY => Some(format_extern_idx(node, ctx)),
        SyntaxKind::EXTERN_IDX_GLOBAL => Some(format_extern_idx(node, ctx)),
        SyntaxKind::EXTERN_IDX_TAG => Some(format_extern_idx(node, ctx)),
        SyntaxKind::INDEX => Some(format_index(node)),
        SyntaxKind::LOCAL => Some(format_local(node, ctx)),
        SyntaxKind::MEM_PAGE_SIZE => Some(format_mem_page_size(node, ctx)),
        SyntaxKind::MEM_USE => Some(format_mem_use(node, ctx)),
        SyntaxKind::OFFSET => Some(format_offset(node, ctx)),
        SyntaxKind::ELEM => Some(format_elem(node, ctx)),
        SyntaxKind::ELEM_LIST => Some(format_elem_list(node, ctx)),
        SyntaxKind::ELEM_EXPR => Some(format_elem_expr(node, ctx)),
        SyntaxKind::TABLE_USE => Some(format_table_use(node, ctx)),
        SyntaxKind::DATA => Some(format_data(node, ctx)),
        SyntaxKind::MODULE => Some(format_module(node, ctx)),
        SyntaxKind::MODULE_FIELD_DATA => Some(format_module_field_data(node, ctx)),
        SyntaxKind::MODULE_FIELD_ELEM => Some(format_module_field_elem(node, ctx)),
        SyntaxKind::MODULE_FIELD_EXPORT => Some(format_module_field_export(node, ctx)),
        SyntaxKind::MODULE_FIELD_FUNC => Some(format_module_field_func(node, ctx)),
        SyntaxKind::MODULE_FIELD_GLOBAL => Some(format_module_field_global(node, ctx)),
        SyntaxKind::MODULE_FIELD_IMPORT => Some(format_module_field_import(node, ctx)),
        SyntaxKind::MODULE_FIELD_MEMORY => Some(format_module_field_memory(node, ctx)),
        SyntaxKind::MODULE_FIELD_START => Some(format_module_field_start(node, ctx)),
        SyntaxKind::MODULE_FIELD_TABLE => Some(format_module_field_table(node, ctx)),
        SyntaxKind::MODULE_FIELD_TAG => Some(format_module_field_tag(node, ctx)),
        SyntaxKind::TYPE_DEF => Some(format_type_def(node, ctx)),
        SyntaxKind::REC_TYPE => Some(format_rec_type(node, ctx)),
        SyntaxKind::ROOT => Some(format_root(node, ctx)),
        _ => None,
    }
}

pub(crate) fn format_root<'a>(root: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    let mut docs = Vec::with_capacity(2);

    let mut nodes_or_tokens = root.children_with_tokens().enumerate().peekable();
    let mut prev_kind = SyntaxKind::WHITESPACE;
    while let Some((index, node_or_token)) = nodes_or_tokens.next() {
        let kind = node_or_token.kind();
        match node_or_token {
            NodeOrToken::Node(module) => {
                if should_ignore(module, root, ctx) {
                    reflow(module.green().to_string(), &mut docs);
                } else {
                    docs.push(format_module(module, ctx));
                }
            }
            NodeOrToken::Token(token) => match kind {
                SyntaxKind::LINE_COMMENT => {
                    docs.push(format_line_comment(token.text(), ctx));
                }
                SyntaxKind::BLOCK_COMMENT => {
                    docs.push(format_block_comment(token.text(), ctx));
                }
                SyntaxKind::WHITESPACE => {
                    if index > 0 && nodes_or_tokens.peek().is_some() {
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
                _ => docs.push(Doc::text(token.text())),
            },
        }
        prev_kind = kind;
    }

    docs.push(Doc::hard_line());
    Doc::list(docs)
}

fn format_trivias_after_node<'a>(node: AmberNode<'a>, parent: AmberNode<'a>, ctx: &Ctx) -> Vec<Doc<'a>> {
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
            docs.push(Doc::text(token.text()));
        }
        _ => {}
    });
    docs
}
fn format_trivias_after_token<'a>(token: AmberToken<'a>, parent: AmberNode<'a>, ctx: &Ctx) -> Vec<Doc<'a>> {
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
            docs.push(Doc::text(token.text()));
        }
        _ => {}
    });
    docs
}

fn format_line_comment<'a>(text: &'a str, ctx: &Ctx) -> Doc<'a> {
    if ctx.options.format_comments {
        let content = text.strip_prefix(";;").expect("line comment must start with `;;`");
        if content.is_empty() || content.starts_with([' ', '\t']) {
            Doc::text(text)
        } else {
            Doc::text(format!(";; {content}"))
        }
    } else {
        Doc::text(text)
    }
}

fn format_block_comment<'a>(text: &'a str, ctx: &Ctx) -> Doc<'a> {
    if ctx.options.format_comments {
        let content = text
            .strip_prefix("(;")
            .and_then(|s| s.strip_suffix(";)"))
            .expect("block comment must be wrapped between `(;` and `;)`");
        let has_leading_ws = content.starts_with([' ', '\t']);
        let has_trailing_ws = content.ends_with([' ', '\t']);
        if content.is_empty() || has_leading_ws && has_trailing_ws {
            Doc::text(text)
        } else if has_leading_ws {
            Doc::text(format!("(;{content} ;)"))
        } else if has_trailing_ws {
            Doc::text(format!("(; {content};)"))
        } else {
            Doc::text(format!("(; {content} ;)"))
        }
    } else {
        Doc::text(text)
    }
}

fn reflow(text: String, docs: &mut Vec<Doc>) {
    let mut lines = text.lines();
    if let Some(line) = lines.next() {
        docs.push(Doc::text(line.to_owned()));
    }
    for line in lines {
        docs.push(Doc::empty_line());
        docs.push(Doc::text(line.to_owned()));
    }
}

fn should_ignore(node: AmberNode, parent: AmberNode, ctx: &Ctx) -> bool {
    parent
        .children_with_tokens()
        .rev()
        .skip_while(|node_or_token| node_or_token.text_range().start() >= node.text_range().start())
        .map_while(NodeOrToken::into_token)
        .nth(1)
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
