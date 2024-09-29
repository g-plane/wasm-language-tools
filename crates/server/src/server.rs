use lsp_server::{Connection, ExtractError, Message, Notification, Request, RequestId, Response};
use lsp_types::{
    notification::{
        DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument, Notification as _,
        PublishDiagnostics,
    },
    request::{
        DocumentSymbolRequest, GotoDeclaration, GotoDefinition, GotoTypeDefinition,
        SemanticTokensFullRequest,
    },
    DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
    PublishDiagnosticsParams,
};
use wat_service::LanguageService;

#[derive(Default)]
pub struct Server {
    service: LanguageService,
}

impl Server {
    pub fn run(&mut self) -> anyhow::Result<()> {
        let (connection, io_threads) = Connection::stdio();
        let (id, params) = connection.initialize_start()?;
        connection.initialize_finish(
            id,
            serde_json::to_value(self.service.initialize(serde_json::from_value(params)?))?,
        )?;
        self.server_loop(connection)?;
        io_threads.join()?;
        Ok(())
    }

    fn server_loop(&mut self, conn: Connection) -> anyhow::Result<()> {
        for msg in &conn.receiver {
            match msg {
                Message::Request(mut req) => {
                    if conn.handle_shutdown(&req)? {
                        return Ok(());
                    }
                    match cast_req::<GotoDefinition>(req) {
                        Ok((id, params)) => {
                            conn.sender.send(Message::Response(Response {
                                id,
                                result: Some(serde_json::to_value(
                                    self.service.goto_definition(params),
                                )?),
                                error: None,
                            }))?;
                            continue;
                        }
                        Err(ExtractError::MethodMismatch(r)) => req = r,
                        Err(ExtractError::JsonError { .. }) => continue,
                    }
                    match cast_req::<GotoTypeDefinition>(req) {
                        Ok((id, params)) => {
                            conn.sender.send(Message::Response(Response {
                                id,
                                result: Some(serde_json::to_value(
                                    self.service.goto_type_definition(params),
                                )?),
                                error: None,
                            }))?;
                            continue;
                        }
                        Err(ExtractError::MethodMismatch(r)) => req = r,
                        Err(ExtractError::JsonError { .. }) => continue,
                    }
                    match cast_req::<GotoDeclaration>(req) {
                        Ok((id, params)) => {
                            conn.sender.send(Message::Response(Response {
                                id,
                                result: Some(serde_json::to_value(
                                    self.service.goto_declaration(params),
                                )?),
                                error: None,
                            }))?;
                            continue;
                        }
                        Err(ExtractError::MethodMismatch(r)) => req = r,
                        Err(ExtractError::JsonError { .. }) => continue,
                    }
                    match cast_req::<SemanticTokensFullRequest>(req) {
                        Ok((id, params)) => {
                            conn.sender.send(Message::Response(Response {
                                id,
                                result: Some(serde_json::to_value(
                                    self.service.semantic_tokens_full(params),
                                )?),
                                error: None,
                            }))?;
                            continue;
                        }
                        Err(ExtractError::MethodMismatch(r)) => req = r,
                        Err(ExtractError::JsonError { .. }) => continue,
                    }
                    match cast_req::<DocumentSymbolRequest>(req) {
                        Ok((id, params)) => {
                            conn.sender.send(Message::Response(Response {
                                id,
                                result: Some(serde_json::to_value(
                                    self.service.document_symbol(params),
                                )?),
                                error: None,
                            }))?;
                            continue;
                        }
                        Err(ExtractError::MethodMismatch(..)) => continue,
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

    fn handle_did_open_text_document(
        &mut self,
        params: DidOpenTextDocumentParams,
        conn: &Connection,
    ) -> anyhow::Result<()> {
        let diagnostics = self
            .service
            .commit_file(params.text_document.uri.clone(), params.text_document.text);
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
            let diagnostics = self
                .service
                .commit_file(params.text_document.uri.clone(), change.text.clone());
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
