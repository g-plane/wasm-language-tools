use crate::{
    message::{try_cast_notification, try_cast_request, CastError, Message, ResponseError},
    sent::SentRequests,
    stdio::Stdio,
};
use lsp_types::{
    notification::{
        DidChangeConfiguration, DidChangeTextDocument, DidOpenTextDocument, Notification as _,
        PublishDiagnostics,
    },
    request::{
        CallHierarchyIncomingCalls, CallHierarchyOutgoingCalls, CallHierarchyPrepare,
        CodeActionRequest, Completion, DocumentDiagnosticRequest, DocumentHighlightRequest,
        DocumentSymbolRequest, FoldingRangeRequest, Formatting, GotoDeclaration, GotoDefinition,
        GotoTypeDefinition, HoverRequest, InlayHintRequest, PrepareRenameRequest, RangeFormatting,
        References, RegisterCapability, Rename, Request as _, SelectionRangeRequest,
        SemanticTokensFullRequest, SemanticTokensRangeRequest, SignatureHelpRequest,
        WorkspaceConfiguration, WorkspaceDiagnosticRefresh,
    },
    ConfigurationItem, ConfigurationParams, DidChangeConfigurationParams,
    DidChangeTextDocumentParams, DidOpenTextDocumentParams, InitializeParams, Uri,
};
use wat_service::LanguageService;

#[derive(Default)]
pub struct Server {
    stdio: Stdio,
    service: LanguageService,
    support_pull_diagnostics: bool,
    support_refresh_diagnostics: bool,
    support_pull_config: bool,
    sent_requests: SentRequests<Vec<Uri>>,
}

impl Server {
    pub async fn run(&mut self) -> anyhow::Result<()> {
        self.initialize().await?;
        self.stdio
            .write(self.sent_requests.add(
                RegisterCapability::METHOD.into(),
                serde_json::to_value(self.service.dynamic_capabilities())?,
                vec![],
            ))
            .await?;

        loop {
            let Ok(message) = self.stdio.read().await else {
                continue;
            };
            match &message {
                Message::Request { id, method, .. } if method == "shutdown" => {
                    self.stdio
                        .write(Message::OkResponse {
                            id: *id,
                            result: serde_json::Value::Null,
                        })
                        .await?;
                    break Ok(());
                }
                _ => {
                    if let Some(message) = self.handle_message(message).await? {
                        self.stdio.write(message).await?;
                    }
                }
            }
        }
    }

    async fn initialize(&mut self) -> anyhow::Result<()> {
        let message = self.stdio.read().await?;
        let (id, params) = match message {
            Message::Request { id, method, params } if method == "initialize" => {
                (id, serde_json::from_value::<InitializeParams>(params)?)
            }
            _ => return Ok(()),
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
        self.stdio
            .write(Message::OkResponse {
                id,
                result: serde_json::to_value(self.service.initialize(params))?,
            })
            .await?;
        Ok(())
    }

    async fn handle_message(&mut self, message: Message) -> anyhow::Result<Option<Message>> {
        match message {
            Message::Request {
                id,
                method,
                mut params,
            } => {
                match try_cast_request::<CallHierarchyPrepare>(&method, params) {
                    Ok(params) => {
                        return Ok(Some(Message::OkResponse {
                            id,
                            result: serde_json::to_value(
                                self.service.prepare_call_hierarchy(params),
                            )?,
                        }));
                    }
                    Err(CastError::MethodMismatch(p)) => params = p,
                    Err(CastError::JsonError(..)) => return Ok(None),
                }
                match try_cast_request::<CallHierarchyIncomingCalls>(&method, params) {
                    Ok(params) => {
                        return Ok(Some(Message::OkResponse {
                            id,
                            result: serde_json::to_value(
                                self.service.call_hierarchy_incoming_calls(params),
                            )?,
                        }));
                    }
                    Err(CastError::MethodMismatch(p)) => params = p,
                    Err(CastError::JsonError(..)) => return Ok(None),
                }
                match try_cast_request::<CallHierarchyOutgoingCalls>(&method, params) {
                    Ok(params) => {
                        return Ok(Some(Message::OkResponse {
                            id,
                            result: serde_json::to_value(
                                self.service.call_hierarchy_outgoing_calls(params),
                            )?,
                        }));
                    }
                    Err(CastError::MethodMismatch(p)) => params = p,
                    Err(CastError::JsonError(..)) => return Ok(None),
                }
                match try_cast_request::<CodeActionRequest>(&method, params) {
                    Ok(params) => {
                        return Ok(Some(Message::OkResponse {
                            id,
                            result: serde_json::to_value(self.service.code_action(params))?,
                        }));
                    }
                    Err(CastError::MethodMismatch(p)) => params = p,
                    Err(CastError::JsonError(..)) => return Ok(None),
                }
                match try_cast_request::<Completion>(&method, params) {
                    Ok(params) => {
                        return Ok(Some(Message::OkResponse {
                            id,
                            result: serde_json::to_value(self.service.completion(params))?,
                        }));
                    }
                    Err(CastError::MethodMismatch(p)) => params = p,
                    Err(CastError::JsonError(..)) => return Ok(None),
                }
                match try_cast_request::<DocumentDiagnosticRequest>(&method, params) {
                    Ok(params) => {
                        return Ok(Some(Message::OkResponse {
                            id,
                            result: serde_json::to_value(self.service.pull_diagnostics(params))?,
                        }));
                    }
                    Err(CastError::MethodMismatch(p)) => params = p,
                    Err(CastError::JsonError(..)) => return Ok(None),
                }
                match try_cast_request::<DocumentHighlightRequest>(&method, params) {
                    Ok(params) => {
                        return Ok(Some(Message::OkResponse {
                            id,
                            result: serde_json::to_value(self.service.document_highlight(params))?,
                        }));
                    }
                    Err(CastError::MethodMismatch(p)) => params = p,
                    Err(CastError::JsonError(..)) => return Ok(None),
                }
                match try_cast_request::<FoldingRangeRequest>(&method, params) {
                    Ok(params) => {
                        return Ok(Some(Message::OkResponse {
                            id,
                            result: serde_json::to_value(self.service.folding_range(params))?,
                        }));
                    }
                    Err(CastError::MethodMismatch(p)) => params = p,
                    Err(CastError::JsonError(..)) => return Ok(None),
                }
                match try_cast_request::<Formatting>(&method, params) {
                    Ok(params) => {
                        return Ok(Some(Message::OkResponse {
                            id,
                            result: serde_json::to_value(self.service.formatting(params))?,
                        }));
                    }
                    Err(CastError::MethodMismatch(p)) => params = p,
                    Err(CastError::JsonError(..)) => return Ok(None),
                }
                match try_cast_request::<RangeFormatting>(&method, params) {
                    Ok(params) => {
                        return Ok(Some(Message::OkResponse {
                            id,
                            result: serde_json::to_value(self.service.range_formatting(params))?,
                        }));
                    }
                    Err(CastError::MethodMismatch(p)) => params = p,
                    Err(CastError::JsonError(..)) => return Ok(None),
                }
                match try_cast_request::<GotoDefinition>(&method, params) {
                    Ok(params) => {
                        return Ok(Some(Message::OkResponse {
                            id,
                            result: serde_json::to_value(self.service.goto_definition(params))?,
                        }));
                    }
                    Err(CastError::MethodMismatch(p)) => params = p,
                    Err(CastError::JsonError(..)) => return Ok(None),
                }
                match try_cast_request::<GotoTypeDefinition>(&method, params) {
                    Ok(params) => {
                        return Ok(Some(Message::OkResponse {
                            id,
                            result: serde_json::to_value(
                                self.service.goto_type_definition(params),
                            )?,
                        }));
                    }
                    Err(CastError::MethodMismatch(p)) => params = p,
                    Err(CastError::JsonError(..)) => return Ok(None),
                }
                match try_cast_request::<GotoDeclaration>(&method, params) {
                    Ok(params) => {
                        return Ok(Some(Message::OkResponse {
                            id,
                            result: serde_json::to_value(self.service.goto_declaration(params))?,
                        }));
                    }
                    Err(CastError::MethodMismatch(p)) => params = p,
                    Err(CastError::JsonError(..)) => return Ok(None),
                }
                match try_cast_request::<HoverRequest>(&method, params) {
                    Ok(params) => {
                        return Ok(Some(Message::OkResponse {
                            id,
                            result: serde_json::to_value(self.service.hover(params))?,
                        }));
                    }
                    Err(CastError::MethodMismatch(p)) => params = p,
                    Err(CastError::JsonError(..)) => return Ok(None),
                }
                match try_cast_request::<InlayHintRequest>(&method, params) {
                    Ok(params) => {
                        return Ok(Some(Message::OkResponse {
                            id,
                            result: serde_json::to_value(self.service.inlay_hint(params))?,
                        }));
                    }
                    Err(CastError::MethodMismatch(p)) => params = p,
                    Err(CastError::JsonError(..)) => return Ok(None),
                }
                match try_cast_request::<References>(&method, params) {
                    Ok(params) => {
                        return Ok(Some(Message::OkResponse {
                            id,
                            result: serde_json::to_value(self.service.find_references(params))?,
                        }));
                    }
                    Err(CastError::MethodMismatch(p)) => params = p,
                    Err(CastError::JsonError(..)) => return Ok(None),
                }
                match try_cast_request::<PrepareRenameRequest>(&method, params) {
                    Ok(params) => {
                        return Ok(Some(Message::OkResponse {
                            id,
                            result: serde_json::to_value(self.service.prepare_rename(params))?,
                        }));
                    }
                    Err(CastError::MethodMismatch(p)) => params = p,
                    Err(CastError::JsonError(..)) => return Ok(None),
                }
                match try_cast_request::<Rename>(&method, params) {
                    Ok(params) => {
                        return match self.service.rename(params) {
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
                            result: serde_json::to_value(self.service.selection_range(params))?,
                        }));
                    }
                    Err(CastError::MethodMismatch(p)) => params = p,
                    Err(CastError::JsonError(..)) => return Ok(None),
                }
                match try_cast_request::<SemanticTokensFullRequest>(&method, params) {
                    Ok(params) => {
                        return Ok(Some(Message::OkResponse {
                            id,
                            result: serde_json::to_value(
                                self.service.semantic_tokens_full(params),
                            )?,
                        }));
                    }
                    Err(CastError::MethodMismatch(p)) => params = p,
                    Err(CastError::JsonError(..)) => return Ok(None),
                }
                match try_cast_request::<SemanticTokensRangeRequest>(&method, params) {
                    Ok(params) => {
                        return Ok(Some(Message::OkResponse {
                            id,
                            result: serde_json::to_value(
                                self.service.semantic_tokens_range(params),
                            )?,
                        }));
                    }
                    Err(CastError::MethodMismatch(p)) => params = p,
                    Err(CastError::JsonError(..)) => return Ok(None),
                }
                match try_cast_request::<SignatureHelpRequest>(&method, params) {
                    Ok(params) => {
                        return Ok(Some(Message::OkResponse {
                            id,
                            result: serde_json::to_value(self.service.signature_help(params))?,
                        }));
                    }
                    Err(CastError::MethodMismatch(p)) => params = p,
                    Err(CastError::JsonError(..)) => return Ok(None),
                }
                match try_cast_request::<DocumentSymbolRequest>(&method, params) {
                    Ok(params) => Ok(Some(Message::OkResponse {
                        id,
                        result: serde_json::to_value(self.service.document_symbol(params))?,
                    })),
                    Err(CastError::MethodMismatch(..)) => Ok(None),
                    Err(CastError::JsonError(..)) => Ok(None),
                }
            }
            Message::OkResponse { id, result } => {
                self.handle_response(id, result).await?;
                Ok(None)
            }
            Message::Notification { method, mut params } => {
                match try_cast_notification::<DidOpenTextDocument>(&method, params) {
                    Ok(params) => {
                        self.handle_did_open_text_document(params).await?;
                        return Ok(None);
                    }
                    Err(CastError::MethodMismatch(p)) => params = p,
                    Err(CastError::JsonError(..)) => return Ok(None),
                };
                match try_cast_notification::<DidChangeTextDocument>(&method, params) {
                    Ok(params) => {
                        self.handle_did_change_text_document(params).await?;
                        return Ok(None);
                    }
                    Err(CastError::MethodMismatch(p)) => params = p,
                    Err(CastError::JsonError(..)) => return Ok(None),
                };
                match try_cast_notification::<DidChangeConfiguration>(&method, params) {
                    Ok(params) => {
                        self.handle_did_change_configuration(params).await?;
                    }
                    Err(CastError::MethodMismatch(..)) => {}
                    Err(CastError::JsonError(..)) => {}
                };
                Ok(None)
            }
            _ => Ok(None),
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
                self.stdio
                    .write(self.sent_requests.add(
                        WorkspaceDiagnosticRefresh::METHOD.into(),
                        serde_json::Value::Null,
                        vec![],
                    ))
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
            self.stdio
                .write(self.sent_requests.add(
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
            self.stdio
                .write(
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
        } else if let Ok(config) = serde_json::from_value(params.settings) {
            self.service.set_global_config(config);
        }
        Ok(())
    }

    async fn publish_diagnostics(&mut self, uri: Uri) -> anyhow::Result<()> {
        self.stdio
            .write(Message::Notification {
                method: PublishDiagnostics::METHOD.into(),
                params: serde_json::to_value(self.service.publish_diagnostics(uri))?,
            })
            .await
    }
}
