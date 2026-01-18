use crate::{
    message::{Message, ResponseError, try_cast_notification, try_cast_request},
    sent::SentRequests,
    stdio,
};
use lspt::{
    ConfigurationItem, ConfigurationParams, DidChangeConfigurationParams,
    DidChangeTextDocumentParams, DidOpenTextDocumentParams, InitializeParams, Registration,
    RegistrationParams,
    notification::{
        DidChangeConfigurationNotification, DidChangeTextDocumentNotification,
        DidCloseTextDocumentNotification, DidOpenTextDocumentNotification, ExitNotification,
        InitializedNotification, Notification as _, PublishDiagnosticsNotification,
    },
    request::{
        CallHierarchyIncomingCallsRequest, CallHierarchyOutgoingCallsRequest,
        CallHierarchyPrepareRequest, CodeActionRequest, CodeLensRequest, CodeLensResolveRequest,
        CompletionRequest, ConfigurationRequest, DeclarationRequest, DefinitionRequest,
        DiagnosticRefreshRequest, DocumentDiagnosticRequest, DocumentFormattingRequest,
        DocumentHighlightRequest, DocumentRangeFormattingRequest, DocumentSymbolRequest,
        FoldingRangeRequest, HoverRequest, InlayHintRefreshRequest, InlayHintRequest,
        PrepareRenameRequest, ReferencesRequest, RegistrationRequest, RenameRequest, Request as _,
        SelectionRangeRequest, SemanticTokensRangeRequest, SemanticTokensRequest, ShutdownRequest,
        SignatureHelpRequest, TypeDefinitionRequest, TypeHierarchyPrepareRequest,
        TypeHierarchySubtypesRequest, TypeHierarchySupertypesRequest,
    },
};
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::io::StdinLock;
use wat_service::LanguageService;

pub struct Server {
    service: LanguageService,
    support_pull_diagnostics: bool,
    support_refresh_diagnostics: bool,
    support_refresh_inlay_hint: bool,
    support_pull_config: bool,
    support_register_change_config: bool,
    sent_requests: SentRequests,
    pool: ThreadPool,
}

impl Server {
    pub fn new() -> Self {
        Self {
            service: LanguageService::default(),
            support_pull_diagnostics: false,
            support_refresh_diagnostics: false,
            support_refresh_inlay_hint: false,
            support_pull_config: false,
            support_register_change_config: false,
            sent_requests: SentRequests::default(),
            pool: ThreadPoolBuilder::new().build().unwrap(),
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        let mut stdin = std::io::stdin().lock();
        self.initialize(&mut stdin)?;

        loop {
            let message = match stdio::read(&mut stdin) {
                Ok(Some(message)) => message,
                Ok(None) => return Ok(()),
                _ => continue,
            };
            match message {
                Message::Request { id, method, params } => {
                    let service = self.service.clone();
                    self.pool.spawn(move || {
                        let _ = stdio::write(Self::handle_request(service, id, method, params));
                    });
                }
                Message::OkResponse { id, result } => {
                    self.handle_response(id, result)?;
                }
                Message::Notification { method, mut params } => {
                    match try_cast_notification::<DidOpenTextDocumentNotification>(&method, params)
                    {
                        Ok(Ok(params)) => {
                            self.handle_did_open_text_document(params)?;
                            continue;
                        }
                        Ok(Err(..)) => continue,
                        Err(p) => params = p,
                    }
                    match try_cast_notification::<DidChangeTextDocumentNotification>(
                        &method, params,
                    ) {
                        Ok(Ok(params)) => {
                            self.handle_did_change_text_document(params)?;
                            continue;
                        }
                        Ok(Err(..)) => continue,
                        Err(p) => params = p,
                    }
                    match try_cast_notification::<DidCloseTextDocumentNotification>(&method, params)
                    {
                        Ok(Ok(params)) => {
                            self.service.did_close(params);
                            continue;
                        }
                        Ok(Err(..)) => continue,
                        Err(p) => params = p,
                    }
                    match try_cast_notification::<DidChangeConfigurationNotification>(
                        &method, params,
                    ) {
                        Ok(Ok(params)) => {
                            self.handle_did_change_configuration(params)?;
                            continue;
                        }
                        Ok(Err(..)) => continue,
                        Err(p) => params = p,
                    }
                    match try_cast_notification::<InitializedNotification>(&method, params) {
                        Ok(..) => {
                            if self.support_register_change_config {
                                stdio::write(Message::Request {
                                    id: self.sent_requests.next_id(),
                                    method: RegistrationRequest::METHOD.into(),
                                    params: serde_json::to_value(RegistrationParams {
                                        registrations: vec![Registration {
                                            id: DidChangeConfigurationNotification::METHOD.into(),
                                            method: DidChangeConfigurationNotification::METHOD
                                                .into(),
                                            register_options: None,
                                        }],
                                    })?,
                                })?;
                            }
                            continue;
                        }
                        Err(p) => params = p,
                    }
                    if try_cast_notification::<ExitNotification>(&method, params).is_ok() {
                        return Ok(());
                    }
                }
                _ => {}
            }
        }
    }

    fn initialize(&mut self, stdin: &mut StdinLock) -> anyhow::Result<()> {
        let (id, params) = match stdio::read(stdin) {
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
        self.support_refresh_diagnostics = matches!(
            params
                .capabilities
                .workspace
                .as_ref()
                .and_then(|it| it.diagnostics.as_ref())
                .and_then(|it| it.refresh_support),
            Some(true)
        );
        self.support_refresh_inlay_hint = matches!(
            params
                .capabilities
                .workspace
                .as_ref()
                .and_then(|it| it.inlay_hint.as_ref())
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
        self.support_register_change_config = matches!(
            params
                .capabilities
                .workspace
                .as_ref()
                .and_then(|it| it.did_change_configuration.as_ref())
                .and_then(|it| it.dynamic_registration),
            Some(true)
        );
        stdio::write(Message::OkResponse {
            id,
            result: serde_json::to_value(self.service.initialize(params))?,
        })?;
        Ok(())
    }

    fn handle_request(
        service: LanguageService,
        id: u32,
        method: String,
        params: serde_json::Value,
    ) -> Message {
        try_cast_request::<CallHierarchyPrepareRequest>(&method, params)
            .map(|params| {
                params
                    .and_then(|params| serde_json::to_value(service.prepare_call_hierarchy(params)))
                    .map(|result| Message::OkResponse { id, result })
            })
            .or_else(|params| {
                try_cast_request::<CallHierarchyIncomingCallsRequest>(&method, params).map(
                    |params| {
                        params
                            .and_then(|params| {
                                serde_json::to_value(service.call_hierarchy_incoming_calls(params))
                            })
                            .map(|result| Message::OkResponse { id, result })
                    },
                )
            })
            .or_else(|params| {
                try_cast_request::<CallHierarchyOutgoingCallsRequest>(&method, params).map(
                    |params| {
                        params
                            .and_then(|params| {
                                serde_json::to_value(service.call_hierarchy_outgoing_calls(params))
                            })
                            .map(|result| Message::OkResponse { id, result })
                    },
                )
            })
            .or_else(|params| {
                try_cast_request::<CodeActionRequest>(&method, params).map(|params| {
                    params
                        .and_then(|params| serde_json::to_value(service.code_action(params)))
                        .map(|result| Message::OkResponse { id, result })
                })
            })
            .or_else(|params| {
                try_cast_request::<CodeLensRequest>(&method, params).map(|params| {
                    params
                        .and_then(|params| serde_json::to_value(service.code_lens(params)))
                        .map(|result| Message::OkResponse { id, result })
                })
            })
            .or_else(|params| {
                try_cast_request::<CodeLensResolveRequest>(&method, params).map(|params| {
                    params
                        .and_then(|params| serde_json::to_value(service.code_lens_resolve(params)))
                        .map(|result| Message::OkResponse { id, result })
                })
            })
            .or_else(|params| {
                try_cast_request::<CompletionRequest>(&method, params).map(|params| {
                    params
                        .and_then(|params| serde_json::to_value(service.completion(params)))
                        .map(|result| Message::OkResponse { id, result })
                })
            })
            .or_else(|params| {
                try_cast_request::<DocumentDiagnosticRequest>(&method, params).map(|params| {
                    params
                        .and_then(|params| serde_json::to_value(service.pull_diagnostics(params)))
                        .map(|result| Message::OkResponse { id, result })
                })
            })
            .or_else(|params| {
                try_cast_request::<DocumentHighlightRequest>(&method, params).map(|params| {
                    params
                        .and_then(|params| serde_json::to_value(service.document_highlight(params)))
                        .map(|result| Message::OkResponse { id, result })
                })
            })
            .or_else(|params| {
                try_cast_request::<FoldingRangeRequest>(&method, params).map(|params| {
                    params
                        .and_then(|params| serde_json::to_value(service.folding_range(params)))
                        .map(|result| Message::OkResponse { id, result })
                })
            })
            .or_else(|params| {
                try_cast_request::<DocumentFormattingRequest>(&method, params).map(|params| {
                    params
                        .and_then(|params| serde_json::to_value(service.formatting(params)))
                        .map(|result| Message::OkResponse { id, result })
                })
            })
            .or_else(|params| {
                try_cast_request::<DocumentRangeFormattingRequest>(&method, params).map(|params| {
                    params
                        .and_then(|params| serde_json::to_value(service.range_formatting(params)))
                        .map(|result| Message::OkResponse { id, result })
                })
            })
            .or_else(|params| {
                try_cast_request::<DefinitionRequest>(&method, params).map(|params| {
                    params
                        .and_then(|params| serde_json::to_value(service.goto_definition(params)))
                        .map(|result| Message::OkResponse { id, result })
                })
            })
            .or_else(|params| {
                try_cast_request::<TypeDefinitionRequest>(&method, params).map(|params| {
                    params
                        .and_then(|params| {
                            serde_json::to_value(service.goto_type_definition(params))
                        })
                        .map(|result| Message::OkResponse { id, result })
                })
            })
            .or_else(|params| {
                try_cast_request::<DeclarationRequest>(&method, params).map(|params| {
                    params
                        .and_then(|params| serde_json::to_value(service.goto_declaration(params)))
                        .map(|result| Message::OkResponse { id, result })
                })
            })
            .or_else(|params| {
                try_cast_request::<HoverRequest>(&method, params).map(|params| {
                    params
                        .and_then(|params| serde_json::to_value(service.hover(params)))
                        .map(|result| Message::OkResponse { id, result })
                })
            })
            .or_else(|params| {
                try_cast_request::<InlayHintRequest>(&method, params).map(|params| {
                    params
                        .and_then(|params| serde_json::to_value(service.inlay_hint(params)))
                        .map(|result| Message::OkResponse { id, result })
                })
            })
            .or_else(|params| {
                try_cast_request::<ReferencesRequest>(&method, params).map(|params| {
                    params
                        .and_then(|params| serde_json::to_value(service.find_references(params)))
                        .map(|result| Message::OkResponse { id, result })
                })
            })
            .or_else(|params| {
                try_cast_request::<PrepareRenameRequest>(&method, params).map(|params| {
                    params
                        .and_then(|params| serde_json::to_value(service.prepare_rename(params)))
                        .map(|result| Message::OkResponse { id, result })
                })
            })
            .or_else(|params| {
                try_cast_request::<RenameRequest>(&method, params).map(|params| {
                    params.and_then(|params| match service.rename(params) {
                        Ok(result) => serde_json::to_value(result)
                            .map(|result| Message::OkResponse { id, result }),
                        Err(message) => Ok(Message::ErrResponse {
                            id,
                            error: ResponseError {
                                code: -1,
                                message,
                                data: None,
                            },
                        }),
                    })
                })
            })
            .or_else(|params| {
                try_cast_request::<SelectionRangeRequest>(&method, params).map(|params| {
                    params
                        .and_then(|params| serde_json::to_value(service.selection_range(params)))
                        .map(|result| Message::OkResponse { id, result })
                })
            })
            .or_else(|params| {
                try_cast_request::<SemanticTokensRequest>(&method, params).map(|params| {
                    params
                        .and_then(|params| {
                            serde_json::to_value(service.semantic_tokens_full(params))
                        })
                        .map(|result| Message::OkResponse { id, result })
                })
            })
            .or_else(|params| {
                try_cast_request::<SemanticTokensRangeRequest>(&method, params).map(|params| {
                    params
                        .and_then(|params| {
                            serde_json::to_value(service.semantic_tokens_range(params))
                        })
                        .map(|result| Message::OkResponse { id, result })
                })
            })
            .or_else(|params| {
                try_cast_request::<SignatureHelpRequest>(&method, params).map(|params| {
                    params
                        .and_then(|params| serde_json::to_value(service.signature_help(params)))
                        .map(|result| Message::OkResponse { id, result })
                })
            })
            .or_else(|params| {
                try_cast_request::<DocumentSymbolRequest>(&method, params).map(|params| {
                    params
                        .and_then(|params| serde_json::to_value(service.document_symbol(params)))
                        .map(|result| Message::OkResponse { id, result })
                })
            })
            .or_else(|params| {
                try_cast_request::<TypeHierarchyPrepareRequest>(&method, params).map(|params| {
                    params
                        .and_then(|params| {
                            serde_json::to_value(service.prepare_type_hierarchy(params))
                        })
                        .map(|result| Message::OkResponse { id, result })
                })
            })
            .or_else(|params| {
                try_cast_request::<TypeHierarchySupertypesRequest>(&method, params).map(|params| {
                    params
                        .and_then(|params| {
                            serde_json::to_value(service.type_hierarchy_supertypes(params))
                        })
                        .map(|result| Message::OkResponse { id, result })
                })
            })
            .or_else(|params| {
                try_cast_request::<TypeHierarchySubtypesRequest>(&method, params).map(|params| {
                    params
                        .and_then(|params| {
                            serde_json::to_value(service.type_hierarchy_subtypes(params))
                        })
                        .map(|result| Message::OkResponse { id, result })
                })
            })
            .or_else(|params| {
                try_cast_request::<ShutdownRequest>(&method, params).map(|_| {
                    Ok(Message::OkResponse {
                        id,
                        result: serde_json::Value::Null,
                    })
                })
            })
            .unwrap_or_else(|params| {
                Ok(Message::ErrResponse {
                    id,
                    error: ResponseError {
                        code: -32601,
                        message: "method not found".into(),
                        data: Some(params),
                    },
                })
            })
            .unwrap_or_else(|error| Message::ErrResponse {
                id,
                error: ResponseError {
                    code: -32603,
                    message: error.to_string(),
                    data: None,
                },
            })
    }

    fn handle_response(&mut self, id: u32, result: serde_json::Value) -> anyhow::Result<()> {
        if let Some(callback) = self.sent_requests.remove(id) {
            callback(self, result)
        } else {
            Ok(())
        }
    }

    fn handle_did_open_text_document(
        &mut self,
        params: DidOpenTextDocumentParams,
    ) -> anyhow::Result<()> {
        let uri = params.text_document.uri.clone();
        self.service.did_open(params);
        if !self.support_pull_diagnostics {
            self.publish_diagnostics(uri.clone())?;
        }
        if self.support_pull_config {
            stdio::write(self.sent_requests.add(
                ConfigurationRequest::METHOD.into(),
                serde_json::to_value(ConfigurationParams {
                    items: vec![ConfigurationItem {
                        scope_uri: Some(uri.clone()),
                        section: Some("wasmLanguageTools".to_string()),
                    }],
                })?,
                move |server, result| server.update_configs(vec![uri], result),
            ))?;
        }
        Ok(())
    }

    fn handle_did_change_text_document(
        &mut self,
        params: DidChangeTextDocumentParams,
    ) -> anyhow::Result<()> {
        let uri = params.text_document.uri.clone();
        self.service.did_change(params);
        if !self.support_pull_diagnostics {
            self.publish_diagnostics(uri)?;
        }
        Ok(())
    }

    fn handle_did_change_configuration(
        &mut self,
        params: DidChangeConfigurationParams,
    ) -> anyhow::Result<()> {
        if self.support_pull_config {
            let uris = self.service.get_opened_uris();
            stdio::write(
                self.sent_requests.add(
                    ConfigurationRequest::METHOD.into(),
                    serde_json::to_value(ConfigurationParams {
                        items: uris
                            .iter()
                            .map(|uri| ConfigurationItem {
                                scope_uri: Some(uri.clone()),
                                section: Some("wasmLanguageTools".to_string()),
                            })
                            .collect(),
                    })?,
                    move |server, result| server.update_configs(uris, result),
                ),
            )?;
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

    fn publish_diagnostics(&mut self, uri: String) -> anyhow::Result<()> {
        stdio::write(Message::Notification {
            method: PublishDiagnosticsNotification::METHOD.into(),
            params: serde_json::to_value(self.service.publish_diagnostics(uri))?,
        })
    }

    fn update_configs(
        &mut self,
        uris: Vec<String>,
        result: serde_json::Value,
    ) -> anyhow::Result<()> {
        uris.iter()
            .zip(serde_json::from_value::<Vec<_>>(result)?)
            .for_each(|(uri, config)| self.service.set_config(uri, config));
        if self.support_refresh_diagnostics {
            stdio::write(Message::Request {
                id: self.sent_requests.next_id(),
                method: DiagnosticRefreshRequest::METHOD.into(),
                params: serde_json::Value::Null,
            })?;
        } else {
            for uri in uris {
                self.publish_diagnostics(uri)?;
            }
        }
        if self.support_refresh_inlay_hint {
            stdio::write(Message::Request {
                id: self.sent_requests.next_id(),
                method: InlayHintRefreshRequest::METHOD.into(),
                params: serde_json::Value::Null,
            })?;
        }
        Ok(())
    }
}
