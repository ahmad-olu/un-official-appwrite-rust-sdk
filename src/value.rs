use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// A value that can be passed as an argument or returned from Convex functions.
/// They correspond to the [supported Convex types]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum Value {
    Null,
    Int64(i64),
    Float64(f64),
    Boolean(bool),
    String(String),
    Bytes(Vec<u8>),
    Array(Vec<Value>),
    Object(BTreeMap<String, Value>),
}

impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(v: Option<T>) -> Value {
        v.map(|v| v.into()).unwrap_or(Value::Null)
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Value {
        Value::Int64(v)
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Value {
        Value::Float64(v)
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Value {
        Value::Boolean(v)
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Value {
        Value::String(v.into())
    }
}

impl From<String> for Value {
    fn from(v: String) -> Value {
        Value::String(v)
    }
}

impl From<Vec<u8>> for Value {
    fn from(v: Vec<u8>) -> Value {
        Value::Bytes(v)
    }
}

impl From<Vec<Value>> for Value {
    fn from(v: Vec<Value>) -> Value {
        Value::Array(v)
    }
}
