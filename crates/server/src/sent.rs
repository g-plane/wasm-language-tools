use crate::message::Message;
use rustc_hash::FxHashMap;
use serde_json::Value;

#[derive(Debug, Default)]
pub struct SentRequests<T> {
    id: u32,
    data: FxHashMap<u32, T>,
}

impl<T> SentRequests<T> {
    pub fn next_id(&mut self) -> u32 {
        let id = self.id;
        self.id += 1;
        id
    }

    pub fn add(&mut self, method: String, params: Value, data: T) -> Message {
        let id = self.next_id();
        self.data.insert(id, data);
        Message::Request { id, method, params }
    }

    pub fn remove(&mut self, id: u32) -> Option<T> {
        self.data.remove(&id)
    }
}
