use lsp_server::{
    Connection, ExtractError, Message, Notification, ReqQueue, Request, RequestId, Response,
    ResponseError,
};
use lsp_types::{
    notification::{
        DidChangeConfiguration, DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument,
        Notification as _, PublishDiagnostics,
    },
    request::{
        CallHierarchyIncomingCalls, CallHierarchyOutgoingCalls, CallHierarchyPrepare,
        CodeActionRequest, Completion, DocumentDiagnosticRequest, DocumentSymbolRequest,
        FoldingRangeRequest, Formatting, GotoDeclaration, GotoDefinition, GotoTypeDefinition,
        HoverRequest, InlayHintRequest, PrepareRenameRequest, RangeFormatting, References,
        RegisterCapability, Rename, Request as _, SelectionRangeRequest, SemanticTokensFullRequest,
        SemanticTokensRangeRequest, SignatureHelpRequest, WorkspaceConfiguration,
        WorkspaceDiagnosticRefresh,
    },
    ConfigurationItem, ConfigurationParams, DidChangeConfigurationParams,
    DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
    InitializeParams, PublishDiagnosticsParams, Uri,
};
use wat_service::LanguageService;

#[derive(Default)]
pub struct Server {
    service: LanguageService,
    support_pull_diagnostics: bool,
    support_refresh_diagnostics: bool,
    support_pull_config: bool,
    req_queue: ReqQueue<(), Vec<Uri>>,
}

impl Server {
    pub fn run(&mut self) -> anyhow::Result<()> {
        let (connection, io_threads) = Connection::stdio();
        let (id, params) = connection.initialize_start()?;
        let params = serde_json::from_value::<InitializeParams>(params)?;
        self.support_pull_diagnostics = params
            .capabilities
            .text_document
            .as_ref()
            .and_then(|it| it.diagnostic.as_ref())
            .is_some();
        self.support_refresh_diagnostics = matches!(
            params
                .capabilities
                .workspace
                .as_ref()
                .and_then(|it| it.diagnostic.as_ref())
                .and_then(|it| it.refresh_support),
            Some(true)
        );
        self.support_pull_config = matches!(
            params
                .capabilities
                .workspace
                .as_ref()
                .and_then(|it| it.configuration),
            Some(true)
        );
        connection.initialize_finish(id, serde_json::to_value(self.service.initialize(params))?)?;
        connection
            .sender
            .send(Message::Request(self.req_queue.outgoing.register(
                RegisterCapability::METHOD.into(),
                self.service.dynamic_capabilities(),
                vec![],
            )))?;
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
                    match cast_req::<CallHierarchyPrepare>(req) {
                        Ok((id, params)) => {
                            conn.sender.send(Message::Response(Response {
                                id,
                                result: Some(serde_json::to_value(
                                    self.service.prepare_call_hierarchy(params),
                                )?),
                                error: None,
                            }))?;
                            continue;
                        }
                        Err(ExtractError::MethodMismatch(r)) => req = r,
                        Err(ExtractError::JsonError { .. }) => continue,
                    }
                    match cast_req::<CallHierarchyIncomingCalls>(req) {
                        Ok((id, params)) => {
                            conn.sender.send(Message::Response(Response {
                                id,
                                result: Some(serde_json::to_value(
                                    self.service.call_hierarchy_incoming_calls(params),
                                )?),
                                error: None,
                            }))?;
                            continue;
                        }
                        Err(ExtractError::MethodMismatch(r)) => req = r,
                        Err(ExtractError::JsonError { .. }) => continue,
                    }
                    match cast_req::<CallHierarchyOutgoingCalls>(req) {
                        Ok((id, params)) => {
                            conn.sender.send(Message::Response(Response {
                                id,
                                result: Some(serde_json::to_value(
                                    self.service.call_hierarchy_outgoing_calls(params),
                                )?),
                                error: None,
                            }))?;
                            continue;
                        }
                        Err(ExtractError::MethodMismatch(r)) => req = r,
                        Err(ExtractError::JsonError { .. }) => continue,
                    }
                    match cast_req::<CodeActionRequest>(req) {
                        Ok((id, params)) => {
                            conn.sender.send(Message::Response(Response {
                                id,
                                result: Some(serde_json::to_value(
                                    self.service.code_action(params),
                                )?),
                                error: None,
                            }))?;
                            continue;
                        }
                        Err(ExtractError::MethodMismatch(r)) => req = r,
                        Err(ExtractError::JsonError { .. }) => continue,
                    }
                    match cast_req::<Completion>(req) {
                        Ok((id, params)) => {
                            conn.sender.send(Message::Response(Response {
                                id,
                                result: Some(serde_json::to_value(
                                    self.service.completion(params),
                                )?),
                                error: None,
                            }))?;
                            continue;
                        }
                        Err(ExtractError::MethodMismatch(r)) => req = r,
                        Err(ExtractError::JsonError { .. }) => continue,
                    }
                    match cast_req::<DocumentDiagnosticRequest>(req) {
                        Ok((id, params)) => {
                            conn.sender.send(Message::Response(Response {
                                id,
                                result: Some(serde_json::to_value(
                                    self.service.pull_diagnostics(params),
                                )?),
                                error: None,
                            }))?;
                            continue;
                        }
                        Err(ExtractError::MethodMismatch(r)) => req = r,
                        Err(ExtractError::JsonError { .. }) => continue,
                    }
                    match cast_req::<FoldingRangeRequest>(req) {
                        Ok((id, params)) => {
                            conn.sender.send(Message::Response(Response {
                                id,
                                result: Some(serde_json::to_value(
                                    self.service.folding_range(params),
                                )?),
                                error: None,
                            }))?;
                            continue;
                        }
                        Err(ExtractError::MethodMismatch(r)) => req = r,
                        Err(ExtractError::JsonError { .. }) => continue,
                    }
                    match cast_req::<Formatting>(req) {
                        Ok((id, params)) => {
                            conn.sender.send(Message::Response(Response {
                                id,
                                result: Some(serde_json::to_value(
                                    self.service.formatting(params),
                                )?),
                                error: None,
                            }))?;
                            continue;
                        }
                        Err(ExtractError::MethodMismatch(r)) => req = r,
                        Err(ExtractError::JsonError { .. }) => continue,
                    }
                    match cast_req::<RangeFormatting>(req) {
                        Ok((id, params)) => {
                            conn.sender.send(Message::Response(Response {
                                id,
                                result: Some(serde_json::to_value(
                                    self.service.range_formatting(params),
                                )?),
                                error: None,
                            }))?;
                            continue;
                        }
                        Err(ExtractError::MethodMismatch(r)) => req = r,
                        Err(ExtractError::JsonError { .. }) => continue,
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
                    match cast_req::<HoverRequest>(req) {
                        Ok((id, params)) => {
                            conn.sender.send(Message::Response(Response {
                                id,
                                result: Some(serde_json::to_value(self.service.hover(params))?),
                                error: None,
                            }))?;
                            continue;
                        }
                        Err(ExtractError::MethodMismatch(r)) => req = r,
                        Err(ExtractError::JsonError { .. }) => continue,
                    }
                    match cast_req::<InlayHintRequest>(req) {
                        Ok((id, params)) => {
                            conn.sender.send(Message::Response(Response {
                                id,
                                result: Some(serde_json::to_value(
                                    self.service.inlay_hint(params),
                                )?),
                                error: None,
                            }))?;
                            continue;
                        }
                        Err(ExtractError::MethodMismatch(r)) => req = r,
                        Err(ExtractError::JsonError { .. }) => continue,
                    }
                    match cast_req::<References>(req) {
                        Ok((id, params)) => {
                            conn.sender.send(Message::Response(Response {
                                id,
                                result: Some(serde_json::to_value(
                                    self.service.find_references(params),
                                )?),
                                error: None,
                            }))?;
                            continue;
                        }
                        Err(ExtractError::MethodMismatch(r)) => req = r,
                        Err(ExtractError::JsonError { .. }) => continue,
                    }
                    match cast_req::<PrepareRenameRequest>(req) {
                        Ok((id, params)) => {
                            conn.sender.send(Message::Response(Response {
                                id,
                                result: Some(serde_json::to_value(
                                    self.service.prepare_rename(params),
                                )?),
                                error: None,
                            }))?;
                            continue;
                        }
                        Err(ExtractError::MethodMismatch(r)) => req = r,
                        Err(ExtractError::JsonError { .. }) => continue,
                    }
                    match cast_req::<Rename>(req) {
                        Ok((id, params)) => {
                            match self.service.rename(params) {
                                Ok(result) => conn.sender.send(Message::Response(Response {
                                    id,
                                    result: Some(serde_json::to_value(result)?),
                                    error: None,
                                }))?,
                                Err(message) => conn.sender.send(Message::Response(Response {
                                    id,
                                    result: None,
                                    error: Some(ResponseError {
                                        code: -1,
                                        message,
                                        data: None,
                                    }),
                                }))?,
                            }
                            continue;
                        }
                        Err(ExtractError::MethodMismatch(r)) => req = r,
                        Err(ExtractError::JsonError { .. }) => continue,
                    }
                    match cast_req::<SelectionRangeRequest>(req) {
                        Ok((id, params)) => {
                            conn.sender.send(Message::Response(Response {
                                id,
                                result: Some(serde_json::to_value(
                                    self.service.selection_range(params),
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
                    match cast_req::<SemanticTokensRangeRequest>(req) {
                        Ok((id, params)) => {
                            conn.sender.send(Message::Response(Response {
                                id,
                                result: Some(serde_json::to_value(
                                    self.service.semantic_tokens_range(params),
                                )?),
                                error: None,
                            }))?;
                            continue;
                        }
                        Err(ExtractError::MethodMismatch(r)) => req = r,
                        Err(ExtractError::JsonError { .. }) => continue,
                    }
                    match cast_req::<SignatureHelpRequest>(req) {
                        Ok((id, params)) => {
                            conn.sender.send(Message::Response(Response {
                                id,
                                result: Some(serde_json::to_value(
                                    self.service.signature_help(params),
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
                        Err(ExtractError::MethodMismatch(n)) => notification = n,
                        Err(ExtractError::JsonError { .. }) => continue,
                    };
                    match cast_notification::<DidChangeConfiguration>(notification) {
                        Ok(params) => {
                            self.handle_did_change_configuration(params, &conn)?;
                            continue;
                        }
                        Err(ExtractError::MethodMismatch(..)) => continue,
                        Err(ExtractError::JsonError { .. }) => continue,
                    };
                }
                Message::Response(response) => {
                    self.handle_response(&conn, response)?;
                }
            }
        }
        Ok(())
    }

    fn handle_response(&mut self, conn: &Connection, response: Response) -> anyhow::Result<()> {
        if let Some((uris, configs)) = self.req_queue.outgoing.complete(response.id).zip(
            response
                .result
                .and_then(|result| serde_json::from_value::<Vec<_>>(result).ok()),
        ) {
            uris.iter()
                .zip(configs)
                .for_each(|(uri, config)| self.service.set_config(uri.clone(), config));
            if self.support_refresh_diagnostics {
                conn.sender
                    .send(Message::Request(self.req_queue.outgoing.register(
                        WorkspaceDiagnosticRefresh::METHOD.into(),
                        serde_json::Value::Null,
                        vec![],
                    )))?;
            }
        }
        Ok(())
    }

    fn handle_did_open_text_document(
        &mut self,
        params: DidOpenTextDocumentParams,
        conn: &Connection,
    ) -> anyhow::Result<()> {
        let uri = params.text_document.uri;
        self.service.commit(uri.clone(), params.text_document.text);
        if !self.support_pull_diagnostics {
            self.publish_diagnostics(conn, uri.clone())?;
        }
        if self.support_pull_config {
            conn.sender
                .send(Message::Request(self.req_queue.outgoing.register(
                    WorkspaceConfiguration::METHOD.into(),
                    ConfigurationParams {
                        items: vec![ConfigurationItem {
                            scope_uri: Some(uri.clone()),
                            section: Some("wasmLanguageTools".to_string()),
                        }],
                    },
                    vec![uri],
                )))?;
        }
        Ok(())
    }

    fn handle_did_change_text_document(
        &mut self,
        params: DidChangeTextDocumentParams,
        conn: &Connection,
    ) -> anyhow::Result<()> {
        if let Some(change) = params.content_changes.first() {
            self.service
                .commit(params.text_document.uri.clone(), change.text.clone());
            if !self.support_pull_diagnostics {
                self.publish_diagnostics(conn, params.text_document.uri)?;
            }
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

    fn handle_did_change_configuration(
        &mut self,
        params: DidChangeConfigurationParams,
        conn: &Connection,
    ) -> anyhow::Result<()> {
        if self.support_pull_config {
            let uris = self
                .service
                .get_configs()
                .map(|(uri, _)| uri)
                .collect::<Vec<_>>();
            conn.sender.send(Message::Request(
                self.req_queue.outgoing.register(
                    WorkspaceConfiguration::METHOD.into(),
                    ConfigurationParams {
                        items: uris
                            .iter()
                            .map(|uri| ConfigurationItem {
                                scope_uri: Some(uri.clone()),
                                section: Some("wasmLanguageTools".to_string()),
                            })
                            .collect(),
                    },
                    uris,
                ),
            ))?;
        } else if let Ok(config) = serde_json::from_value(params.settings) {
            self.service.set_global_config(config);
        }
        Ok(())
    }

    fn publish_diagnostics(&self, conn: &Connection, uri: Uri) -> anyhow::Result<()> {
        conn.sender.send(Message::Notification(Notification {
            method: PublishDiagnostics::METHOD.to_string(),
            params: serde_json::to_value(self.service.publish_diagnostics(uri))?,
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
