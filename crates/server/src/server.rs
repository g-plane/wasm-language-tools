use crate::{
    binder::{SymbolTable, SymbolTables},
    files::{get_line_index, Files},
    parser::parse,
};
use comemo::Track;
use lsp_server::{Connection, ExtractError, Message, Notification, Request, RequestId};
use lsp_types::{
    notification::{
        DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument, Notification as _,
        PublishDiagnostics,
    },
    Diagnostic, DiagnosticSeverity, DidChangeTextDocumentParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, Position, PublishDiagnosticsParams, Range, ServerCapabilities,
    TextDocumentSyncCapability, TextDocumentSyncKind, Uri,
};
use rowan::ast::AstNode;

#[derive(Default)]
pub struct Server {
    files: Files,
    symbol_tables: SymbolTables,
}

impl Server {
    pub fn run(&mut self) -> anyhow::Result<()> {
        let (connection, io_threads) = Connection::stdio();

        let server_capabilities = serde_json::to_value(&ServerCapabilities {
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

    fn handle_did_open_text_document(
        &mut self,
        params: DidOpenTextDocumentParams,
        conn: &Connection,
    ) -> anyhow::Result<()> {
        self.files
            .write(params.text_document.uri.clone(), params.text_document.text);

        let diagnostics = self.accept_file(&params.text_document.uri);
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
            self.files
                .write(params.text_document.uri.clone(), change.text.clone());
        }

        let diagnostics = self.accept_file(&params.text_document.uri);
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

    fn accept_file(&mut self, uri: &Uri) -> Vec<Diagnostic> {
        let (root, diagnostics) = parse(uri, self.files.track());
        let mut diagnostics = diagnostics
            .into_iter()
            .map(|diag| Diagnostic {
                range: diag.range,
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("wat".into()),
                message: diag.message,
                ..Default::default()
            })
            .collect::<Vec<_>>();

        let mut modules = root.modules();
        if let Some(module) = modules.next() {
            self.symbol_tables
                .write(uri.clone(), SymbolTable::new(&module));
        }
        let line_index = get_line_index(uri, self.files.track());
        diagnostics.extend(modules.map(|module| {
            let range = module.syntax().text_range();
            let start = line_index.line_col(range.start());
            let end = line_index.line_col(range.end());
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
