use super::find_meaningful_token;
use crate::{
    binder::{SymbolItemKind, SymbolTablesCtx},
    files::FilesCtx,
    helpers,
    types_analyzer::TypesAnalyzerCtx,
    LanguageService,
};
use lsp_types::{Hover, HoverContents, HoverParams, LanguageString, MarkedString};
use rowan::ast::{support::child, AstNode};
use wat_syntax::ast::GlobalType;

impl LanguageService {
    pub fn hover(&self, params: HoverParams) -> Option<Hover> {
        let uri = self.ctx.uri(
            params
                .text_document_position_params
                .text_document
                .uri
                .clone(),
        );
        let token = find_meaningful_token(
            &self.ctx,
            uri,
            params.text_document_position_params.position,
        )?;
        let parent = token.parent()?;

        let line_index = self.ctx.line_index(uri);
        let symbol_table = self.ctx.symbol_table(uri);

        let key = parent.into();
        symbol_table
            .find_param_def(&key)
            .or_else(|| symbol_table.find_local_def(&key))
            .map(|symbol| {
                let mut content_value = '('.to_string();
                match &symbol.kind {
                    SymbolItemKind::Param(idx) => {
                        content_value.push_str("param");
                        if let Some(name) = &idx.name {
                            content_value.push(' ');
                            content_value.push_str(name);
                        }
                    }
                    SymbolItemKind::Local(idx) => {
                        content_value.push_str("local");
                        if let Some(name) = &idx.name {
                            content_value.push(' ');
                            content_value.push_str(name);
                        }
                    }
                    _ => {}
                }
                self.ctx
                    .extract_types(symbol.key.green.clone())
                    .into_iter()
                    .for_each(|ty| {
                        content_value.push(' ');
                        content_value.push_str(&ty.to_string());
                    });
                content_value.push(')');
                Hover {
                    contents: HoverContents::Scalar(MarkedString::LanguageString(LanguageString {
                        language: "wat".into(),
                        value: content_value,
                    })),
                    range: Some(helpers::rowan_range_to_lsp_range(
                        &line_index,
                        token.text_range(),
                    )),
                }
            })
            .or_else(|| {
                symbol_table.find_global_defs(&key).map(|symbols| {
                    let root = self.ctx.root(uri);
                    Hover {
                        contents: HoverContents::Array(
                            symbols
                                .map(|symbol| {
                                    let mut content_value = '('.to_string();
                                    if let SymbolItemKind::GlobalDef(idx) = &symbol.kind {
                                        content_value.push_str("global");
                                        if let Some(name) = &idx.name {
                                            content_value.push(' ');
                                            content_value.push_str(name);
                                        }
                                    }
                                    let node = symbol.key.ptr.to_node(&root);
                                    if let Some(global_type) = child::<GlobalType>(&node) {
                                        let mutable = global_type.mut_keyword().is_some();
                                        if mutable {
                                            content_value.push_str(" (mut");
                                        }
                                        if let Some(val_type) = global_type.val_type() {
                                            content_value.push(' ');
                                            content_value.push_str(&val_type.syntax().to_string());
                                        }
                                        if mutable {
                                            content_value.push(')');
                                        }
                                    }
                                    content_value.push(')');
                                    MarkedString::LanguageString(LanguageString {
                                        language: "wat".into(),
                                        value: content_value,
                                    })
                                })
                                .collect(),
                        ),
                        range: Some(helpers::rowan_range_to_lsp_range(
                            &line_index,
                            token.text_range(),
                        )),
                    }
                })
            })
    }
}
