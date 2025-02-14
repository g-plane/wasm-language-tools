use crate::config::{FormatOptions, LanguageOptions};
use rowan::{ast::AstNode, Direction};
use tiny_pretty::Doc;
use wat_syntax::{ast::*, SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken, WatLanguage};

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
}

pub(crate) fn format_node(node: SyntaxNode, ctx: &Ctx) -> Option<Doc<'static>> {
    match node.kind() {
        SyntaxKind::MODULE_NAME => ModuleName::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::NAME => Name::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::NUM_TYPE => NumType::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::VEC_TYPE => VecType::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::REF_TYPE => RefType::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::FUNC_TYPE => FuncType::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::PARAM => Param::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::RESULT => Result::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::TABLE_TYPE => TableType::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::MEMORY_TYPE => MemoryType::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::GLOBAL_TYPE => GlobalType::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::BLOCK_TYPE => BlockType::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::PLAIN_INSTR => PlainInstr::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::BLOCK_BLOCK => BlockBlock::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::BLOCK_LOOP => BlockLoop::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::BLOCK_IF => BlockIf::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::BLOCK_IF_THEN => BlockIfThen::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::BLOCK_IF_ELSE => BlockIfElse::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::IMMEDIATE => Immediate::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::TYPE_USE => TypeUse::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::LIMITS => Limits::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::IMPORT => Import::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::EXPORT => Export::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::IMPORT_DESC_TYPE_USE => ImportDescTypeUse::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::IMPORT_DESC_TABLE_TYPE => {
            ImportDescTableType::cast(node).map(|node| node.doc(ctx))
        }
        SyntaxKind::IMPORT_DESC_MEMORY_TYPE => {
            ImportDescMemoryType::cast(node).map(|node| node.doc(ctx))
        }
        SyntaxKind::IMPORT_DESC_GLOBAL_TYPE => {
            ImportDescGlobalType::cast(node).map(|node| node.doc(ctx))
        }
        SyntaxKind::EXPORT_DESC_FUNC => ExportDescFunc::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::EXPORT_DESC_TABLE => ExportDescTable::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::EXPORT_DESC_MEMORY => ExportDescMemory::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::EXPORT_DESC_GLOBAL => ExportDescGlobal::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::INDEX => Index::cast(node).map(|node| node.doc(ctx)),
        SyntaxKind::LOCAL => Local::cast(node).map(|node| node.doc(ctx)),
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
        SyntaxKind::MODULE_FIELD_TYPE => ModuleFieldType::cast(node).map(|node| node.doc(ctx)),
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
        .map_while(|element| match element {
            SyntaxElement::Token(token)
                if matches!(
                    token.kind(),
                    SyntaxKind::LINE_COMMENT
                        | SyntaxKind::BLOCK_COMMENT
                        | SyntaxKind::WHITESPACE
                        | SyntaxKind::ERROR
                ) =>
            {
                Some(token)
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    if trivias.iter().all(|token| {
        token.kind() == SyntaxKind::WHITESPACE
            && token.text().chars().filter(|c| *c == '\n').count() < 2
    }) {
        return vec![];
    }
    let mut docs = Vec::with_capacity(3);
    if trivias
        .first()
        .is_some_and(|token| token.kind().is_comment())
    {
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
        SyntaxKind::ERROR => {
            docs.push(Doc::text(token.to_string()));
        }
        _ => {}
    });
    docs
}
fn format_trivias_after_token(token: SyntaxToken, ctx: &Ctx) -> Vec<Doc<'static>> {
    let respect_first_whitespace = token
        .siblings_with_tokens(Direction::Next)
        .skip(1)
        .find(|element| {
            !matches!(
                element.kind(),
                SyntaxKind::WHITESPACE | SyntaxKind::LINE_COMMENT | SyntaxKind::BLOCK_COMMENT
            )
        })
        .is_some_and(|element| {
            matches!(
                element.kind(),
                SyntaxKind::MODULE_FIELD_DATA
                    | SyntaxKind::MODULE_FIELD_ELEM
                    | SyntaxKind::MODULE_FIELD_EXPORT
                    | SyntaxKind::MODULE_FIELD_FUNC
                    | SyntaxKind::MODULE_FIELD_GLOBAL
                    | SyntaxKind::MODULE_FIELD_IMPORT
                    | SyntaxKind::MODULE_FIELD_MEMORY
                    | SyntaxKind::MODULE_FIELD_START
                    | SyntaxKind::MODULE_FIELD_TABLE
                    | SyntaxKind::MODULE_FIELD_TYPE
                    | SyntaxKind::PLAIN_INSTR
                    | SyntaxKind::BLOCK_BLOCK
                    | SyntaxKind::BLOCK_IF
                    | SyntaxKind::BLOCK_LOOP
            )
        });
    let trivias = token
        .siblings_with_tokens(Direction::Next)
        .skip(1)
        .map_while(|element| match element {
            SyntaxElement::Token(token)
                if matches!(
                    token.kind(),
                    SyntaxKind::LINE_COMMENT
                        | SyntaxKind::BLOCK_COMMENT
                        | SyntaxKind::WHITESPACE
                        | SyntaxKind::ERROR
                ) =>
            {
                Some(token)
            }
            _ => None,
        })
        .skip_while(|token| !respect_first_whitespace && token.kind() == SyntaxKind::WHITESPACE)
        .collect::<Vec<_>>();
    if trivias
        .iter()
        .all(|token| token.kind() == SyntaxKind::WHITESPACE)
    {
        return vec![];
    }
    let mut docs = Vec::with_capacity(3);
    if !respect_first_whitespace {
        docs.push(Doc::space());
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
        SyntaxKind::ERROR => {
            docs.push(Doc::text(token.to_string()));
        }
        _ => {}
    });
    docs
}

fn format_line_comment(text: &str, ctx: &Ctx) -> Doc<'static> {
    if ctx.options.format_comments {
        let content = text
            .strip_prefix(";;")
            .expect("line comment must start with `;;`");
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
            SyntaxElement::Token(token) if token.kind() == SyntaxKind::LINE_COMMENT => {
                token.text().strip_prefix(";;").and_then(|s| {
                    s.trim_start()
                        .strip_prefix(&ctx.options.ignore_comment_directive)
                })
            }
            _ => None,
        })
        .is_some_and(|rest| rest.is_empty() || rest.starts_with(|c: char| c.is_ascii_whitespace()))
}

fn has_line_break_after_token(token: &SyntaxToken) -> bool {
    token
        .siblings_with_tokens(Direction::Next)
        .skip(1)
        .map_while(SyntaxElement::into_token)
        .any(|token| token.text().contains('\n'))
}
