use crate::message::Message;
use rustc_hash::FxHashMap;
use serde_json::Value;

#[derive(Debug, Default)]
pub struct SentRequests<T> {
    id: u32,
    data: FxHashMap<u32, T>,
}

impl<T> SentRequests<T> {
    pub fn add(&mut self, method: String, params: Value) -> Message {
        let id = self.id;
        self.id += 1;
        Message::Request { id, method, params }
    }

    pub fn add_with_data(&mut self, method: String, params: Value, data: T) -> Message {
        let id = self.id;
        self.data.insert(id, data);
        self.id += 1;
        Message::Request { id, method, params }
    }

    pub fn remove(&mut self, id: u32) -> Option<T> {
        self.data.remove(&id)
    }
}
