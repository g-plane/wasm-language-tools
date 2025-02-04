use serde::{ser::SerializeStruct, Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum Message {
    Request {
        id: u32,
        method: String,
        #[serde(default)]
        params: Value,
    },
    OkResponse {
        id: u32,
        #[serde(default)]
        result: Value,
    },
    ErrResponse {
        id: u32,
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

// ↑ `lsp-types` crate unrelated code
// ---------------------
// ↓ `lsp-types` crate related code

pub fn try_cast_request<R>(method: &str, params: Value) -> Result<R::Params, CastError>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    if method == R::METHOD {
        serde_json::from_value(params).map_err(CastError::JsonError)
    } else {
        Err(CastError::MethodMismatch(params))
    }
}

pub fn try_cast_notification<N>(method: &str, params: Value) -> Result<N::Params, CastError>
where
    N: lsp_types::notification::Notification,
    N::Params: serde::de::DeserializeOwned,
{
    if method == N::METHOD {
        serde_json::from_value(params).map_err(CastError::JsonError)
    } else {
        Err(CastError::MethodMismatch(params))
    }
}

#[derive(Debug)]
pub enum CastError {
    MethodMismatch(Value),
    #[expect(unused)]
    JsonError(serde_json::Error),
}
