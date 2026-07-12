use self::lexer::Lexer;
use crate::error::SyntaxError;
use wat_syntax::{GreenNode, GreenToken, NodeOrToken, SyntaxKind};

mod builder;
mod green;
mod helpers;
mod instr;
mod lexer;
mod module;
mod ty;

/// Parse the code into green node.
pub fn parse(source: &str) -> (GreenNode, Vec<SyntaxError>) {
    let mut parser = Parser::new(source);
    (parser.parse_root(), parser.errors)
}

#[inline]
/// Parse the given code snippet as the specified syntax kind of node.
///
/// It will return `None` if such syntax kind isn't supported or the code doesn't reflect corresponding syntax kind.
///
/// Note that there can't be leading whitespaces or comments after that offset.
///
/// ## Examples
///
/// ```
/// use wat_parser::parse_as;
/// use wat_syntax::SyntaxKind;
/// assert!(parse_as(SyntaxKind::MODULE_FIELD_FUNC, "(module (func))").is_none());
/// assert!(parse_as(SyntaxKind::MODULE_FIELD_FUNC, "(fun)").is_none());
/// let (green, errors) = parse_as(SyntaxKind::MODULE_FIELD_FUNC, "(func ())").unwrap();
/// assert_eq!(green.kind(), SyntaxKind::MODULE_FIELD_FUNC);
/// assert!(!errors.is_empty());
/// ```
pub fn parse_as(kind: SyntaxKind, source: &str) -> Option<(GreenNode, Vec<SyntaxError>)> {
    let mut parser = Parser::new(source);
    let green = match kind {
        SyntaxKind::MODULE_NAME => parser.parse_module_name(),
        SyntaxKind::NAME => parser.parse_name(),
        SyntaxKind::NUM_TYPE | SyntaxKind::VEC_TYPE => None,
        SyntaxKind::REF_TYPE => parser.parse_ref_type(),
        SyntaxKind::HEAP_TYPE | SyntaxKind::PACKED_TYPE => None,
        SyntaxKind::FIELD_TYPE => parser.parse_field_type(),
        SyntaxKind::STRUCT_TYPE | SyntaxKind::ARRAY_TYPE | SyntaxKind::FUNC_TYPE | SyntaxKind::CONT_TYPE => {
            parser.parse_composite_type()
        }
        SyntaxKind::PARAM => parser.parse_param(),
        SyntaxKind::RESULT => parser.parse_result(),
        SyntaxKind::FIELD => parser.parse_field(),
        SyntaxKind::SUB_TYPE => parser.parse_sub_type(),
        SyntaxKind::TABLE_TYPE => parser.parse_table_type(),
        SyntaxKind::MEM_TYPE => parser.parse_mem_type(),
        SyntaxKind::ADDR_TYPE => parser.parse_addr_type(),
        SyntaxKind::GLOBAL_TYPE => parser.parse_global_type(),
        SyntaxKind::PLAIN_INSTR
        | SyntaxKind::BLOCK_BLOCK
        | SyntaxKind::BLOCK_LOOP
        | SyntaxKind::BLOCK_IF
        | SyntaxKind::BLOCK_TRY_TABLE => parser.parse_instr(),
        SyntaxKind::BLOCK_IF_THEN | SyntaxKind::BLOCK_IF_ELSE => None,
        SyntaxKind::CATCH | SyntaxKind::CATCH_ALL => parser.parse_catch(),
        SyntaxKind::MEM_ARG => parser.parse_mem_arg(),
        SyntaxKind::ON_CLAUSE => parser.parse_on_clause(),
        SyntaxKind::IMMEDIATE => parser.parse_immediate(),
        SyntaxKind::TYPE_USE => parser.parse_type_use(),
        SyntaxKind::LIMITS => parser.parse_limits(),
        SyntaxKind::IMPORT | SyntaxKind::EXPORT | SyntaxKind::IMPORT_ITEM => None,
        SyntaxKind::EXTERN_TYPE_FUNC
        | SyntaxKind::EXTERN_TYPE_TABLE
        | SyntaxKind::EXTERN_TYPE_MEMORY
        | SyntaxKind::EXTERN_TYPE_GLOBAL
        | SyntaxKind::EXTERN_TYPE_TAG => None,
        SyntaxKind::EXTERN_IDX_FUNC
        | SyntaxKind::EXTERN_IDX_TABLE
        | SyntaxKind::EXTERN_IDX_MEMORY
        | SyntaxKind::EXTERN_IDX_GLOBAL
        | SyntaxKind::EXTERN_IDX_TAG => parser.parse_extern_idx(),
        SyntaxKind::INDEX => parser.parse_index(),
        SyntaxKind::LOCAL => parser.parse_local(),
        SyntaxKind::MEM_PAGE_SIZE => parser.parse_mem_page_size(),
        SyntaxKind::MEM_USE => parser.parse_mem_use(),
        SyntaxKind::OFFSET => parser.parse_offset(),
        SyntaxKind::ELEM => parser.parse_elem(),
        SyntaxKind::ELEM_LIST => parser.parse_elem_list(),
        SyntaxKind::ELEM_EXPR => parser.parse_elem_expr(),
        SyntaxKind::TABLE_USE => parser.parse_table_use(),
        SyntaxKind::DATA => parser.parse_data(),
        SyntaxKind::MODULE => parser.parse_module(),
        SyntaxKind::MODULE_FIELD_DATA
        | SyntaxKind::MODULE_FIELD_ELEM
        | SyntaxKind::MODULE_FIELD_EXPORT
        | SyntaxKind::MODULE_FIELD_FUNC
        | SyntaxKind::MODULE_FIELD_GLOBAL
        | SyntaxKind::MODULE_FIELD_IMPORT
        | SyntaxKind::MODULE_FIELD_MEMORY
        | SyntaxKind::MODULE_FIELD_START
        | SyntaxKind::MODULE_FIELD_TABLE
        | SyntaxKind::MODULE_FIELD_TAG
        | SyntaxKind::TYPE_DEF
        | SyntaxKind::REC_TYPE => parser.parse_module_field(),
        SyntaxKind::ROOT => Some(parser.parse_root()),
        SyntaxKind::WHITESPACE
        | SyntaxKind::LINE_COMMENT
        | SyntaxKind::BLOCK_COMMENT
        | SyntaxKind::L_PAREN
        | SyntaxKind::R_PAREN
        | SyntaxKind::KEYWORD
        | SyntaxKind::INSTR_NAME
        | SyntaxKind::IDENT
        | SyntaxKind::STRING
        | SyntaxKind::INT
        | SyntaxKind::UNSIGNED_INT
        | SyntaxKind::FLOAT
        | SyntaxKind::TYPE_KEYWORD
        | SyntaxKind::MODIFIER_KEYWORD
        | SyntaxKind::EQ
        | SyntaxKind::MEM_ARG_KEYWORD
        | SyntaxKind::SHAPE_DESCRIPTOR
        | SyntaxKind::ANNOT_START
        | SyntaxKind::ANNOT_ELEM
        | SyntaxKind::ANNOT_END
        | SyntaxKind::ERROR => None,
    };
    green.map(|green| (green, parser.errors))
}

#[inline]
/// Checks if a character is a valid identifier character.
///
/// ## Examples
///
/// ```
/// # use wat_parser::is_id_char;
/// assert!(is_id_char('a'));
/// assert!(is_id_char('Z'));
/// assert!(is_id_char('0'));
/// assert!(is_id_char('$'));
/// assert!(is_id_char('.'));
/// assert!(!is_id_char('('));
/// assert!(!is_id_char(')'));
/// ```
pub fn is_id_char(c: char) -> bool {
    c.is_ascii_alphanumeric()
        || c.is_ascii_punctuation() && !matches!(c, '"' | ',' | ';' | '(' | ')' | '[' | ']' | '{' | '}')
}

type GreenElement = NodeOrToken<GreenNode, GreenToken>;

fn node<I>(kind: SyntaxKind, children: I) -> GreenNode
where
    I: IntoIterator<Item = GreenElement>,
    I::IntoIter: ExactSizeIterator,
{
    GreenNode::new(kind, children)
}

struct Parser<'s> {
    source: &'s str,
    lexer: Lexer<'s>,
    errors: Vec<SyntaxError>,
    elements: Vec<GreenElement>,
}

impl<'s> Parser<'s> {
    fn new(source: &'s str) -> Self {
        Parser {
            source,
            lexer: Lexer::new(source),
            errors: Vec::new(),
            elements: Vec::new(),
        }
    }

    fn parse_root(&mut self) -> GreenNode {
        let mark = self.start_node();
        while self.recover(Self::parse_module) {}
        self.parse_trivias();
        self.finish_node(SyntaxKind::ROOT, mark)
    }
}
