use crate::{
    LanguageService,
    binder::{SymbolKey, SymbolKind, SymbolTable},
    helpers, types_analyzer,
};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticSeverity, Union2};
use rowan::{
    TextRange,
    ast::{AstNode, support},
};
use rustc_hash::FxHashMap;
use wat_syntax::{
    SyntaxKind, SyntaxNode,
    ast::{BlockInstr, Instr},
};

const DIAGNOSTIC_CODE: &str = "uninit";

pub fn check(
    service: &LanguageService,
    diagnostics: &mut Vec<Diagnostic>,
    line_index: &LineIndex,
    symbol_table: &SymbolTable,
    node: &SyntaxNode,
) {
    let region = SymbolKey::new(node);
    let locals = symbol_table
        .symbols
        .iter()
        .filter(|symbol| symbol.kind == SymbolKind::Local && symbol.region == region)
        .map(|symbol| {
            let ty = types_analyzer::extract_type(service, symbol.green.clone());
            (
                symbol.key,
                if let Some(false) = ty.map(|ty| ty.defaultable()) {
                    Init::Unset
                } else {
                    Init::Set
                },
            )
        })
        .collect::<FxHashMap<_, _>>();
    Checker {
        service,
        diagnostics,
        line_index,
        symbol_table,
        locals,
    }
    .check(node);
}

struct Checker<'a, 'db> {
    service: &'a LanguageService,
    diagnostics: &'a mut Vec<Diagnostic>,
    line_index: &'a LineIndex,
    symbol_table: &'a SymbolTable<'db>,
    locals: FxHashMap<SymbolKey, Init>,
}
impl Checker<'_, '_> {
    fn check(&mut self, node: &SyntaxNode) {
        let conditional = matches!(
            node.kind(),
            SyntaxKind::BLOCK_IF_THEN | SyntaxKind::BLOCK_IF_ELSE
        );
        support::children::<Instr>(node).for_each(|instr| match instr {
            Instr::Plain(plain_instr) => {
                self.check(plain_instr.syntax());

                let Some(token) = plain_instr.instr_name() else {
                    return;
                };
                match token.text() {
                    "local.get" => {
                        let Some(immediate) = plain_instr.immediates().next() else {
                            return;
                        };
                        let immediate = immediate.syntax();
                        let Some(def_symbol) = self
                            .symbol_table
                            .find_param_or_local_def(SymbolKey::new(immediate))
                        else {
                            return;
                        };
                        let set = match self.locals.get(&def_symbol.key) {
                            Some(Init::Set) | None => true,
                            Some(Init::Conditional(range)) => {
                                range.contains_range(immediate.text_range())
                            }
                            Some(Init::Unset) => false,
                        };
                        if !set {
                            self.diagnostics.push(Diagnostic {
                                range: helpers::rowan_range_to_lsp_range(
                                    self.line_index,
                                    immediate.text_range(),
                                ),
                                severity: Some(DiagnosticSeverity::Error),
                                source: Some("wat".into()),
                                code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                                message: format!(
                                    "local `{}` is used before being initialized",
                                    def_symbol.idx.render(self.service)
                                ),
                                ..Default::default()
                            });
                        }
                    }
                    "local.set" | "local.tee" => {
                        if let Some(initialized) = plain_instr
                            .immediates()
                            .next()
                            .and_then(|immediate| {
                                self.symbol_table
                                    .find_param_or_local_def(SymbolKey::new(immediate.syntax()))
                            })
                            .and_then(|symbol| self.locals.get_mut(&symbol.key))
                        {
                            if conditional && matches!(initialized, Init::Unset) {
                                *initialized = Init::Conditional(node.text_range());
                            } else {
                                *initialized = Init::Set;
                            }
                        }
                    }
                    _ => {}
                }
            }
            Instr::Block(BlockInstr::Block(block_block)) => {
                self.check(block_block.syntax());
            }
            Instr::Block(BlockInstr::Loop(block_loop)) => {
                self.check(block_loop.syntax());
            }
            Instr::Block(BlockInstr::If(block_if)) => {
                self.check(block_if.syntax());
                if let Some(then_block) = block_if.then_block() {
                    self.check(then_block.syntax());
                }
                if let Some(else_block) = block_if.else_block() {
                    self.check(else_block.syntax());
                }
            }
        });
    }
}

enum Init {
    Unset,
    Conditional(TextRange),
    Set,
}
