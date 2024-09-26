use crate::{binder::SymbolTable, features, files::File};
use ahash::AHashMap;
use leptos_reactive::{create_memo, create_rw_signal, Memo, RwSignal, SignalGet, SignalSet};
use lsp_server::{Connection, ExtractError, Message, Notification, Request, RequestId, Response};
use lsp_types::{
    notification::{
        DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument, Notification as _,
        PublishDiagnostics,
    },
    request::GotoDefinition,
    Diagnostic, DiagnosticSeverity, DidChangeTextDocumentParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, GotoDefinitionParams, OneOf, Position, PublishDiagnosticsParams,
    Range, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind, Uri,
};
use rowan::ast::AstNode;

#[derive(Default)]
pub struct Server {
    files: AHashMap<Uri, RwSignal<File>>,
    symbol_tables: AHashMap<Uri, Memo<SymbolTable>>,
}

impl Server {
    pub fn run(&mut self) -> anyhow::Result<()> {
        let (connection, io_threads) = Connection::stdio();

        let server_capabilities = serde_json::to_value(&ServerCapabilities {
            definition_provider: Some(OneOf::Left(true)),
            text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
            ..Default::default()
        })
        .unwrap();
        let initialization_params = match connection.initialize(server_capabilities) {
            Ok(it) => it,
            Err(e) => {
                if e.channel_is_disconnected() {
                    io_threads.join()?;
                }
                return Err(e.into());
            }
        };
        self.server_loop(connection, initialization_params)?;
        io_threads.join()?;
        Ok(())
    }

    fn server_loop(&mut self, conn: Connection, _params: serde_json::Value) -> anyhow::Result<()> {
        for msg in &conn.receiver {
            match msg {
                Message::Request(mut req) => {
                    if conn.handle_shutdown(&req)? {
                        return Ok(());
                    }
                    match cast_req::<GotoDefinition>(req) {
                        Ok((id, params)) => {
                            self.handle_goto_definition(id, params, &conn)?;
                            continue;
                        }
                        Err(ExtractError::MethodMismatch(r)) => req = r,
                        Err(ExtractError::JsonError { .. }) => continue,
                    }
                }
                Message::Notification(mut notification) => {
                    match cast_notification::<DidOpenTextDocument>(notification) {
                        Ok(params) => {
                            self.handle_did_open_text_document(params, &conn)?;
                            continue;
                        }
                        Err(ExtractError::MethodMismatch(n)) => notification = n,
                        Err(ExtractError::JsonError { .. }) => continue,
                    };
                    match cast_notification::<DidChangeTextDocument>(notification) {
                        Ok(params) => {
                            self.handle_did_change_text_document(params, &conn)?;
                            continue;
                        }
                        Err(ExtractError::MethodMismatch(n)) => notification = n,
                        Err(ExtractError::JsonError { .. }) => continue,
                    };
                    match cast_notification::<DidCloseTextDocument>(notification) {
                        Ok(params) => {
                            self.handle_did_close_text_document(params, &conn)?;
                            continue;
                        }
                        Err(ExtractError::MethodMismatch(..)) => continue,
                        Err(ExtractError::JsonError { .. }) => continue,
                    };
                }
                Message::Response(_) => {}
            }
        }
        Ok(())
    }

    fn handle_goto_definition(
        &self,
        id: RequestId,
        params: GotoDefinitionParams,
        conn: &Connection,
    ) -> anyhow::Result<()> {
        let uri = params.text_document_position_params.text_document.uri;
        let resp = features::goto_definition(
            params.text_document_position_params.position,
            self.files.get(&uri),
            self.symbol_tables.get(&uri).map(SignalGet::get),
            uri,
        );
        conn.sender.send(Message::Response(Response {
            id,
            result: Some(serde_json::to_value(resp)?),
            error: None,
        }))?;

        Ok(())
    }

    fn handle_did_open_text_document(
        &mut self,
        params: DidOpenTextDocumentParams,
        conn: &Connection,
    ) -> anyhow::Result<()> {
        let diagnostics = self.accept_file(&params.text_document.uri, params.text_document.text);
        conn.sender.send(Message::Notification(Notification {
            method: PublishDiagnostics::METHOD.to_string(),
            params: serde_json::to_value(PublishDiagnosticsParams {
                uri: params.text_document.uri,
                diagnostics,
                version: None,
            })?,
        }))?;

        Ok(())
    }

    fn handle_did_change_text_document(
        &mut self,
        params: DidChangeTextDocumentParams,
        conn: &Connection,
    ) -> anyhow::Result<()> {
        if let Some(change) = params.content_changes.first() {
            let diagnostics = self.accept_file(&params.text_document.uri, change.text.clone());
            conn.sender.send(Message::Notification(Notification {
                method: PublishDiagnostics::METHOD.to_string(),
                params: serde_json::to_value(PublishDiagnosticsParams {
                    uri: params.text_document.uri,
                    diagnostics,
                    version: None,
                })?,
            }))?;
        }

        Ok(())
    }

    fn handle_did_close_text_document(
        &mut self,
        params: DidCloseTextDocumentParams,
        conn: &Connection,
    ) -> anyhow::Result<()> {
        self.files.remove(&params.text_document.uri);
        conn.sender.send(Message::Notification(Notification {
            method: PublishDiagnostics::METHOD.to_string(),
            params: serde_json::to_value(PublishDiagnosticsParams {
                uri: params.text_document.uri,
                diagnostics: vec![],
                version: None,
            })?,
        }))?;
        Ok(())
    }

    fn accept_file(&mut self, uri: &Uri, source: String) -> Vec<Diagnostic> {
        let file = self
            .files
            .entry(uri.clone())
            .and_modify(|file| file.set(File::new(&source)))
            .or_insert_with(|| create_rw_signal(File::new(&source)))
            .get();

        let mut diagnostics = file
            .syntax_errors
            .into_iter()
            .map(|diag| Diagnostic {
                range: diag.range,
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("wat".into()),
                message: diag.message,
                ..Default::default()
            })
            .collect::<Vec<_>>();

        let mut modules = file.tree.modules();
        if let Some(module) = modules.next() {
            if !self.symbol_tables.contains_key(uri) {
                self.symbol_tables
                    .insert(uri.clone(), create_memo(move |_| SymbolTable::new(&module)));
            }
        }
        diagnostics.extend(modules.map(|module| {
            let range = module.syntax().text_range();
            let start = file.line_index.line_col(range.start());
            let end = file.line_index.line_col(range.end());
            Diagnostic {
                range: Range::new(
                    Position::new(start.line, start.col),
                    Position::new(end.line, end.col),
                ),
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("wat".into()),
                message: "only one module is allowed in one file".into(),
                ..Default::default()
            }
        }));

        diagnostics
    }
}

fn cast_req<R>(req: Request) -> Result<(RequestId, R::Params), ExtractError<Request>>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    req.extract(R::METHOD)
}

fn cast_notification<N>(notification: Notification) -> Result<N::Params, ExtractError<Notification>>
where
    N: lsp_types::notification::Notification,
    N::Params: serde::de::DeserializeOwned,
{
    notification.extract(N::METHOD)
}
