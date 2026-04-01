use std::fmt;
use serde::{Deserialize, Serialize, ser::SerializeStruct};
use serde_json::Value;
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(transparent)]
pub struct RequestId(IdRepr);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(untagged)]
enum IdRepr {
    U32(u32),
    String(String),
}

impl From<u32> for RequestId {
    fn from(id: u32) -> RequestId {
        RequestId(IdRepr::U32(id))
    }
}

impl From<String> for RequestId {
    fn from(id: String) -> RequestId {
        RequestId(IdRepr::String(id))
    }
}

impl fmt::Display for RequestId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            IdRepr::U32(it) => fmt::Display::fmt(it, f),
            // Use debug to make it clear that `92` and `"92"` are
            // different, and to reduce WTF factor if the sever uses `" "` as an
            // ID.
            IdRepr::String(it) => fmt::Debug::fmt(it, f),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum Message {
    Request {
        id: RequestId,
        method: String,
        #[serde(default)]
        params: Value,
    },
    OkResponse {
        id: RequestId,
        #[serde(default)]
        result: Value,
    },
    ErrResponse {
        id: RequestId,
        error: ResponseError,
    },
    Notification {
        method: String,
        #[serde(default)]
        params: Value,
    },
}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Message::Request { id, method, params } => {
                let mut state = serializer.serialize_struct("Message", 4)?;
                state.serialize_field("jsonrpc", "2.0")?;
                state.serialize_field("id", id)?;
                state.serialize_field("method", method)?;
                state.serialize_field("params", params)?;
                state.end()
            }
            Message::OkResponse { id, result } => {
                let mut state = serializer.serialize_struct("Message", 3)?;
                state.serialize_field("jsonrpc", "2.0")?;
                state.serialize_field("id", id)?;
                state.serialize_field("result", result)?;
                state.end()
            }
            Message::ErrResponse { id, error } => {
                let mut state = serializer.serialize_struct("Message", 3)?;
                state.serialize_field("jsonrpc", "2.0")?;
                state.serialize_field("id", id)?;
                state.serialize_field("error", error)?;
                state.end()
            }
            Message::Notification { method, params } => {
                let mut state = serializer.serialize_struct("Message", 3)?;
                state.serialize_field("jsonrpc", "2.0")?;
                state.serialize_field("method", method)?;
                state.serialize_field("params", params)?;
                state.end()
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseError {
    pub code: i64,
    pub message: String,
    pub data: Option<Value>,
}

// ↑ `lspt` crate unrelated code
// ---------------------
// ↓ `lspt` crate related code

pub fn try_cast_request<R>(method: &str, params: Value) -> Result<Result<R::Params, serde_json::Error>, Value>
where
    R: lspt::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    if method == R::METHOD {
        Ok(serde_json::from_value(params))
    } else {
        Err(params)
    }
}

pub fn try_cast_notification<N>(method: &str, params: Value) -> Result<Result<N::Params, serde_json::Error>, Value>
where
    N: lspt::notification::Notification,
    N::Params: serde::de::DeserializeOwned,
{
    if method == N::METHOD {
        Ok(serde_json::from_value(params))
    } else {
        Err(params)
    }
}
