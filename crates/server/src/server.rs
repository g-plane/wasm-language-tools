use crate::{
    message::{try_cast_notification, try_cast_request, CastError, Message, ResponseError},
    sent::SentRequests,
    stdio,
};
use lsp_types::{
    error_codes,
    notification::{
        DidChangeConfiguration, DidChangeTextDocument, DidOpenTextDocument, Exit,
        Notification as _, PublishDiagnostics,
    },
    request::{
        CallHierarchyIncomingCalls, CallHierarchyOutgoingCalls, CallHierarchyPrepare,
        CodeActionRequest, Completion, DocumentDiagnosticRequest, DocumentHighlightRequest,
        DocumentSymbolRequest, FoldingRangeRequest, Formatting, GotoDeclaration, GotoDefinition,
        GotoTypeDefinition, HoverRequest, InlayHintRequest, PrepareRenameRequest, RangeFormatting,
        References, RegisterCapability, Rename, Request as _, SelectionRangeRequest,
        SemanticTokensFullRequest, SemanticTokensRangeRequest, Shutdown, SignatureHelpRequest,
        WorkspaceConfiguration, WorkspaceDiagnosticRefresh,
    },
    ConfigurationItem, ConfigurationParams, DidChangeConfigurationParams,
    DidChangeTextDocumentParams, DidOpenTextDocumentParams, InitializeParams, Uri,
};
use std::ops::Deref;
use wat_service::LanguageService;

#[derive(Default)]
pub struct Server {
    service: LanguageService,
    support_pull_diagnostics: bool,
    support_refresh_diagnostics: bool,
    support_pull_config: bool,
    sent_requests: SentRequests<Vec<Uri>>,
}

impl Server {
    pub async fn run(&mut self) -> anyhow::Result<()> {
        self.initialize().await?;
        stdio::write(Message::Request {
            id: self.sent_requests.next_id(),
            method: RegisterCapability::METHOD.into(),
            params: serde_json::to_value(self.service.dynamic_capabilities())?,
        })
        .await?;

        loop {
            let message = match stdio::read().await {
                Ok(Some(message)) => message,
                Ok(None) => return Ok(()),
                _ => continue,
            };
            match message {
                Message::Request { id, method, params } => {
                    blocking::unblock({
                        let service = self.service.fork();
                        move || {
                            if let Ok(Some(message)) =
                                Self::handle_request(service, id, method, params)
                            {
                                stdio::write_sync(message)?;
                            }
                            Ok::<_, anyhow::Error>(())
                        }
                    })
                    .detach();
                }
                Message::OkResponse { id, result } => {
                    self.handle_response(id, result).await?;
                }
                Message::Notification { method, mut params } => {
                    match try_cast_notification::<DidOpenTextDocument>(&method, params) {
                        Ok(params) => {
                            self.handle_did_open_text_document(params).await?;
                            continue;
                        }
                        Err(CastError::MethodMismatch(p)) => params = p,
                        Err(CastError::JsonError(..)) => continue,
                    };
                    match try_cast_notification::<DidChangeTextDocument>(&method, params) {
                        Ok(params) => {
                            self.handle_did_change_text_document(params).await?;
                            continue;
                        }
                        Err(CastError::MethodMismatch(p)) => params = p,
                        Err(CastError::JsonError(..)) => continue,
                    };
                    match try_cast_notification::<DidChangeConfiguration>(&method, params) {
                        Ok(params) => {
                            self.handle_did_change_configuration(params).await?;
                            continue;
                        }
                        Err(CastError::MethodMismatch(p)) => params = p,
                        Err(CastError::JsonError(..)) => continue,
                    };
                    match try_cast_notification::<Exit>(&method, params) {
                        Ok(..) => return Ok(()),
                        Err(CastError::MethodMismatch(..)) => {}
                        Err(CastError::JsonError(..)) => {}
                    };
                }
                _ => {}
            }
        }
    }

    async fn initialize(&mut self) -> anyhow::Result<()> {
        let (id, params) = match stdio::read().await {
            Ok(Some(Message::Request { id, method, params })) if method == "initialize" => {
                (id, serde_json::from_value::<InitializeParams>(params)?)
            }
            _ => return Err(anyhow::anyhow!("expected `initialize` request")),
        };
        self.support_pull_diagnostics = params
            .capabilities
            .text_document
            .as_ref()
            .and_then(|it| it.diagnostic.as_ref())
            .is_some();
        // read it from capabilities once https://github.com/gluon-lang/lsp-types/pull/281 is merged
        self.support_refresh_diagnostics = true;
        self.support_pull_config = matches!(
            params
                .capabilities
                .workspace
                .as_ref()
                .and_then(|it| it.configuration),
            Some(true)
        );
        stdio::write(Message::OkResponse {
            id,
            result: serde_json::to_value(self.service.initialize(params))?,
        })
        .await?;
        Ok(())
    }

    fn handle_request(
        service: impl Deref<Target = LanguageService>,
        id: u32,
        method: String,
        mut params: serde_json::Value,
    ) -> anyhow::Result<Option<Message>> {
        if service.is_cancelled() {
            return Ok(Some(Message::ErrResponse {
                id,
                error: ResponseError {
                    code: error_codes::SERVER_CANCELLED,
                    message: "This request is cancelled by server.".into(),
                    data: None,
                },
            }));
        }
        match try_cast_request::<CallHierarchyPrepare>(&method, params) {
            Ok(params) => {
                return Ok(Some(Message::OkResponse {
                    id,
                    result: serde_json::to_value(service.prepare_call_hierarchy(params))?,
                }));
            }
            Err(CastError::MethodMismatch(p)) => params = p,
            Err(CastError::JsonError(..)) => return Ok(None),
        }
        match try_cast_request::<CallHierarchyIncomingCalls>(&method, params) {
            Ok(params) => {
                return Ok(Some(Message::OkResponse {
                    id,
                    result: serde_json::to_value(service.call_hierarchy_incoming_calls(params))?,
                }));
            }
            Err(CastError::MethodMismatch(p)) => params = p,
            Err(CastError::JsonError(..)) => return Ok(None),
        }
        match try_cast_request::<CallHierarchyOutgoingCalls>(&method, params) {
            Ok(params) => {
                return Ok(Some(Message::OkResponse {
                    id,
                    result: serde_json::to_value(service.call_hierarchy_outgoing_calls(params))?,
                }));
            }
            Err(CastError::MethodMismatch(p)) => params = p,
            Err(CastError::JsonError(..)) => return Ok(None),
        }
        match try_cast_request::<CodeActionRequest>(&method, params) {
            Ok(params) => {
                return Ok(Some(Message::OkResponse {
                    id,
                    result: serde_json::to_value(service.code_action(params))?,
                }));
            }
            Err(CastError::MethodMismatch(p)) => params = p,
            Err(CastError::JsonError(..)) => return Ok(None),
        }
        match try_cast_request::<Completion>(&method, params) {
            Ok(params) => {
                return Ok(Some(Message::OkResponse {
                    id,
                    result: serde_json::to_value(service.completion(params))?,
                }));
            }
            Err(CastError::MethodMismatch(p)) => params = p,
            Err(CastError::JsonError(..)) => return Ok(None),
        }
        match try_cast_request::<DocumentDiagnosticRequest>(&method, params) {
            Ok(params) => {
                return Ok(Some(Message::OkResponse {
                    id,
                    result: serde_json::to_value(service.pull_diagnostics(params))?,
                }));
            }
            Err(CastError::MethodMismatch(p)) => params = p,
            Err(CastError::JsonError(..)) => return Ok(None),
        }
        match try_cast_request::<DocumentHighlightRequest>(&method, params) {
            Ok(params) => {
                return Ok(Some(Message::OkResponse {
                    id,
                    result: serde_json::to_value(service.document_highlight(params))?,
                }));
            }
            Err(CastError::MethodMismatch(p)) => params = p,
            Err(CastError::JsonError(..)) => return Ok(None),
        }
        match try_cast_request::<FoldingRangeRequest>(&method, params) {
            Ok(params) => {
                return Ok(Some(Message::OkResponse {
                    id,
                    result: serde_json::to_value(service.folding_range(params))?,
                }));
            }
            Err(CastError::MethodMismatch(p)) => params = p,
            Err(CastError::JsonError(..)) => return Ok(None),
        }
        match try_cast_request::<Formatting>(&method, params) {
            Ok(params) => {
                return Ok(Some(Message::OkResponse {
                    id,
                    result: serde_json::to_value(service.formatting(params))?,
                }));
            }
            Err(CastError::MethodMismatch(p)) => params = p,
            Err(CastError::JsonError(..)) => return Ok(None),
        }
        match try_cast_request::<RangeFormatting>(&method, params) {
            Ok(params) => {
                return Ok(Some(Message::OkResponse {
                    id,
                    result: serde_json::to_value(service.range_formatting(params))?,
                }));
            }
            Err(CastError::MethodMismatch(p)) => params = p,
            Err(CastError::JsonError(..)) => return Ok(None),
        }
        match try_cast_request::<GotoDefinition>(&method, params) {
            Ok(params) => {
                return Ok(Some(Message::OkResponse {
                    id,
                    result: serde_json::to_value(service.goto_definition(params))?,
                }));
            }
            Err(CastError::MethodMismatch(p)) => params = p,
            Err(CastError::JsonError(..)) => return Ok(None),
        }
        match try_cast_request::<GotoTypeDefinition>(&method, params) {
            Ok(params) => {
                return Ok(Some(Message::OkResponse {
                    id,
                    result: serde_json::to_value(service.goto_type_definition(params))?,
                }));
            }
            Err(CastError::MethodMismatch(p)) => params = p,
            Err(CastError::JsonError(..)) => return Ok(None),
        }
        match try_cast_request::<GotoDeclaration>(&method, params) {
            Ok(params) => {
                return Ok(Some(Message::OkResponse {
                    id,
                    result: serde_json::to_value(service.goto_declaration(params))?,
                }));
            }
            Err(CastError::MethodMismatch(p)) => params = p,
            Err(CastError::JsonError(..)) => return Ok(None),
        }
        match try_cast_request::<HoverRequest>(&method, params) {
            Ok(params) => {
                return Ok(Some(Message::OkResponse {
                    id,
                    result: serde_json::to_value(service.hover(params))?,
                }));
            }
            Err(CastError::MethodMismatch(p)) => params = p,
            Err(CastError::JsonError(..)) => return Ok(None),
        }
        match try_cast_request::<InlayHintRequest>(&method, params) {
            Ok(params) => {
                return Ok(Some(Message::OkResponse {
                    id,
                    result: serde_json::to_value(service.inlay_hint(params))?,
                }));
            }
            Err(CastError::MethodMismatch(p)) => params = p,
            Err(CastError::JsonError(..)) => return Ok(None),
        }
        match try_cast_request::<References>(&method, params) {
            Ok(params) => {
                return Ok(Some(Message::OkResponse {
                    id,
                    result: serde_json::to_value(service.find_references(params))?,
                }));
            }
            Err(CastError::MethodMismatch(p)) => params = p,
            Err(CastError::JsonError(..)) => return Ok(None),
        }
        match try_cast_request::<PrepareRenameRequest>(&method, params) {
            Ok(params) => {
                return Ok(Some(Message::OkResponse {
                    id,
                    result: serde_json::to_value(service.prepare_rename(params))?,
                }));
            }
            Err(CastError::MethodMismatch(p)) => params = p,
            Err(CastError::JsonError(..)) => return Ok(None),
        }
        match try_cast_request::<Rename>(&method, params) {
            Ok(params) => {
                return match service.rename(params) {
                    Ok(result) => Ok(Some(Message::OkResponse {
                        id,
                        result: serde_json::to_value(result)?,
                    })),
                    Err(message) => Ok(Some(Message::ErrResponse {
                        id,
                        error: ResponseError {
                            code: -1,
                            message,
                            data: None,
                        },
                    })),
                };
            }
            Err(CastError::MethodMismatch(p)) => params = p,
            Err(CastError::JsonError(..)) => return Ok(None),
        }
        match try_cast_request::<SelectionRangeRequest>(&method, params) {
            Ok(params) => {
                return Ok(Some(Message::OkResponse {
                    id,
                    result: serde_json::to_value(service.selection_range(params))?,
                }));
            }
            Err(CastError::MethodMismatch(p)) => params = p,
            Err(CastError::JsonError(..)) => return Ok(None),
        }
        match try_cast_request::<SemanticTokensFullRequest>(&method, params) {
            Ok(params) => {
                return Ok(Some(Message::OkResponse {
                    id,
                    result: serde_json::to_value(service.semantic_tokens_full(params))?,
                }));
            }
            Err(CastError::MethodMismatch(p)) => params = p,
            Err(CastError::JsonError(..)) => return Ok(None),
        }
        match try_cast_request::<SemanticTokensRangeRequest>(&method, params) {
            Ok(params) => {
                return Ok(Some(Message::OkResponse {
                    id,
                    result: serde_json::to_value(service.semantic_tokens_range(params))?,
                }));
            }
            Err(CastError::MethodMismatch(p)) => params = p,
            Err(CastError::JsonError(..)) => return Ok(None),
        }
        match try_cast_request::<SignatureHelpRequest>(&method, params) {
            Ok(params) => {
                return Ok(Some(Message::OkResponse {
                    id,
                    result: serde_json::to_value(service.signature_help(params))?,
                }));
            }
            Err(CastError::MethodMismatch(p)) => params = p,
            Err(CastError::JsonError(..)) => return Ok(None),
        }
        match try_cast_request::<DocumentSymbolRequest>(&method, params) {
            Ok(params) => {
                return Ok(Some(Message::OkResponse {
                    id,
                    result: serde_json::to_value(service.document_symbol(params))?,
                }));
            }
            Err(CastError::MethodMismatch(p)) => params = p,
            Err(CastError::JsonError(..)) => return Ok(None),
        }
        match try_cast_request::<Shutdown>(&method, params) {
            Ok(..) => Ok(Some(Message::OkResponse {
                id,
                result: serde_json::Value::Null,
            })),
            Err(CastError::MethodMismatch(..)) => Ok(None),
            Err(CastError::JsonError(..)) => Ok(None),
        }
    }

    async fn handle_response(&mut self, id: u32, result: serde_json::Value) -> anyhow::Result<()> {
        if let Some((uris, configs)) = self
            .sent_requests
            .remove(id)
            .zip(serde_json::from_value::<Vec<_>>(result).ok())
        {
            uris.iter()
                .zip(configs)
                .for_each(|(uri, config)| self.service.set_config(uri.clone(), config));
            if self.support_refresh_diagnostics {
                stdio::write(Message::Request {
                    id: self.sent_requests.next_id(),
                    method: WorkspaceDiagnosticRefresh::METHOD.into(),
                    params: serde_json::Value::Null,
                })
                .await?;
            }
        }
        Ok(())
    }

    async fn handle_did_open_text_document(
        &mut self,
        params: DidOpenTextDocumentParams,
    ) -> anyhow::Result<()> {
        let uri = params.text_document.uri;
        self.service.commit(uri.clone(), params.text_document.text);
        if !self.support_pull_diagnostics {
            self.publish_diagnostics(uri.clone()).await?;
        }
        if self.support_pull_config {
            stdio::write(self.sent_requests.add(
                WorkspaceConfiguration::METHOD.into(),
                serde_json::to_value(ConfigurationParams {
                    items: vec![ConfigurationItem {
                        scope_uri: Some(uri.clone()),
                        section: Some("wasmLanguageTools".to_string()),
                    }],
                })?,
                vec![uri],
            ))
            .await?;
        }
        Ok(())
    }

    async fn handle_did_change_text_document(
        &mut self,
        params: DidChangeTextDocumentParams,
    ) -> anyhow::Result<()> {
        if let Some(change) = params.content_changes.first() {
            self.service
                .commit(params.text_document.uri.clone(), change.text.clone());
            if !self.support_pull_diagnostics {
                self.publish_diagnostics(params.text_document.uri).await?;
            }
        }
        Ok(())
    }

    async fn handle_did_change_configuration(
        &mut self,
        params: DidChangeConfigurationParams,
    ) -> anyhow::Result<()> {
        if self.support_pull_config {
            let uris = self
                .service
                .get_configs()
                .map(|(uri, _)| uri)
                .collect::<Vec<_>>();
            stdio::write(
                self.sent_requests.add(
                    WorkspaceConfiguration::METHOD.into(),
                    serde_json::to_value(ConfigurationParams {
                        items: uris
                            .iter()
                            .map(|uri| ConfigurationItem {
                                scope_uri: Some(uri.clone()),
                                section: Some("wasmLanguageTools".to_string()),
                            })
                            .collect(),
                    })?,
                    uris,
                ),
            )
            .await?;
        }
        match &params.settings {
            serde_json::Value::Object(object) if !object.is_empty() => {
                if let Ok(config) = serde_json::from_value(params.settings) {
                    self.service.set_global_config(config);
                }
            }
            _ => {}
        }
        Ok(())
    }

    async fn publish_diagnostics(&mut self, uri: Uri) -> anyhow::Result<()> {
        stdio::write(Message::Notification {
            method: PublishDiagnostics::METHOD.into(),
            params: serde_json::to_value(self.service.publish_diagnostics(uri))?,
        })
        .await
    }
}
