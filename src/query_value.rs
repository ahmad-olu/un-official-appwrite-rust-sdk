use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[allow(missing_docs)]
#[serde(untagged)]
pub enum QueryValue {
    Null,
    Int64(i64),
    Float64(f64),
    Boolean(bool),
    String(String),
    Array(Vec<QueryValue>),
    JsonObject(serde_json::Value),
}

impl QueryValue {
    pub(crate) fn convert_to_array_if_not_array(self) -> Self {
        match self {
            QueryValue::Array(_) => self, // Already an array, use as is
            other => vec![other].into(),  // Wrap in an array
        }
    }
}

impl<T: Into<QueryValue>> From<Option<T>> for QueryValue {
    fn from(v: Option<T>) -> QueryValue {
        v.map(|v| v.into()).unwrap_or(QueryValue::Null)
    }
}

impl From<i64> for QueryValue {
    fn from(v: i64) -> QueryValue {
        QueryValue::Int64(v)
    }
}

impl From<f64> for QueryValue {
    fn from(v: f64) -> QueryValue {
        QueryValue::Float64(v)
    }
}

impl From<bool> for QueryValue {
    fn from(v: bool) -> QueryValue {
        QueryValue::Boolean(v)
    }
}

impl From<&str> for QueryValue {
    fn from(v: &str) -> QueryValue {
        QueryValue::String(v.into())
    }
}

impl From<String> for QueryValue {
    fn from(v: String) -> QueryValue {
        QueryValue::String(v)
    }
}

impl From<Vec<QueryValue>> for QueryValue {
    fn from(v: Vec<QueryValue>) -> QueryValue {
        QueryValue::Array(v)
    }
}

// impl From<&[QueryValue]> for QueryValue {
//     fn from(v: &[QueryValue]) -> Self {
//         QueryValue::Array(v.to_vec())
//     }
// }

impl From<Vec<String>> for QueryValue {
    fn from(v: Vec<String>) -> Self {
        QueryValue::Array(v.into_iter().map(QueryValue::from).collect())
    }
}

impl From<Vec<&str>> for QueryValue {
    fn from(v: Vec<&str>) -> Self {
        QueryValue::Array(v.into_iter().map(QueryValue::from).collect())
    }
}

impl From<Vec<i64>> for QueryValue {
    fn from(v: Vec<i64>) -> Self {
        QueryValue::Array(v.into_iter().map(QueryValue::from).collect())
    }
}

impl From<Vec<f64>> for QueryValue {
    fn from(v: Vec<f64>) -> Self {
        QueryValue::Array(v.into_iter().map(QueryValue::from).collect())
    }
}

impl From<Vec<bool>> for QueryValue {
    fn from(v: Vec<bool>) -> Self {
        QueryValue::Array(v.into_iter().map(QueryValue::from).collect())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Query {
    method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<Value>, // Use serde_json::Value for dynamic values
}

/// * attribute => QueryValue => string?
/// * value => QueryValue => any
impl Query {
    pub fn new(method: &str, attribute: QueryValue, values: QueryValue) -> Self {
        let att = match attribute {
            QueryValue::Null => None,
            QueryValue::String(val) => Some(val),
            _ => None,
        };
        let val: Option<Value> = match values {
            QueryValue::Null => None,
            QueryValue::Int64(v) => Some(json!(v)),
            QueryValue::Float64(v) => Some(json!(v)),
            QueryValue::Boolean(v) => Some(json!(v)),
            QueryValue::String(v) => Some(json!(v)),
            QueryValue::Array(vec) => Some(json!(vec)),
            QueryValue::JsonObject(value) => Some(value),
        };

        Self {
            method: String::from(method),
            attribute: att,
            values: val,
        }
    }
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
