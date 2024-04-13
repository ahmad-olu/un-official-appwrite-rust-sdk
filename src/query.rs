use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize, Debug)]
struct QueryString {
    method: String,
    attribute: Option<String>,
    value: Option<Value>, // Use serde_json::Value for dynamic values
}
impl QueryString {
    fn new(method: &str, attribute: Option<String>, value: Option<Value>) -> Self {
        Self {
            method: method.to_string(),
            attribute,
            value,
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct QueryDynamic {
    method: String,
    attribute: Option<String>,
    values: Option<Vec<serde_json::Value>>, // Use serde_json::Value for dynamic values
}
impl QueryDynamic {
    fn new(method: &str, attribute: Option<String>, values: Option<Vec<Value>>) -> Self {
        Self {
            method: method.to_string(),
            attribute,
            values,
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

/// Helper class to generate query strings.
#[derive(Serialize, Deserialize, Debug)]
pub struct Query;

impl Query {
    /// Filter resources where [attribute] is equal to [value].
    ///
    /// [value] can be a single value or a list. If a list is used
    /// the query will return resources where [attribute] is equal
    /// to any of the values in the list.
    pub fn equal(attribute: &str, value: Value) -> String {
        QueryDynamic::new("equal", Some(attribute.to_string()), Some(vec![value])).to_string()
    }

    /// Filter resources where [attribute] is not equal to [value].
    pub fn not_equal(attribute: &str, value: Value) -> String {
        QueryDynamic::new("notEqual", Some(attribute.to_string()), Some(vec![value])).to_string()
    }

    /// Filter resources where [attribute] is less than [value].
    pub fn less_than(attribute: &str, value: Value) -> String {
        QueryDynamic::new("lessThan", Some(attribute.to_string()), Some(vec![value])).to_string()
    }

    /// Filter resources where [attribute] is less than or equal to [value].
    pub fn less_than_equal(attribute: &str, value: Value) -> String {
        QueryDynamic::new(
            "lessThanEqual",
            Some(attribute.to_string()),
            Some(vec![value]),
        )
        .to_string()
    }

    /// Filter resources where [attribute] is greater than [value].
    pub fn greater_than(attribute: &str, value: Value) -> String {
        QueryDynamic::new(
            "greaterThan",
            Some(attribute.to_string()),
            Some(vec![value]),
        )
        .to_string()
    }

    /// Filter resources where [attribute] is greater than or equal to [value].
    pub fn greater_than_equal(attribute: &str, value: Value) -> String {
        QueryDynamic::new(
            "greaterThanEqual",
            Some(attribute.to_string()),
            Some(vec![value]),
        )
        .to_string()
    }

    /// Filter resources where by searching [attribute] for [value].
    pub fn search(attribute: &str, value: &str) -> String {
        QueryString::new("search", Some(attribute.to_string()), Some(json!(value))).to_string()
    }

    /// Filter resources where [attribute] is null.
    pub fn is_null(attribute: &str) -> String {
        QueryString::new("isNull", Some(attribute.to_string()), None).to_string()
    }

    /// Filter resources where [attribute] is not null.
    pub fn is_not_null(attribute: &str) -> String {
        QueryString::new("isNotNull", Some(attribute.to_string()), None).to_string()
    }

    /// Filter resources where [attribute] is between [start] and [end] (inclusive).
    pub fn between(attribute: &str, start: Value, end: Value) -> String {
        QueryDynamic::new(
            "between",
            Some(attribute.to_string()),
            Some(vec![start, end]),
        )
        .to_string()
    }

    /// Filter resources where [attribute] starts with [value].
    pub fn starts_with(
        attribute: &str,
        value: &str, // &str TODO
    ) -> String {
        QueryString::new(
            "startsWith",
            Some(attribute.to_string()),
            Some(json!(value)),
        )
        .to_string()
    }

    /// Filter resources where [attribute] ends with [value].
    pub fn ends_with(
        attribute: &str,
        value: &str, // &str TODO
    ) -> String {
        QueryString::new("endsWith", Some(attribute.to_string()), Some(json!(value))).to_string()
    }

    /// Filter resources where [attribute] contains [value]
    /// [value] can be a single value or a list.
    pub fn contains(attribute: &str, value: Value) -> String {
        QueryDynamic::new("contains", Some(attribute.to_string()), Some(vec![value])).to_string()
    }

    pub fn or(queries: Vec<String>) -> String {
        let parsed_queries: Vec<serde_json::Value> = queries
            .iter()
            .map(|q| serde_json::from_str(q).unwrap())
            .collect();
        QueryDynamic::new("or", None, Some(parsed_queries)).to_string()
    }

    pub fn and_queries(queries: Vec<String>) -> String {
        let parsed_queries: Vec<serde_json::Value> = queries
            .iter()
            .map(|q| serde_json::from_str(q).unwrap())
            .collect();
        QueryDynamic::new("and", None, Some(parsed_queries)).to_string()
    }

    /// Specify which attributes should be returned by the API call.
    pub fn select(attributes: Vec<&str>) -> String {
        QueryDynamic::new(
            "select",
            None,
            Some(
                attributes
                    .iter()
                    .map(|attr| serde_json::Value::String(attr.to_string()))
                    .collect(),
            ),
        )
        .to_string()
    }

    /// Sort results by [attribute] ascending.
    pub fn order_asc(attribute: &str) -> String {
        QueryDynamic::new("orderAsc", Some(attribute.to_string()), None).to_string()
    }

    /// Sort results by [attribute] descending.
    pub fn order_desc(attribute: &str) -> String {
        QueryDynamic::new("orderDesc", Some(attribute.to_string()), None).to_string()
    }

    /// Return results before [id].
    ///
    /// Refer to the [Cursor Based Pagination](https://appwrite.io/docs/pagination#cursor-pagination)
    /// docs for more information.
    pub fn cursor_before(id: &str) -> String {
        QueryString::new("cursorBefore", None, Some(json!(id))).to_string()
    }

    /// Return results after [id].
    ///
    /// Refer to the [Cursor Based Pagination](https://appwrite.io/docs/pagination#cursor-pagination)
    /// docs for more information.
    pub fn cursor_after(id: &str) -> String {
        QueryString::new("cursorAfter", None, Some(json!(id))).to_string()
    }

    /// Return only [limit] results.
    pub fn limit(limit: u32) -> String {
        QueryString::new("limit", None, Some(json!(limit))).to_string()
    }

    /// Return results from [offset].
    ///
    /// Refer to the [Offset Pagination](https://appwrite.io/docs/pagination#offset-pagination)
    /// docs for more information.
    pub fn offset(offset: u32) -> String {
        QueryString::new("offset", None, Some(json!(offset))).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal_query() {
        let query = Query::equal("name", serde_json::Value::String("John".to_string()));
        assert_eq!(
            query,
            r#"{"method":"equal","attribute":"name","values":["John"]}"#
        );
    }

    // #[test]
    // fn test_select_query() {
    //     let query = Query::select(vec!["name", "age"]);
    //     assert_eq!(
    //         query.to_string(),
    //         r#"{"method":"select","values":["name","age"]}"#
    //     );
    // }

    // #[test]
    // fn test_or_query() {
    //     let queries = vec![
    //         r#"{"method":"equal","attribute":"name","values":["John"]}"#.to_string(),
    //         r#"{"method":"equal","attribute":"age","values":[30]}"#.to_string(),
    //     ];
    //     let query = Query::or_queries(queries);
    //     assert!(query.to_string().starts_with(r#"{"method":"or","values":[{"method":"equal"..."#));
    // }
}
