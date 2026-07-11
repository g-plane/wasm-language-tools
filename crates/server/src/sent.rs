use crate::{message::Message, server::Server};
use anyhow::Result;
use lspt::NumberOrString;
use rustc_hash::FxHashMap;
use serde_json::Value;

type Callback = Box<dyn FnOnce(&mut Server, Value) -> Result<()> + 'static>;

#[derive(Default)]
pub struct SentRequests {
    id: i32,
    callbacks: FxHashMap<i32, Callback>,
}

impl SentRequests {
    pub fn next_id(&mut self) -> i32 {
        let id = self.id;
        self.id += 1;
        id
    }

    pub fn add<F>(&mut self, method: String, params: Value, callback: F) -> Message
    where
        F: FnOnce(&mut Server, Value) -> Result<()> + 'static,
    {
        let id = self.next_id();
        self.callbacks.insert(id, Box::new(callback));
        Message::Request {
            id: NumberOrString::Integer(id),
            method,
            params,
        }
    }

    pub fn remove(&mut self, id: i32) -> Option<Callback> {
        self.callbacks.remove(&id)
    }
}
