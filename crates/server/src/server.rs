use crate::files::Files;
use lsp_server::{Connection, ExtractError, Message, Notification, Request, RequestId};
use lsp_types::{
    notification::{DidCloseTextDocument, DidOpenTextDocument},
    DidCloseTextDocumentParams, DidOpenTextDocumentParams, ServerCapabilities,
    TextDocumentSyncCapability, TextDocumentSyncKind,
};

#[derive(Default)]
pub struct Server {
    files: Files,
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

    fn server_loop(
        &mut self,
        connection: Connection,
        _params: serde_json::Value,
    ) -> anyhow::Result<()> {
        for msg in &connection.receiver {
            match msg {
                Message::Request(mut req) => {
                    if connection.handle_shutdown(&req)? {
                        return Ok(());
                    }
                }
                Message::Notification(mut notification) => {
                    match cast_notification::<DidOpenTextDocument>(notification) {
                        Ok(params) => {
                            self.handle_did_open_text_document(params);
                            continue;
                        }
                        Err(ExtractError::MethodMismatch(n)) => notification = n,
                        Err(ExtractError::JsonError { .. }) => continue,
                    };
                    match cast_notification::<DidCloseTextDocument>(notification) {
                        Ok(params) => {
                            self.handle_did_close_text_document(params);
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

    fn handle_did_open_text_document(&mut self, params: DidOpenTextDocumentParams) {
        self.files
            .write(params.text_document.uri, params.text_document.text);
    }

    fn handle_did_close_text_document(&mut self, params: DidCloseTextDocumentParams) {
        self.files.remove(&params.text_document.uri);
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
