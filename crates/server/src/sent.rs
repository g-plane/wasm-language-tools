use crate::message::Message;
use rustc_hash::FxHashMap;

#[derive(Debug, Default)]
pub struct SentRequests<T> {
    id: u32,
    data: FxHashMap<u32, T>,
}

impl<T> SentRequests<T> {
    pub fn add(&mut self, method: String, params: serde_json::Value, data: T) -> Message {
        let id = self.id;
        self.data.insert(id, data);
        self.id += 1;
        Message::Request { id, method, params }
    }

    pub fn remove(&mut self, id: u32) -> Option<T> {
        self.data.remove(&id)
    }
}
