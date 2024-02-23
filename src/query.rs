use std::{u32, u64};

/// Helper struct to generate query strings.
struct Query;

impl Query {
    /// Filter resources where [attribute] is equal to [value].
    ///
    /// [value] can be a single value or a list. If a list is used
    /// the query will return resources where [attribute] is equal
    /// to any of the values in the list.
    fn equal(&mut self, attribute: &str, value: &serde_json::Value) -> String {
        self._add_query(attribute, "equal", value)
    }

    /// Filter resources where [attribute] is not equal to [value].
    ///
    /// [value] can be a single value or a list. If a list is used
    /// the query will return resources where [attribute] is equal
    /// to any of the values in the list.
    fn not_equal(&mut self, attribute: &str, value: &serde_json::Value) -> String {
        self._add_query(attribute, "notEqual", value)
    }

    /// Filter resources where [attribute] is less than [value].
    fn less_than(&mut self, attribute: &str, value: &serde_json::Value) -> String {
        self._add_query(attribute, "lessThan", value)
    }

    /// Filter resources where [attribute] is less than or equal to [value].
    fn less_than_equal(&mut self, attribute: &str, value: &serde_json::Value) -> String {
        self._add_query(attribute, "lessThanEqual", value)
    }

    /// Filter resources where [attribute] is greater than [value].
    fn greater_than(&mut self, attribute: &str, value: &serde_json::Value) -> String {
        self._add_query(attribute, "greaterThan", value)
    }

    /// Filter resources where [attribute] is greater than or equal to [value].
    fn greater_than_equal(&mut self, attribute: &str, value: &serde_json::Value) -> String {
        self._add_query(attribute, "greaterThanEqual", value)
    }

    /// Filter resources where by searching [attribute] for [value].
    fn search(&mut self, attribute: &str, value: &serde_json::Value) -> String {
        self._add_query(attribute, "search", value)
    }

    /// Filter resources where [attribute] is null.
    fn is_null(&mut self, attribute: &str) -> String {
        format!("isNull(\"{}\")", attribute)
    }

    /// Filter resources where [attribute] is not null.
    fn is_not_null(&mut self, attribute: &str) -> String {
        format!("isNotNull(\"{}\")", attribute)
    }

    /// Filter resources where [attribute] is between [start] and [end] (inclusive).
    fn between(
        &mut self,
        attribute: &str,
        start: &serde_json::Value,
        end: &serde_json::Value,
    ) -> String {
        format!(
            "between(\"{}\", {}, {})",
            attribute,
            self._parse_values(start),
            self._parse_values(end),
        )
    }

    /// Filter resources where [attribute] starts with [value].
    fn starts_with(&mut self, attribute: &str, value: &serde_json::Value) -> String {
        self._add_query(attribute, "startsWith", value)
    }

    /// Filter resources where [attribute] ends with [value].
    fn ends_with(&mut self, attribute: &str, value: &serde_json::Value) -> String {
        self._add_query(attribute, "endsWith", value)
    }

    /// Specify which attributes should be returned by the API call.
    fn select(&mut self, attributes: Vec<String>) -> String {
        format!(
            "select([{}])",
            attributes
                .iter()
                .map(|attr| format!("\"{}\"", attr))
                .collect::<Vec<String>>()
                .join(",")
        )
    }

    /// Sort results by [attribute] ascending.
    fn order_asc(&mut self, attributes: String) -> String {
        format!("orderAsc(\"{}\")", attributes)
    }

    /// Sort results by [attribute] descending.
    fn order_desc(&mut self, attributes: String) -> String {
        format!("orderDesc(\"{}\")", attributes)
    }

    /// Return results before [id].
    ///
    /// Refer to the [Cursor Based Pagination](https://appwrite.io/docs/pagination#cursor-pagination)
    /// docs for more information.
    fn cursor_before(&mut self, id: String) -> String {
        format!("cursorBefore(\"{}\")", id)
    }

    /// Return results after [id].
    ///
    /// Refer to the [Cursor Based Pagination](https://appwrite.io/docs/pagination#cursor-pagination)
    /// docs for more information.
    fn cursor_after(&mut self, id: String) -> String {
        format!("cursorAfter(\"{}\")", id)
    }

    /// Return only [limit] results.
    fn limit(&mut self, limit: u32) -> String {
        format!("limit({})", limit)
    }

    /// Return results from [offset].
    ///
    /// Refer to the [Offset Pagination](https://appwrite.io/docs/pagination#offset-pagination)
    /// docs for more information.
    fn offset(&mut self, offset: u32) -> String {
        format!("offset({})", offset)
    }

    fn _add_query(&mut self, attribute: &str, method: &str, value: &serde_json::Value) -> String {
        if let serde_json::Value::Array(array) = value {
            let formatted_values: Vec<String> =
                array.iter().map(|item| self._parse_values(item)).collect();
            return format!(
                "{}(\"{}\", [{}])",
                method,
                attribute,
                formatted_values.join(",")
            );
        } else {
            return format!(
                "{}(\"{}\", [{}])",
                method,
                attribute,
                self._parse_values(value)
            );
        }
    }

    fn _parse_values(&self, value: &serde_json::Value) -> String {
        match value {
            serde_json::Value::String(s) => format!("\"{}\"", s),
            _ => format!("{}", value),
        }
    }
}
