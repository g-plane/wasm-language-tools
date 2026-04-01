use crate::{message::Message, server::Server};
use anyhow::Result;
use rustc_hash::FxHashMap;
use serde_json::Value;
use crate::message::RequestId;

type Callback = Box<dyn FnOnce(&mut Server, Value) -> Result<()> + 'static>;

#[derive(Default)]
pub struct SentRequests {
    id: u32,
    callbacks: FxHashMap<RequestId, Callback>,
}

impl SentRequests {
    pub fn next_id(&mut self) -> RequestId {
        let id = self.id;
        self.id += 1;
        RequestId::from(id)
    }

    pub fn add<F>(&mut self, method: String, params: Value, callback: F) -> Message
    where
        F: FnOnce(&mut Server, Value) -> Result<()> + 'static,
    {
        let id = self.next_id();
        self.callbacks.insert(id.clone(), Box::new(callback));
        Message::Request { id, method, params }
    }

    pub fn remove(&mut self, id: RequestId) -> Option<Callback> {
        self.callbacks.remove(&id)
    }
}
